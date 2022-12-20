//! Read.

use std::{
    cmp,
    io::{BufRead, BufReader, Error, ErrorKind, Read, Result, Seek, SeekFrom},
    str,
};

/// Extends [`Read`] and [`Seek`].
trait AdvancedRead: Read + Seek {
    /// Reads an unsigned 16 bit integer in little endian.
    #[inline]
    fn read_le_u16(&mut self) -> Result<u16> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    /// Reads an unsigned 32 bit integer in little endian.
    #[inline]
    fn read_le_u32(&mut self) -> Result<u32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    /// Searches for bytes.
    #[inline]
    fn search(&mut self, bytes: &[u8], size: u64) -> Result<bool> {
        let position = self.stream_position()?;
        let length = self.seek(SeekFrom::End(0))?;
        self.seek(SeekFrom::Start(position))?;
        let mut buffer = vec![0; cmp::min(size as usize, length.saturating_sub(position) as usize)];
        self.read_exact(&mut buffer)?;
        self.seek(SeekFrom::Start(position))?;
        Ok(buffer.windows(bytes.len()).any(|window| window == bytes))
    }
}

impl<R: Read + Seek + ?Sized> AdvancedRead for R {}

impl crate::FileFormat {
    /// Determines [`FileFormat`] from a **Compound File Binary** reader.
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

    /// Determines [`FileFormat`] from a **Matroska Video** reader.
    pub(crate) fn from_mkv<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        Ok(if reader.search(b"webm", 4096)? {
            Self::Webm
        } else {
            Self::MatroskaVideo
        })
    }

    /// Determines [`FileFormat`] from a **MS-DOS Executable** reader.
    pub(crate) fn from_ms_dos_exe<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        reader.seek(SeekFrom::Start(0x3C))?;
        let address = reader.read_le_u32()?;
        reader.seek(SeekFrom::Start(address as u64))?;
        if reader.read_le_u32()? == 0x4550 {
            reader.seek(SeekFrom::Current(0x12))?;
            return Ok(if reader.read_le_u16()? & 0x2000 == 0x2000 {
                Self::DynamicLinkLibrary
            } else {
                Self::PortableExecutable
            });
        }
        Ok(Self::MsDosExecutable)
    }

    /// Determines [`FileFormat`] from a **Portable Document Format** reader.
    pub(crate) fn from_pdf<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        Ok(if reader.search(b"AIPrivateData", 1048576)? {
            Self::AdobeIllustratorArtwork
        } else {
            Self::PortableDocumentFormat
        })
    }

    /// Determines [`FileFormat`] from a **Plain Text** reader.
    pub(crate) fn from_plain_text<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        let mut reader = reader.take(1048576);
        let mut buffer = Vec::new();
        let mut index = 0;
        while index < 32 && reader.read_until(b'\n', &mut buffer)? > 0 {
            if str::from_utf8(&buffer).map_or(true, |str| {
                str.chars()
                    .any(|char| char.is_control() && !char.is_whitespace())
            }) {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid UTF-8 text"));
            }
            buffer.clear();
            index += 1;
        }
        Ok(Self::PlainText)
    }

    /// Determines [`FileFormat`] from a **ZIP** reader.
    #[cfg(feature = "zip")]
    pub(crate) fn from_zip<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        let mut archive = zip::ZipArchive::new(reader)?;
        let mut format = Self::Zip;
        for index in 0..cmp::min(archive.len(), 1024) {
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
