//! Read.

use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Result, Seek, SeekFrom};

/// Provides several methods for reading and searching within a reader.
trait AdvancedRead: Read + Seek {
    /// Reads two bytes from the reader and returns them as a little-endian `u16` value.
    #[inline]
    fn read_le_u16(&mut self) -> Result<u16> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    /// Reads four bytes from the reader and returns them as a little-endian `u32` value.
    #[inline]
    fn read_le_u32(&mut self) -> Result<u32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    /// Checks if the stream contains the given `bytes` within the first `size` bytes.
    #[inline]
    fn contains(&mut self, bytes: &[u8], size: u64) -> Result<bool> {
        self.rewind()?;
        Ok(BufReader::new(self.take(size))
            .fill_buf()?
            .windows(bytes.len())
            .any(|window| window == bytes))
    }
}

/// Implements the `AdvancedRead` trait for any type `R` that implements the `Read` and `Seek`
/// traits.
///
/// This allows types such as `BufReader<R>` to use the methods provided by the `AdvancedRead`
/// trait.
impl<R: Read + Seek + ?Sized> AdvancedRead for R {}

impl crate::FileFormat {
    /// Attempts to parse the reader as a CFB.
    ///
    /// It extracts its root entry's CLSID, then compares it to a set of known values and returns
    /// the corresponding variant. If the CLSID does not match any of the known values, the function
    /// returns the `CompoundFileBinary` variant.
    #[cfg(feature = "cfb")]
    pub(crate) fn from_cfb<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
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
    /// It then seeks to this address and reads the `Signature` field. If this dword is
    /// `0x00004550`, it indicates that it is a Portable Executable. Otherwise, it returns the
    /// `MsDosExecutable` variant.
    ///
    /// Finally, it seeks to `0x12` offset and reads the `Characteristics` field. If this word has
    /// the `0x2000` bit set (`IMAGE_FILE_DLL`), it returns the `DynamicLinkLibrary` variant.
    /// Otherwise, it returns the `PortableExecutable` variant.
    pub(crate) fn from_exe<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        reader.seek(SeekFrom::Start(0x3C))?;
        let address = reader.read_le_u32()?;
        reader.seek(SeekFrom::Start(address as u64))?;
        if reader.read_le_u32()? == 0x00004550 {
            reader.seek(SeekFrom::Current(0x12))?;
            return Ok(if reader.read_le_u16()? & 0x2000 == 0x2000 {
                Self::DynamicLinkLibrary
            } else {
                Self::PortableExecutable
            });
        }
        Ok(Self::MsDosExecutable)
    }

    /// Searches the reader for the "webm" byte sequence. If this sequence is found the function
    /// returns the `Webm` variant. Otherwise, it returns the `MatroskaVideo` variant.
    pub(crate) fn from_mkv<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        const SEARCH_LIMIT: u64 = match cfg!(feature = "accuracy") {
            true => 4096,
            false => 1024,
        };
        Ok(if reader.contains(b"webm", SEARCH_LIMIT)? {
            Self::Webm
        } else {
            Self::MatroskaVideo
        })
    }

    /// Searches the reader for the "AIPrivateData" byte sequence. If this sequence is found the
    /// function returns the `AdobeIllustratorArtwork` variant. Otherwise, it returns the
    /// `PortableDocumentFormat` variant.
    pub(crate) fn from_pdf<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        const SEARCH_LIMIT: u64 = match cfg!(feature = "accuracy") {
            true => 4_194_304,
            false => 1_048_576,
        };
        Ok(if reader.contains(b"AIPrivateData", SEARCH_LIMIT)? {
            Self::AdobeIllustratorArtwork
        } else {
            Self::PortableDocumentFormat
        })
    }

    /// Attempts to determine if the reader contains Plain Text by checking the first lines for
    /// control characters. If any control characters (other than whitespace) are found, this
    /// function returns an error. Otherwise, it returns the `PlainText` variant.
    pub(crate) fn from_txt<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        const READ_LIMIT: u64 = match cfg!(feature = "accuracy") {
            true => 8_388_608,
            false => 1_048_576,
        };
        const LINE_LIMIT: usize = match cfg!(feature = "accuracy") {
            true => 256,
            false => 32,
        };
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

    /// Searches the reader for byte sequences that indicate the presence of various XML-based
    /// formats. If none are found, it returns the `ExtensibleMarkupLanguage` variant.
    pub(crate) fn from_xml<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        const SEARCH_LIMIT: u64 = match cfg!(feature = "accuracy") {
            true => 1024,
            false => 256,
        };
        Ok(if reader.contains(b"<xsl", SEARCH_LIMIT)? {
            Self::ExtensibleStylesheetLanguageTransformations
        } else if reader.contains(b"<gml", SEARCH_LIMIT)? {
            Self::GeographyMarkupLanguage
        } else if reader.contains(b"<kml", SEARCH_LIMIT)? {
            Self::KeyholeMarkupLanguage
        } else if reader.contains(b"<score-partwise", SEARCH_LIMIT)? {
            Self::Musicxml
        } else if reader.contains(b"<rss", SEARCH_LIMIT)? {
            Self::ReallySimpleSyndication
        } else if reader.contains(b"<svg", SEARCH_LIMIT)? {
            Self::ScalableVectorGraphics
        } else if reader.contains(b"<soap", SEARCH_LIMIT)? {
            Self::SimpleObjectAccessProtocol
        } else {
            Self::ExtensibleMarkupLanguage
        })
    }

    /// Attempts to parse the reader as a ZIP.
    ///
    /// It checks for certain file names or contents within the archive that indicate the presence
    /// of specific file formats. If a match is found, the corresponding variant is returned.
    /// Otherwise, it returns the `Zip` variant.
    #[cfg(feature = "zip")]
    pub(crate) fn from_zip<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        const FILE_LIMIT: usize = match cfg!(feature = "accuracy") {
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
