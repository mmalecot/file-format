//! Binary-based file formats.

use std::{
    cmp,
    io::{Read, Result, Seek, SeekFrom},
};

impl crate::FileFormat {
    /// Determines `FileFormat` from a Compound File Binary reader.
    #[cfg(feature = "cfb")]
    pub(crate) fn from_cfb<R: Read + Seek>(reader: R) -> Result<Self> {
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

    /// Determines `FileFormat` from a Matroska Video reader.
    pub(crate) fn from_mkv<R: Read + Seek>(mut reader: R) -> Result<Self> {
        let length = reader.seek(SeekFrom::End(0))?;
        reader.rewind()?;
        let mut buf = vec![0; cmp::min(4096, length as usize)];
        reader.read_exact(&mut buf)?;
        if buf
            .windows(2)
            .position(|bytes| bytes == b"\x42\x82")
            .map(|pos| &buf[pos + 3..pos + 7])
            .filter(|bytes| bytes == b"webm")
            .is_some()
        {
            Ok(Self::Webm)
        } else {
            Ok(Self::MatroskaVideo)
        }
    }

    /// Determines `FileFormat` from a MS-DOS Executable reader.
    pub(crate) fn from_ms_dos_executable<R: Read + Seek>(mut reader: R) -> Result<Self> {
        reader.seek(SeekFrom::Start(0x3C))?;
        let mut address = [0; 4];
        reader.read_exact(&mut address)?;
        reader.seek(SeekFrom::Start(u32::from_le_bytes(address) as u64))?;
        let mut signature = [0; 4];
        reader.read_exact(&mut signature)?;
        if &signature == b"PE\x00\x00" {
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

    /// Determines `FileFormat` from a Portable Document Format reader.
    pub(crate) fn from_pdf<R: Read + Seek>(mut reader: R) -> Result<Self> {
        let length = reader.seek(SeekFrom::End(0))?;
        reader.rewind()?;
        let mut buf = vec![0; cmp::min(10 * 1024 * 1024, length as usize)];
        reader.read_exact(&mut buf)?;
        Ok(if buf.windows(13).any(|bytes| bytes == b"AIPrivateData") {
            Self::AdobeIllustratorArtwork
        } else {
            Self::PortableDocumentFormat
        })
    }

    /// Determines `FileFormat` from a ZIP reader.
    #[cfg(feature = "zip")]
    pub(crate) fn from_zip<R: Read + Seek>(reader: R) -> Result<Self> {
        let mut archive = zip::ZipArchive::new(reader)?;
        let mut format = Self::Zip;
        for index in 0..archive.len() {
            let mut file = archive.by_index(index)?;
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
                "mimetype" => {
                    let mut content = String::new();
                    file.read_to_string(&mut content)?;
                    match content.trim() {
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
                        _ => {}
                    }
                }
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
