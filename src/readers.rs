//! Readers.

use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Result, Seek, SeekFrom};

impl crate::FileFormat {
    /// Determines file format from a reader according to the specified format.
    pub(crate) fn from_format_reader<R: Read + Seek>(
        format: Self,
        reader: &mut BufReader<R>,
    ) -> Result<Self> {
        Ok(match format {
            #[cfg(feature = "cfb")]
            Self::CompoundFileBinary => Self::from_cfb_reader(reader)?,
            Self::ExtensibleMarkupLanguage => Self::from_xml_reader(reader)?,
            Self::MatroskaVideo => Self::from_mkv_reader(reader)?,
            Self::MsDosExecutable => Self::from_exe_reader(reader)?,
            Self::PortableDocumentFormat => Self::from_pdf_reader(reader)?,
            #[cfg(feature = "zip")]
            Self::Zip => Self::from_zip_reader(reader)?,
            _ => format,
        })
    }

    /// Attempts to parse the reader as a CFB.
    ///
    /// It extracts its root entry's CLSID, then compares it to a set of known values and returns
    /// the corresponding variant. If the CLSID does not match any of the known values, the function
    /// returns the `CompoundFileBinary` variant.
    #[cfg(feature = "cfb")]
    pub(crate) fn from_cfb_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        let file = cfb::CompoundFile::open(reader)?;
        Ok(match file.root_entry().clsid().to_string().as_str() {
            "00020810-0000-0000-c000-000000000046" => Self::MicrosoftExcelSpreadsheet,
            "00020820-0000-0000-c000-000000000046" => Self::MicrosoftExcelSpreadsheet,
            "64818d10-4f9b-11cf-86ea-00aa00b929e8" => Self::MicrosoftPowerpointPresentation,
            "74b78f3a-c8c8-11d1-be11-00c04fb6faf1" => Self::MicrosoftProjectPlan,
            "00021201-0000-0000-00c0-000000000046" => Self::MicrosoftPublisherDocument,
            "000c1084-0000-0000-c000-000000000046" => Self::MicrosoftSoftwareInstaller,
            "00021a14-0000-0000-c000-000000000046" => Self::MicrosoftVisioDrawing,
            "00020906-0000-0000-c000-000000000046" => Self::MicrosoftWordDocument,
            _ => Self::CompoundFileBinary,
        })
    }

    /// Attempts to parse the reader as a MS-DOS Executable.
    ///
    /// It first seeks to the `0x3C` offset within the reader and reads the `e_lfanew` field which
    /// indicates the offset to the beginning of the Portable Executable header.
    ///
    /// It then seeks to this address and reads the `Signature` field. If this is `PE\0\0`, it
    /// indicates that it is a Portable Executable. Otherwise, it returns the `MsDosExecutable`
    /// variant.
    ///
    /// Finally, it seeks to `0x12` offset and reads the `Characteristics` field. If this word has
    /// the `0x2000` bit set (`IMAGE_FILE_DLL`), it returns the `DynamicLinkLibrary` variant.
    /// Otherwise, it returns the `PortableExecutable` variant.
    pub(crate) fn from_exe_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        reader.seek(SeekFrom::Start(0x3C))?;
        let mut e_lfanew = [0; 4];
        reader.read_exact(&mut e_lfanew)?;
        reader.seek(SeekFrom::Start(u32::from_le_bytes(e_lfanew) as u64))?;
        let mut signature = [0; 4];
        reader.read_exact(&mut signature)?;
        if &signature == b"PE\0\0" {
            reader.seek(SeekFrom::Current(0x12))?;
            let mut characteristics = [0; 2];
            reader.read_exact(&mut characteristics)?;
            return Ok(if u16::from_le_bytes(characteristics) & 0x2000 == 0x2000 {
                Self::DynamicLinkLibrary
            } else {
                Self::PortableExecutable
            });
        }
        Ok(Self::MsDosExecutable)
    }

    /// Searches the reader for the "webm" byte sequence. If this sequence is found the function
    /// returns the `Webm` variant. Otherwise, it returns the `MatroskaVideo` variant.
    pub(crate) fn from_mkv_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        let length = reader.seek(SeekFrom::End(0))?;
        reader.rewind()?;
        let mut buffer = vec![0; std::cmp::min(limit!(1024, 4096), length as usize)];
        reader.read_exact(&mut buffer)?;
        Ok(if crate::utils::contains(&buffer, b"webm") {
            Self::Webm
        } else {
            Self::MatroskaVideo
        })
    }

    /// Searches the reader for the "AIPrivateData" byte sequence. If this sequence is found the
    /// function returns the `AdobeIllustratorArtwork` variant. Otherwise, it returns the
    /// `PortableDocumentFormat` variant.
    pub(crate) fn from_pdf_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        let length = reader.seek(SeekFrom::End(0))?;
        reader.rewind()?;
        let mut buffer = vec![0; std::cmp::min(limit!(1_048_576, 4_194_304), length as usize)];
        reader.read_exact(&mut buffer)?;
        Ok(if crate::utils::contains(&buffer, b"AIPrivateData") {
            Self::AdobeIllustratorArtwork
        } else {
            Self::PortableDocumentFormat
        })
    }

    /// Attempts to determine if the reader contains Plain Text by checking the first lines for
    /// control characters. If any control characters (other than whitespace) are found, this
    /// function returns an error. Otherwise, it returns the `PlainText` variant.
    pub(crate) fn from_txt_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        reader
            .take(limit!(1_048_576, 8_388_608))
            .lines()
            .take(limit!(32, 256))
            .try_for_each(|line| {
                line?
                    .chars()
                    .find(|char| char.is_control() && !char.is_whitespace())
                    .map(|_| Err(Error::new(ErrorKind::InvalidData, "Invalid chars")))
                    .unwrap_or(Ok(()))
            })
            .map(|_| Self::PlainText)
    }

    /// Searches the reader for byte sequences that indicate the presence of various XML-based
    /// formats. If none are found, it returns the `ExtensibleMarkupLanguage` variant.
    pub(crate) fn from_xml_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        for line in reader
            .take(limit!(131_072, 262_144))
            .fill_buf()?
            .lines()
            .take(limit!(4, 8))
        {
            let buffer = line?.to_lowercase();
            if buffer.contains("<xsl") {
                return Ok(Self::ExtensibleStylesheetLanguageTransformations);
            } else if buffer.contains("<gml") {
                return Ok(Self::GeographyMarkupLanguage);
            } else if buffer.contains("<kml") {
                return Ok(Self::KeyholeMarkupLanguage);
            } else if buffer.contains("<score-partwise") {
                return Ok(Self::Musicxml);
            } else if buffer.contains("<rss") {
                return Ok(Self::ReallySimpleSyndication);
            } else if buffer.contains("<svg") {
                return Ok(Self::ScalableVectorGraphics);
            } else if buffer.contains("<soap") {
                return Ok(Self::SimpleObjectAccessProtocol);
            }
        }
        Ok(Self::ExtensibleMarkupLanguage)
    }

    /// Attempts to parse the reader as a ZIP.
    ///
    /// It checks for certain file names or contents within the archive that indicate the presence
    /// of specific file formats. If a match is found, the corresponding variant is returned.
    /// Otherwise, it returns the `Zip` variant.
    #[cfg(feature = "zip")]
    pub(crate) fn from_zip_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        let mut archive = zip::ZipArchive::new(reader)?;
        let mut format = Self::Zip;
        for index in 0..std::cmp::min(archive.len(), limit!(1024, 4096)) {
            let file = archive.by_index(index)?;
            match file.name() {
                "AndroidManifest.xml" => return Ok(Self::AndroidPackage),
                "AppManifest.xaml" => return Ok(Self::Xap),
                "AppxManifest.xml" => return Ok(Self::WindowsAppPackage),
                "META-INF/MANIFEST.MF" => format = Self::JavaArchive,
                "META-INF/application.xml" => return Ok(Self::EnterpriseApplicationArchive),
                "META-INF/mozilla.rsa" => return Ok(Self::Xpinstall),
                "WEB-INF/web.xml" => return Ok(Self::WebApplicationArchive),
                "doc.kml" => return Ok(Self::KeyholeMarkupLanguageZipped),
                "extension.vsixmanifest" => return Ok(Self::MicrosoftVisualStudioExtension),
                "mimetype" => match std::io::read_to_string(file)?.trim() {
                    "application/epub+zip" => return Ok(Self::ElectronicPublication),
                    "application/vnd.oasis.opendocument.graphics" => {
                        return Ok(Self::OpenDocumentGraphics)
                    }
                    "application/vnd.oasis.opendocument.presentation" => {
                        return Ok(Self::OpenDocumentPresentation);
                    }
                    "application/vnd.oasis.opendocument.spreadsheet" => {
                        return Ok(Self::OpenDocumentSpreadsheet);
                    }
                    "application/vnd.oasis.opendocument.text" => {
                        return Ok(Self::OpenDocumentText);
                    }
                    "application/vnd.recordare.musicxml" => return Ok(Self::MusicxmlZipped),
                    "image/openraster" => return Ok(Self::Openraster),
                    _ => {}
                },
                _ => {
                    if file.name().starts_with("circuitdiagram/") {
                        return Ok(Self::CircuitDiagramDocument);
                    } else if file.name().starts_with("dwf/") {
                        return Ok(Self::DesignWebFormatXps);
                    } else if file.name().starts_with("word/") {
                        return Ok(Self::OfficeOpenXmlDocument);
                    } else if file.name().starts_with("visio/") {
                        return Ok(Self::OfficeOpenXmlDrawing);
                    } else if file.name().starts_with("ppt/") {
                        return Ok(Self::OfficeOpenXmlPresentation);
                    } else if file.name().starts_with("xl/") {
                        return Ok(Self::OfficeOpenXmlSpreadsheet);
                    } else if file.name().starts_with("3D/") && file.name().ends_with(".model") {
                        return Ok(Self::ThreeDimensionalManufacturingFormat);
                    } else if file.name().starts_with("Payload/") && file.name().contains(".app/") {
                        return Ok(Self::IosAppStorePackage);
                    }
                }
            }
        }
        Ok(format)
    }
}
