//! Readers for specific file formats.

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
            #[cfg(feature = "reader-mp4")]
            Self::Mpeg4Part14 => Self::from_mp4_reader(reader)?,
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

        // Constants for limits.
        const SEARCH_LIMIT: usize = 8192;

        // Rewinds to the beginning of the stream.
        reader.rewind()?;

        // Gets the stream length.
        let length = reader.seek(SeekFrom::End(0))?;
        reader.rewind()?;

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
        // Rewinds to the beginning of the stream.
        reader.rewind()?;

        // Opens the compound file.
        let file = cfb::CompoundFile::open(reader)?;

        // Reads the CLSID from the root entry and returns the corresponding variant.
        Ok(match file.root_entry().clsid().to_string().as_str() {
            "e60f81e1-49b3-11d0-93c3-7e0706000000" => Self::AutodeskInventorAssembly,
            "bbf9fdf1-52dc-11d0-8c04-0800090be8ec" => Self::AutodeskInventorDrawing,
            "4d29b490-49b2-11d0-93c3-7e0706000000" => Self::AutodeskInventorPart,
            "76283a80-50dd-11d3-a7e3-00c04f79d7bc" => Self::AutodeskInventorPresentation,
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
            "00021303-0000-0000-c000-000000000046" => Self::MicrosoftWorksDatabase,
            "28cddbc3-0ae2-11ce-a29a-00aa004a1a72" => Self::MicrosoftWorksDatabase,
            "00021302-0000-0000-c000-000000000046" => Self::MicrosoftWorksWordProcessor,
            "0ea45ab2-9e0a-11d1-a407-00c04fb932ba" => Self::MicrosoftWorksWordProcessor,
            "28cddbc2-0ae2-11ce-a29a-00aa004a1a72" => Self::MicrosoftWorksWordProcessor,
            "83a33d36-27c5-11ce-bfd4-00400513bb57" => Self::SolidworksAssembly,
            "83a33d34-27c5-11ce-bfd4-00400513bb57" => Self::SolidworksDrawing,
            "83a33d30-27c5-11ce-bfd4-00400513bb57" => Self::SolidworksPart,
            "3f543fa0-b6a6-101b-9961-04021c007002" => Self::Starcalc,
            "6361d441-4235-11d0-89cb-008029e4b0b1" => Self::Starcalc,
            "c6a5b861-85d6-11d1-89cb-008029e4b0b1" => Self::Starcalc,
            "02b3b7e0-4225-11d0-89ca-008029e4b0b1" => Self::Starchart,
            "bf884321-85dd-11d1-89d0-008029e4b0b1" => Self::Starchart,
            "fb9c99e0-2c6d-101c-8e2c-00001b4cc711" => Self::Starchart,
            "2e8905a0-85bd-11d1-89d0-008029e4b0b1" => Self::Stardraw,
            "af10aae0-b36d-101b-9961-04021c007002" => Self::Stardraw,
            "012d3cc0-4216-11d0-89cb-008029e4b0b1" => Self::Starimpress,
            "565c7221-85bc-11d1-89d0-008029e4b0b1" => Self::Starimpress,
            "02b3b7e1-4225-11d0-89ca-008029e4b0b1" => Self::Starmath,
            "d4590460-35fd-101c-b12a-04021c007002" => Self::Starmath,
            "ffb5e640-85de-11d1-89d0-008029e4b0b1" => Self::Starmath,
            "8b04e9b0-420e-11d0-a45e-00a0249d57b1" => Self::Starwriter,
            "c20cf9d1-85ae-11d1-aab4-006097da561a" => Self::Starwriter,
            "dc5c7e40-b35c-101b-9961-04021c007002" => Self::Starwriter,
            "1cdd8c7b-81c0-45a0-9fed-04143144cc1e" => Self::ThreeDimensionalStudioMax,
            "519873ff-2dad-0220-1937-0000929679cd" => Self::WordperfectDocument,
            "402efe60-1999-101b-99ae-04021c007002" => Self::WordperfectGraphics,
            _ => {
                if file.exists("WksSSWorkBook") {
                    Self::MicrosoftWorks6Spreadsheet
                } else if file.exists("MatOST") {
                    Self::MicrosoftWorksWordProcessor
                } else {
                    Self::CompoundFileBinary
                }
            }
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
        const CLUSTER: u32 = 0x1F43B675;

        // Constants for limits.
        const ITERATION_LIMIT: usize = 512;
        const STRING_LIMIT: usize = 64;

        /// Helper function to read the ID of an EBML element.
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

        /// Helper function to read the size of an EBML element.
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

        // Rewinds to the beginning of the stream.
        reader.rewind()?;

        // Flags indicating the presence of audio and video codecs.
        let mut audio_codec = false;
        let mut video_codec = false;

        // Iterates through the EBML elements in the reader.
        let mut iteration_count = 0;
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
                    let mut buffer = vec![0; std::cmp::min(STRING_LIMIT, size as usize)];
                    reader.read_exact(&mut buffer)?;

                    // Converts the buffer to a string and trims null characters.
                    let doc_type = String::from_utf8_lossy(&buffer)
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
                    let mut buffer = vec![0; std::cmp::min(STRING_LIMIT, size as usize)];
                    reader.read_exact(&mut buffer)?;

                    // Converts the buffer to a string.
                    let codec_id = String::from_utf8_lossy(&buffer).to_string();

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
                CLUSTER => {
                    // No need to continue reading.
                    break;
                }
                _ => {
                    // Seeks to the next element.
                    reader.seek(SeekFrom::Current(size as i64))?;
                }
            }

            // Increments the iteration count.
            iteration_count += 1;

            // Checks if the iteration limit has been reached.
            if iteration_count == ITERATION_LIMIT {
                break;
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
        // Rewinds to the beginning of the stream.
        reader.rewind()?;

        // Gets the stream length.
        let length = reader.seek(SeekFrom::End(0))?;
        reader.rewind()?;

        // Reads the e_lfanew field.
        reader.seek(SeekFrom::Current(0x3C))?;
        let mut e_lfanew = [0; 4];
        reader.read_exact(&mut e_lfanew)?;
        let e_lfanew = u32::from_le_bytes(e_lfanew) as u64;

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

    /// Determines file format from a MP4 reader.
    #[cfg(feature = "reader-mp4")]
    pub(crate) fn from_mp4_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        // Constants for limits.
        const ITERATION_LIMIT: usize = 512;

        // Rewinds to the beginning of the stream.
        reader.rewind()?;

        // Flags indicating the presence of audio, video and subtitles tracks.
        let mut audio_track = false;
        let mut video_track = false;
        let mut subtitles_track = false;

        // Iterates through boxes in the reader.
        let mut iteration_count = 0;
        let mut box_header = [0; 8];
        while let Ok(_) = reader.read_exact(&mut box_header) {
            let box_size =
                u32::from_be_bytes([box_header[0], box_header[1], box_header[2], box_header[3]]);
            match &box_header[4..8] {
                b"moov" | b"trak" | b"mdia" => {
                    // Does nothing for these boxes.
                }
                b"hdlr" => {
                    // Skips the first 8 bytes.
                    reader.seek(SeekFrom::Current(8))?;

                    // Reads the handler type.
                    let mut handler_type = [0; 4];
                    reader.read_exact(&mut handler_type)?;

                    // Checks the handler type.
                    if &handler_type == b"vide" {
                        video_track = true
                    } else if &handler_type == b"soun" {
                        audio_track = true
                    } else if &handler_type == b"sbtl"
                        || &handler_type == b"subt"
                        || &handler_type == b"text"
                    {
                        subtitles_track = true
                    }

                    // Seeks to the next box.
                    reader.seek(SeekFrom::Current(box_size as i64 - 20))?;
                }
                _ => {
                    // Seeks to the next box.
                    reader.seek(SeekFrom::Current(box_size as i64 - 8))?;
                }
            }

            // Increments the iteration count.
            iteration_count += 1;

            // Checks if the iteration limit has been reached.
            if iteration_count == ITERATION_LIMIT {
                break;
            }
        }

        // Determines the file format based on the identified tracks.
        Ok(if video_track {
            Self::Mpeg4Part14Video
        } else if audio_track {
            Self::Mpeg4Part14Audio
        } else if subtitles_track {
            Self::Mpeg4Part14Subtitles
        } else {
            Self::Mpeg4Part14
        })
    }

    /// Determines file format from a PDF reader.
    #[cfg(feature = "reader-pdf")]
    pub(crate) fn from_pdf_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self> {
        // Constants for limits.
        const SEARCH_LIMIT: usize = 4_194_304;

        // Rewinds to the beginning of the stream.
        reader.rewind()?;

        // Gets the stream length.
        let length = reader.seek(SeekFrom::End(0))?;
        reader.rewind()?;

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
        // Constants for limits.
        const SEARCH_LIMIT: usize = 4096;

        // Rewinds to the beginning of the stream.
        reader.rewind()?;

        // Gets the stream length.
        let length = reader.seek(SeekFrom::End(0))?;
        reader.rewind()?;

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
        // Constants for limits.
        const READ_LIMIT: u64 = 8_388_608;
        const LINE_LIMIT: usize = 256;

        // Rewinds to the beginning of the stream.
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
        // Constants for limits.
        const READ_LIMIT: u64 = 262_144;
        const LINE_LIMIT: usize = 8;
        const CHAR_LIMIT: usize = 2048;

        // Rewinds to the beginning of the stream.
        reader.rewind()?;

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
            } else if contains(&buffer, b"<feed") {
                return Ok(Self::Atom);
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
            } else if contains(&buffer, b"<math") {
                return Ok(Self::MathematicalMarkupLanguage);
            } else if contains(&buffer, b"<MPD") {
                return Ok(Self::MpegDashManifest);
            } else if contains(&buffer, b"<score-partwise") {
                return Ok(Self::Musicxml);
            } else if contains(&buffer, b"<rss") {
                return Ok(Self::ReallySimpleSyndication);
            } else if contains(&buffer, b"<SVG") || contains(&buffer, b"<svg") {
                return Ok(Self::ScalableVectorGraphics);
            } else if contains(&buffer, b"<soap") {
                return Ok(Self::SimpleObjectAccessProtocol);
            } else if contains(&buffer, b"<map") {
                return Ok(Self::TiledMapXml);
            } else if contains(&buffer, b"<tileset") {
                return Ok(Self::TiledTilesetXml);
            } else if contains(&buffer, b"<tt")
                && contains(&buffer, b"xmlns=\"http://www.w3.org/ns/ttml\"")
            {
                return Ok(Self::TimedTextMarkupLanguage);
            } else if contains(&buffer, b"<TrainingCenterDatabase") {
                return Ok(Self::TrainingCenterXml);
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
        // Constants for limits.
        const FILE_LIMIT: usize = 4096;

        // Rewinds to the beginning of the stream.
        reader.rewind()?;

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
                    "application/vnd.sun.xml.calc" => return Ok(Self::SunXmlCalc),
                    "application/vnd.sun.xml.calc.template" => return Ok(Self::SunXmlCalcTemplate),
                    "application/vnd.sun.xml.draw" => return Ok(Self::SunXmlDraw),
                    "application/vnd.sun.xml.draw.template" => return Ok(Self::SunXmlDrawTemplate),
                    "application/vnd.sun.xml.impress" => return Ok(Self::SunXmlImpress),
                    "application/vnd.sun.xml.impress.template" => {
                        return Ok(Self::SunXmlImpressTemplate)
                    }
                    "application/vnd.sun.xml.math" => return Ok(Self::SunXmlMath),
                    "application/vnd.sun.xml.writer" => return Ok(Self::SunXmlWriter),
                    "application/vnd.sun.xml.writer.global" => return Ok(Self::SunXmlWriterGlobal),
                    "application/vnd.sun.xml.writer.template" => {
                        return Ok(Self::SunXmlWriterTemplate)
                    }
                    "image/openraster" => return Ok(Self::Openraster),
                    _ => {}
                },
                _ => {
                    if file.name().starts_with("Fusion[Active]/") {
                        return Ok(Self::Autodesk123d);
                    } else if file.name().starts_with("circuitdiagram/") {
                        return Ok(Self::CircuitDiagramDocument);
                    } else if file.name().starts_with("dwf/") {
                        return Ok(Self::DesignWebFormatXps);
                    } else if file.name().ends_with(".fb2") && !file.name().contains('/') {
                        return Ok(Self::FictionbookZipped);
                    } else if file.name().starts_with("FusionAssetName[Active]/") {
                        return Ok(Self::Fusion360);
                    } else if file.name().starts_with("Payload/") && file.name().contains(".app/") {
                        return Ok(Self::IosAppStorePackage);
                    } else if file.name().starts_with("word/") {
                        return Ok(Self::OfficeOpenXmlDocument);
                    } else if file.name().starts_with("visio/") {
                        return Ok(Self::OfficeOpenXmlDrawing);
                    } else if file.name().starts_with("ppt/") {
                        return Ok(Self::OfficeOpenXmlPresentation);
                    } else if file.name().starts_with("xl/") {
                        return Ok(Self::OfficeOpenXmlSpreadsheet);
                    } else if file.name().starts_with("SpaceClaim/") {
                        return Ok(Self::SpaceclaimDocument);
                    } else if file.name().starts_with("3D/") && file.name().ends_with(".model") {
                        return Ok(Self::ThreeDimensionalManufacturingFormat);
                    } else if (file.name().ends_with(".usd")
                        || file.name().ends_with(".usda")
                        || file.name().ends_with(".usdc"))
                        && !file.name().contains('/')
                    {
                        return Ok(Self::UniversalSceneDescriptionZipped);
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
