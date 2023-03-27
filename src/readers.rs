//! Readers.

use std::io::*;

impl crate::FileFormat {
    /// Determines file format from the specified format reader.
    #[allow(unused_variables)]
    #[inline]
    pub(crate) fn from_format_reader<R: Read + Seek>(
        format: Self,
        reader: &mut BufReader<R>,
    ) -> Result<Self> {
        Ok(match format {
            #[cfg(feature = "reader-cfb")]
            Self::CompoundFileBinary => Self::from_cfb_reader(reader)?,
            #[cfg(feature = "reader-xml")]
            Self::ExtensibleMarkupLanguage => Self::from_xml_reader(reader)?,
            #[cfg(feature = "reader-mkv")]
            Self::MatroskaVideo => Self::from_mkv_reader(reader)?,
            #[cfg(feature = "reader-exe")]
            Self::MsDosExecutable => Self::from_exe_reader(reader)?,
            #[cfg(feature = "reader-pdf")]
            Self::PortableDocumentFormat => Self::from_pdf_reader(reader)?,
            #[cfg(feature = "reader-zip")]
            Self::Zip => Self::from_zip_reader(reader)?,
            _ => format,
        })
    }

    /// Determines file format from a generic reader:
    /// [Plain Text (TXT)](`crate::FileFormat::PlainText`) or
    /// [Arbitrary Binary Data (BIN)](`crate::FileFormat::ArbitraryBinaryData`).
    #[allow(unused_variables)]
    #[inline]
    pub(crate) fn from_generic_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Self {
        #[cfg(feature = "reader-txt")]
        {
            Self::from_txt_reader(reader).unwrap_or_default()
        }
        #[cfg(not(feature = "reader-txt"))]
        {
            Self::default()
        }
    }

    /// Determines file format from a
    /// [Compound File Binary (CFB)](`crate::FileFormat::CompoundFileBinary`) reader.
    ///
    /// It reads the CLSID from the root entry of the file and returns the corresponding variant.
    /// If it is not recognized, it returns
    /// [Compound File Binary (CFB)](`crate::FileFormat::CompoundFileBinary`).
    #[cfg(feature = "reader-cfb")]
    pub(crate) fn from_cfb_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        let file = cfb::CompoundFile::open(reader)?;
        Ok(match file.root_entry().clsid().to_string().as_str() {
            "00020810-0000-0000-c000-000000000046" => Self::MicrosoftExcelSpreadsheet,
            "00020820-0000-0000-c000-000000000046" => Self::MicrosoftExcelSpreadsheet,
            "00044851-0000-0000-c000-000000000046" => Self::MicrosoftPowerpointPresentation,
            "64818d10-4f9b-11cf-86ea-00aa00b929e8" => Self::MicrosoftPowerpointPresentation,
            "ea7bae70-fb3b-11cd-a903-00aa00510ea3" => Self::MicrosoftPowerpointPresentation,
            "74b78f3a-c8c8-11d1-be11-00c04fb6faf1" => Self::MicrosoftProjectPlan,
            "00021201-0000-0000-00c0-000000000046" => Self::MicrosoftPublisherDocument,
            "000c1084-0000-0000-c000-000000000046" => Self::MicrosoftSoftwareInstaller,
            "00021a13-0000-0000-c000-000000000046" => Self::MicrosoftVisioDrawing,
            "00021a14-0000-0000-c000-000000000046" => Self::MicrosoftVisioDrawing,
            "00020900-0000-0000-c000-000000000046" => Self::MicrosoftWordDocument,
            "00020906-0000-0000-c000-000000000046" => Self::MicrosoftWordDocument,
            "1cdd8c7b-81c0-45a0-9fed-04143144cc1e" => Self::ThreeDimensionalStudioMax,
            _ => Self::CompoundFileBinary,
        })
    }

    /// Determines file format from a
    /// [MS-DOS Executable (EXE)](`crate::FileFormat::MsDosExecutable`) reader.
    ///
    /// It first seeks to the `0x3C` offset within the reader and reads the `e_lfanew` field.
    ///
    /// It then seeks to this address and reads the `Signature` field. If this is `PE\0\0`, it seeks
    /// to the `0x12` offset and reads the `Characteristics` field in order to know if it is a
    /// [Dynamic Link Library (DLL)](`crate::FileFormat::DynamicLinkLibrary`) or a
    /// [Portable Executable (PE)](`crate::FileFormat::PortableExecutable`). Otherwise, it returns
    /// [MS-DOS Executable (EXE)](`crate::FileFormat::MsDosExecutable`).
    #[cfg(feature = "reader-exe")]
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

    /// Determines file format from a [Matroska Video (MKV)](`crate::FileFormat::MatroskaVideo`)
    /// reader.
    ///
    /// Searches the reader for the "webm" byte sequence. If this sequence is found it returns
    /// [WebM](`crate::FileFormat::Webm`). Otherwise, it returns
    /// [Matroska Video (MKV)](`crate::FileFormat::MatroskaVideo`).
    #[cfg(feature = "reader-mkv")]
    pub(crate) fn from_mkv_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        const SEARCH_LIMIT: usize = match cfg!(feature = "accuracy-mkv") {
            true => 4096,
            false => 1024,
        };
        let length = reader.seek(SeekFrom::End(0))?;
        reader.rewind()?;
        let mut buffer = vec![0; std::cmp::min(SEARCH_LIMIT, length as usize)];
        reader.read_exact(&mut buffer)?;
        Ok(if contains(&buffer, b"webm") {
            Self::Webm
        } else {
            Self::MatroskaVideo
        })
    }

    /// Determines file format from a
    /// [Portable Document Format (PDF)](`crate::FileFormat::PortableDocumentFormat`) reader.
    ///
    /// Searches the reader for the "AIPrivateData" byte sequence. If this sequence is found, it
    /// returns [Adobe Illustrator Artwork (AI)](`crate::FileFormat::AdobeIllustratorArtwork`).
    /// Otherwise, it returns
    /// [Portable Document Format (PDF)](`crate::FileFormat::PortableDocumentFormat`).
    #[cfg(feature = "reader-pdf")]
    pub(crate) fn from_pdf_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        const SEARCH_LIMIT: usize = match cfg!(feature = "accuracy-pdf") {
            true => 4_194_304,
            false => 1_048_576,
        };
        let length = reader.seek(SeekFrom::End(0))?;
        reader.rewind()?;
        let mut buffer = vec![0; std::cmp::min(SEARCH_LIMIT, length as usize)];
        reader.read_exact(&mut buffer)?;
        Ok(if contains(&buffer, b"AIPrivateData") {
            Self::AdobeIllustratorArtwork
        } else {
            Self::PortableDocumentFormat
        })
    }

    /// Determines file format from a [Plain Text (TXT)](`crate::FileFormat::PlainText`) reader.
    ///
    /// Attempts to determine if the reader contains only UTF-8 encoded text by checking the first
    /// lines for control characters. If any control characters (other than whitespace) are found,
    /// it returns an error. Otherwise, it returns
    /// [Plain Text (TXT)](`crate::FileFormat::PlainText`).
    #[cfg(feature = "reader-txt")]
    pub(crate) fn from_txt_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        const READ_LIMIT: u64 = match cfg!(feature = "accuracy-txt") {
            true => 8_388_608,
            false => 1_048_576,
        };
        const LINE_LIMIT: usize = match cfg!(feature = "accuracy-txt") {
            true => 256,
            false => 32,
        };
        reader.rewind()?;
        reader
            .take(READ_LIMIT)
            .lines()
            .take(LINE_LIMIT)
            .try_for_each(|line| {
                line?
                    .chars()
                    .find(|char| char.is_control() && !char.is_whitespace())
                    .map(|_| Err(Error::new(ErrorKind::InvalidData, "Invalid chars")))
                    .unwrap_or(Ok(()))
            })
            .map(|_| Self::PlainText)
    }

    /// Determines file format from an
    /// [Extensible Markup Language (XML)](`crate::FileFormat::ExtensibleMarkupLanguage`) reader.
    ///
    /// Searches the reader for byte sequences that indicate the presence of various file formats.
    /// If none are found, it returns
    /// [Extensible Markup Language (XML)](`crate::FileFormat::ExtensibleMarkupLanguage`).
    #[cfg(feature = "reader-xml")]
    pub(crate) fn from_xml_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        const READ_LIMIT: u64 = match cfg!(feature = "accuracy-xml") {
            true => 262_144,
            false => 131_072,
        };
        const LINE_LIMIT: usize = match cfg!(feature = "accuracy-xml") {
            true => 8,
            false => 4,
        };
        for line in reader.take(READ_LIMIT).lines().take(LINE_LIMIT) {
            let buffer = line?.to_lowercase();
            if buffer.contains("<amf") {
                return Ok(Self::AdditiveManufacturingFormat);
            } else if buffer.contains("<collada") {
                return Ok(Self::DigitalAssetExchange);
            } else if buffer.contains("<mxfile") {
                return Ok(Self::Drawio);
            } else if buffer.contains("<x3d") {
                return Ok(Self::Extensible3dGraphics);
            } else if buffer.contains("<xsl") {
                return Ok(Self::ExtensibleStylesheetLanguageTransformations);
            } else if buffer.contains("<gml") {
                return Ok(Self::GeographyMarkupLanguage);
            } else if buffer.contains("<gpx") {
                return Ok(Self::GpsExchangeFormat);
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
            } else if buffer.contains("<xliff") {
                return Ok(Self::XmlLocalizationInterchangeFileFormat);
            } else if buffer.contains("<playlist") {
                return Ok(Self::XmlShareablePlaylistFormat);
            }
        }
        Ok(Self::ExtensibleMarkupLanguage)
    }

    /// Determines file format from a [ZIP](`crate::FileFormat::Zip`) reader.
    ///
    /// It checks for certain file names or contents within the archive that indicate the presence
    /// of specific file formats. If a match is found, the corresponding variant is returned.
    /// Otherwise, it returns [ZIP](`crate::FileFormat::Zip`).
    #[cfg(feature = "reader-zip")]
    pub(crate) fn from_zip_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        const FILE_LIMIT: usize = match cfg!(feature = "accuracy-zip") {
            true => 4096,
            false => 1024,
        };
        let mut archive = zip::ZipArchive::new(reader)?;
        let mut format = Self::Zip;
        for index in 0..std::cmp::min(archive.len(), FILE_LIMIT) {
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
                "mimetype" => match read_to_string(file.take(64))?.trim() {
                    "application/epub+zip" => return Ok(Self::ElectronicPublication),
                    "application/vnd.adobe.indesign-idml-package" => {
                        return Ok(Self::IndesignMarkupLanguage)
                    }
                    "application/vnd.oasis.opendocument.graphics" => {
                        return Ok(Self::OpendocumentGraphics)
                    }
                    "application/vnd.oasis.opendocument.presentation" => {
                        return Ok(Self::OpendocumentPresentation);
                    }
                    "application/vnd.oasis.opendocument.spreadsheet" => {
                        return Ok(Self::OpendocumentSpreadsheet);
                    }
                    "application/vnd.oasis.opendocument.text" => {
                        return Ok(Self::OpendocumentText);
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

/// Checks if the `data` array contains the `target` sequence using the Boyer-Moore algorithm.
#[cfg(any(feature = "reader-mkv", feature = "reader-pdf"))]
fn contains(data: &[u8], target: &[u8]) -> bool {
    let data_len = data.len();
    let target_len = target.len();
    if target_len > data_len {
        return false;
    }
    let mut skip_table = [0; 256];
    for (index, &val) in target.iter().enumerate().rev() {
        skip_table[val as usize] = index;
    }
    let mut data_index = target_len - 1;
    while data_index < data_len {
        let mut target_index = target_len;
        while target_index > 0 && target[target_index - 1] == data[data_index] {
            target_index -= 1;
            data_index -= 1;
        }
        if target_index == 0 {
            return true;
        }
        if target_index < skip_table[data[data_index] as usize] + 1 {
            data_index += target_len - target_index;
        } else {
            data_index += target_len - skip_table[data[data_index] as usize] - 1;
        }
    }
    false
}
