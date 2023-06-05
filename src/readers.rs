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
            #[cfg(feature = "reader-asf")]
            Self::AdvancedSystemsFormat => Self::from_asf_reader(reader)?,
            #[cfg(feature = "reader-cfb")]
            Self::CompoundFileBinary => Self::from_cfb_reader(reader)?,
            #[cfg(feature = "reader-ebml")]
            Self::ExtensibleBinaryMetaLanguage => Self::from_ebml_reader(reader)?,
            #[cfg(feature = "reader-exe")]
            Self::MsDosExecutable => Self::from_exe_reader(reader)?,
            #[cfg(feature = "reader-pdf")]
            Self::PortableDocumentFormat => Self::from_pdf_reader(reader)?,
            #[cfg(feature = "reader-rm")]
            Self::Realmedia => Self::from_rm_reader(reader)?,
            #[cfg(feature = "reader-xml")]
            Self::ExtensibleMarkupLanguage => Self::from_xml_reader(reader)?,
            #[cfg(feature = "reader-zip")]
            Self::Zip => Self::from_zip_reader(reader)?,
            _ => format,
        })
    }

    /// Determines file format from a generic reader.
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

    /// Determines file format from an ASF reader.
    #[cfg(feature = "reader-asf")]
    pub(crate) fn from_asf_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        // Constants representing GUIDs and descriptors.
        const VIDEO_MEDIA_GUID: &[u8] =
            b"\xC0\xEF\x19\xBC\x4D\x5B\xCF\x11\xA8\xFD\x00\x80\x5F\x5C\x44\x2B";
        const AUDIO_MEDIA_GUID: &[u8] =
            b"\x40\x9E\x69\xF8\x4D\x5B\xCF\x11\xA8\xFD\x00\x80\x5F\x5C\x44\x2B";
        const DVR_DESCRIPTOR: &[u8] =
            b"D\x00V\x00R\x00 \x00F\x00i\x00l\x00e\x00 \x00V\x00e\x00r\x00s\x00i\x00o\x00n";

        // Sets limits.
        const SEARCH_LIMIT: usize = 8192;

        // Gets the stream length.
        let old_pos = reader.stream_position()?;
        let length = reader.seek(SeekFrom::End(0))?;
        if old_pos != length {
            reader.seek(SeekFrom::Start(old_pos))?;
        }

        // Fills the buffer.
        let mut buffer = vec![0; std::cmp::min(SEARCH_LIMIT, length as usize)];
        reader.read_exact(&mut buffer)?;

        // Searches for an Extended Content Description descriptor named "DVR File Version" in the
        // buffer.
        if contains(&buffer, DVR_DESCRIPTOR) {
            return Ok(Self::MicrosoftDigitalVideoRecording);
        }

        // Searches for specific GUIDs in the buffer.
        Ok(if contains(&buffer, VIDEO_MEDIA_GUID) {
            Self::WindowsMediaVideo
        } else if contains(&buffer, AUDIO_MEDIA_GUID) {
            Self::WindowsMediaAudio
        } else {
            Self::AdvancedSystemsFormat
        })
    }

    /// Determines file format from a CFB reader.
    #[cfg(feature = "reader-cfb")]
    pub(crate) fn from_cfb_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        // Opens the compound file.
        let file = cfb::CompoundFile::open(reader)?;

        // Reads the CLSID from the root entry and returns the corresponding variant.
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

    /// Determines file format from an EBML reader.
    #[cfg(feature = "reader-ebml")]
    pub(crate) fn from_ebml_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        // Constants representing EBML element IDs.
        const EBML: u32 = 0x1A45DFA3;
        const DOC_TYPE: u32 = 0x4282;
        const SEGMENT: u32 = 0x18538067;
        const TRACKS: u32 = 0x1654AE6B;
        const TRACK_ENTRY: u32 = 0xAE;
        const CODEC_ID: u32 = 0x86;
        const VIDEO: u32 = 0xE0;
        const STEREO_MODE: u32 = 0x53B8;

        // Helper function to read the ID of an EBML element.
        fn read_id<R: Read>(reader: &mut R) -> Result<u32> {
            // Reads the first byte.
            let mut first_byte = [0];
            reader.read_exact(&mut first_byte)?;

            // Determines the number of bytes used to represent the ID.
            let num_bytes = first_byte[0].leading_zeros() + 1;
            if num_bytes > 4 {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid EBML ID"));
            }

            // Calculates the ID value based on the number of bytes.
            let mut value = first_byte[0] as u32;
            for _ in 1..num_bytes {
                let mut byte = [0];
                reader.read_exact(&mut byte)?;
                value = (value << 8) | (byte[0] as u32);
            }
            Ok(value)
        }

        // Helper function to read the size of an EBML element.
        fn read_size<R: Read>(reader: &mut R) -> Result<u64> {
            // Reads the first byte.
            let mut first_byte = [0];
            reader.read_exact(&mut first_byte)?;

            // Determines the number of bytes used to represent the size.
            let num_bytes = first_byte[0].leading_zeros() + 1;
            if num_bytes > 8 {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid EBML size"));
            }

            // Calculates the size value based on the number of bytes.
            let mut value = u64::from(first_byte[0] & ((128 >> first_byte[0].leading_zeros()) - 1));
            for _ in 1..num_bytes {
                let mut byte = [0];
                reader.read_exact(&mut byte)?;
                value = (value << 8) | (byte[0] as u64);
            }
            Ok(value)
        }

        // Flag indicating the presence of an audio codec.
        let mut audio_codec = false;

        // Flag indicating the presence of a video codec.
        let mut video_codec = false;

        // Loops through the EBML elements in the reader.
        while let Ok(id) = read_id(reader) {
            // Reads the size of the element.
            let size = read_size(reader)?;

            // Checks the ID of the element to perform specific actions.
            match id {
                EBML | SEGMENT | TRACKS | TRACK_ENTRY | VIDEO => {
                    // Does nothing for these elements.
                }
                DOC_TYPE => {
                    // Reads the buffer containing the DocType.
                    let mut buffer = vec![0; std::cmp::min(16, size as usize)];
                    reader.read_exact(&mut buffer)?;

                    // Converts the buffer to a string and trims null characters.
                    let doc_type = String::from_utf8(buffer)
                        .unwrap_or_default()
                        .trim_end_matches('\0')
                        .to_string();

                    // Checks the DocType.
                    if doc_type == "webm" {
                        return Ok(Self::Webm);
                    } else if doc_type != "matroska" {
                        return Ok(Self::ExtensibleBinaryMetaLanguage);
                    }
                }
                CODEC_ID => {
                    // Reads the buffer containing the Codec ID.
                    let mut buffer = vec![0; std::cmp::min(64, size as usize)];
                    reader.read_exact(&mut buffer)?;

                    // Converts the buffer to a string.
                    let codec_id = String::from_utf8(buffer).unwrap_or_default();

                    // Checks the Codec ID.
                    if codec_id.starts_with("A_") {
                        audio_codec = true;
                    }
                    if codec_id.starts_with("V_") {
                        video_codec = true;
                    }
                }
                STEREO_MODE => {
                    // Reads a single byte to determine the StereoMode.
                    let mut buffer = [0];
                    reader.read_exact(&mut buffer)?;

                    // Positive value indicates stereoscopic video.
                    if buffer[0] > 0 {
                        return Ok(Self::Matroska3dVideo);
                    }
                }
                _ => {
                    // Seeks to the next element.
                    reader.seek(SeekFrom::Current(size as i64))?;
                }
            }
        }

        // Determines the file format based on the identified codecs.
        Ok(if video_codec {
            Self::MatroskaVideo
        } else if audio_codec {
            Self::MatroskaAudio
        } else {
            Self::ExtensibleBinaryMetaLanguage
        })
    }

    /// Determines file format from an EXE reader.
    #[cfg(feature = "reader-exe")]
    pub(crate) fn from_exe_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        // Reads the e_lfanew field.
        reader.seek(SeekFrom::Start(0x3C))?;
        let mut e_lfanew = [0; 4];
        reader.read_exact(&mut e_lfanew)?;
        let e_lfanew = u32::from_le_bytes(e_lfanew) as u64;

        // Gets the stream length.
        let old_pos = reader.stream_position()?;
        let length = reader.seek(SeekFrom::End(0))?;
        if old_pos != length {
            reader.seek(SeekFrom::Start(old_pos))?;
        }

        // Checks that the e_lfanew value is not outside the stream's boundaries.
        if e_lfanew + 4 < length {
            // Seeks to e_lfanew.
            reader.seek(SeekFrom::Start(e_lfanew))?;

            // Reads the signature.
            let mut signature = [0; 4];
            reader.read_exact(&mut signature)?;

            // Checks the signature.
            if &signature == b"PE\0\0" {
                reader.seek(SeekFrom::Current(0x12))?;
                let mut characteristics = [0; 2];
                reader.read_exact(&mut characteristics)?;
                return Ok(if u16::from_le_bytes(characteristics) & 0x2000 == 0x2000 {
                    Self::DynamicLinkLibrary
                } else {
                    Self::PortableExecutable
                });
            } else if &signature[..2] == b"LE" || &signature[..2] == b"LX" {
                return Ok(Self::LinearExecutable);
            } else if &signature[..2] == b"NE" {
                return Ok(Self::NewExecutable);
            }
        }
        Ok(Self::MsDosExecutable)
    }

    /// Determines file format from a PDF reader.
    #[cfg(feature = "reader-pdf")]
    pub(crate) fn from_pdf_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        // Sets limits.
        const SEARCH_LIMIT: usize = 4_194_304;

        // Gets the stream length.
        let old_pos = reader.stream_position()?;
        let length = reader.seek(SeekFrom::End(0))?;
        if old_pos != length {
            reader.seek(SeekFrom::Start(old_pos))?;
        }

        // Fills the buffer.
        let mut buffer = vec![0; std::cmp::min(SEARCH_LIMIT, length as usize)];
        reader.read_exact(&mut buffer)?;

        // Searches for the "AIPrivateData" sequence in the buffer.
        Ok(if contains(&buffer, b"AIPrivateData") {
            Self::AdobeIllustratorArtwork
        } else {
            Self::PortableDocumentFormat
        })
    }

    /// Determines file format from a RM reader.
    #[cfg(feature = "reader-rm")]
    pub(crate) fn from_rm_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        // Sets limits.
        const SEARCH_LIMIT: usize = 4096;

        // Gets the stream length.
        let old_pos = reader.stream_position()?;
        let length = reader.seek(SeekFrom::End(0))?;
        if old_pos != length {
            reader.seek(SeekFrom::Start(old_pos))?;
        }

        // Fills the buffer.
        let mut buffer = vec![0; std::cmp::min(SEARCH_LIMIT, length as usize)];
        reader.read_exact(&mut buffer)?;

        // Searches for the media type in the buffer.
        Ok(if contains(&buffer, b"video/x-pn-realvideo") {
            Self::Realvideo
        } else if contains(&buffer, b"audio/x-pn-realaudio")
            || contains(&buffer, b"audio/x-pn-multirate-realaudio")
        {
            Self::Realaudio
        } else {
            Self::Realmedia
        })
    }

    /// Determines file format from a TXT reader.
    #[cfg(feature = "reader-txt")]
    pub(crate) fn from_txt_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        // Sets limits.
        const READ_LIMIT: u64 = 8_388_608;
        const LINE_LIMIT: usize = 256;

        // Rewinds to the beginning of a stream.
        reader.rewind()?;

        // Determines if the reader contains only ASCII/UTF-8-encoded text by checking the first
        // lines for control characters other than whitespace.
        reader
            .take(READ_LIMIT)
            .lines()
            .take(LINE_LIMIT)
            .try_for_each(|line| {
                line?
                    .chars()
                    .find(|char| char.is_control() && !char.is_whitespace())
                    .map(|_| Err(Error::new(ErrorKind::InvalidData, "Invalid characters")))
                    .unwrap_or(Ok(()))
            })
            .map(|_| Self::PlainText)
    }

    /// Determines file format from a XML reader.
    #[cfg(feature = "reader-xml")]
    pub(crate) fn from_xml_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        // Sets limits.
        const READ_LIMIT: u64 = 262_144;
        const LINE_LIMIT: usize = 8;
        const CHAR_LIMIT: usize = 2048;

        // Searches the reader for byte sequences that indicate the presence of various file
        // formats.
        for line in reader.take(READ_LIMIT).lines().take(LINE_LIMIT) {
            let buffer: Vec<u8> = line?.as_bytes().iter().take(CHAR_LIMIT).copied().collect();
            if contains(&buffer, b"<abiword template=\"false\"") {
                return Ok(Self::Abiword);
            } else if contains(&buffer, b"<abiword template=\"true\"") {
                return Ok(Self::AbiwordTemplate);
            } else if contains(&buffer, b"<amf") {
                return Ok(Self::AdditiveManufacturingFormat);
            } else if contains(&buffer, b"<ASX") || contains(&buffer, b"<asx") {
                return Ok(Self::AdvancedStreamRedirector);
            } else if contains(&buffer, b"<COLLADA") || contains(&buffer, b"<collada") {
                return Ok(Self::DigitalAssetExchange);
            } else if contains(&buffer, b"<mxfile") {
                return Ok(Self::Drawio);
            } else if contains(&buffer, b"<X3D") || contains(&buffer, b"<x3d") {
                return Ok(Self::Extensible3d);
            } else if contains(&buffer, b"<xsl") {
                return Ok(Self::ExtensibleStylesheetLanguageTransformations);
            } else if contains(&buffer, b"<FictionBook") {
                return Ok(Self::Fictionbook);
            } else if contains(&buffer, b"<gml") {
                return Ok(Self::GeographyMarkupLanguage);
            } else if contains(&buffer, b"<gpx") {
                return Ok(Self::GpsExchangeFormat);
            } else if contains(&buffer, b"<kml") {
                return Ok(Self::KeyholeMarkupLanguage);
            } else if contains(&buffer, b"<score-partwise") {
                return Ok(Self::Musicxml);
            } else if contains(&buffer, b"<rss") {
                return Ok(Self::ReallySimpleSyndication);
            } else if contains(&buffer, b"<svg") {
                return Ok(Self::ScalableVectorGraphics);
            } else if contains(&buffer, b"<soap") {
                return Ok(Self::SimpleObjectAccessProtocol);
            } else if contains(&buffer, b"<TrainingCenterDatabase") {
                return Ok(Self::TrainingCenterXml);
            } else if contains(&buffer, b"<tt xmlns=\"http://www.w3.org/ns/ttml\"") {
                return Ok(Self::TimedTextMarkupLanguage);
            } else if contains(&buffer, b"<USFSubtitles") {
                return Ok(Self::UniversalSubtitleFormat);
            } else if contains(&buffer, b"<xliff") {
                return Ok(Self::XmlLocalizationInterchangeFileFormat);
            } else if contains(&buffer, b"<playlist") {
                return Ok(Self::XmlShareablePlaylistFormat);
            }
        }
        Ok(Self::ExtensibleMarkupLanguage)
    }

    /// Determines file format from a ZIP reader.
    #[cfg(feature = "reader-zip")]
    pub(crate) fn from_zip_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        // Sets limits.
        const FILE_LIMIT: usize = 4096;

        // Opens the archive.
        let mut archive = zip::ZipArchive::new(reader)?;

        // Sets the default variant.
        let mut format = Self::Zip;

        // Browses archive files.
        for index in 0..std::cmp::min(archive.len(), FILE_LIMIT) {
            let file = archive.by_index(index)?;
            match file.name() {
                "AndroidManifest.xml" => return Ok(Self::AndroidPackage),
                "AppManifest.xaml" => return Ok(Self::Xap),
                "AppxManifest.xml" => return Ok(Self::WindowsAppPackage),
                "META-INF/AIR/application.xml" => return Ok(Self::AdobeIntegratedRuntime),
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
                    "application/vnd.oasis.opendocument.base"
                    | "application/vnd.oasis.opendocument.database" => {
                        return Ok(Self::OpendocumentDatabase)
                    }
                    "application/vnd.oasis.opendocument.formula" => {
                        return Ok(Self::OpendocumentFormula)
                    }
                    "application/vnd.oasis.opendocument.formula-template" => {
                        return Ok(Self::OpendocumentFormulaTemplate)
                    }
                    "application/vnd.oasis.opendocument.graphics" => {
                        return Ok(Self::OpendocumentGraphics)
                    }
                    "application/vnd.oasis.opendocument.graphics-template" => {
                        return Ok(Self::OpendocumentGraphicsTemplate)
                    }
                    "application/vnd.oasis.opendocument.presentation" => {
                        return Ok(Self::OpendocumentPresentation);
                    }
                    "application/vnd.oasis.opendocument.presentation-template" => {
                        return Ok(Self::OpendocumentPresentationTemplate);
                    }
                    "application/vnd.oasis.opendocument.spreadsheet" => {
                        return Ok(Self::OpendocumentSpreadsheet);
                    }
                    "application/vnd.oasis.opendocument.spreadsheet-template" => {
                        return Ok(Self::OpendocumentSpreadsheetTemplate);
                    }
                    "application/vnd.oasis.opendocument.text" => {
                        return Ok(Self::OpendocumentText);
                    }
                    "application/vnd.oasis.opendocument.text-master" => {
                        return Ok(Self::OpendocumentTextMaster);
                    }
                    "application/vnd.oasis.opendocument.text-master-template" => {
                        return Ok(Self::OpendocumentTextMasterTemplate);
                    }
                    "application/vnd.oasis.opendocument.text-template" => {
                        return Ok(Self::OpendocumentTextTemplate);
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
                    } else if file.name().ends_with(".fb2") {
                        return Ok(Self::FictionbookZipped);
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
#[cfg(any(
    feature = "reader-asf",
    feature = "reader-pdf",
    feature = "reader-rm",
    feature = "reader-xml"
))]
fn contains(data: &[u8], target: &[u8]) -> bool {
    // An empty target sequence is always considered to be contained in the data.
    if target.is_empty() {
        return true;
    }

    // The data array is shorter than the target sequence, so it cannot contain the target.
    if data.len() < target.len() {
        return false;
    }

    // Builds the bad character shift table.
    let mut bad_char_table = [0; 256];
    for (index, &char) in target.iter().enumerate().take(target.len() - 1) {
        bad_char_table[char as usize] = target.len() - 1 - index;
    }

    // Starts searching from the last possible position in the data array.
    let mut pos = target.len() - 1;
    while pos < data.len() {
        let mut target_index = target.len() - 1;
        let mut data_index = pos;
        while data[data_index] == target[target_index] {
            if target_index == 0 {
                return true;
            }
            target_index -= 1;
            data_index -= 1;
        }

        // Calculates the maximum shift based on the bad character rule and good suffix rule.
        let bad_char_shift = bad_char_table[data[pos] as usize];
        let good_suffix_shift = target.len() - target_index;
        let shift = std::cmp::max(bad_char_shift, good_suffix_shift);
        pos += shift;
    }
    false
}
