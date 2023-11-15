//! Readers for specific file formats.

use std::io::*;

impl crate::FileFormat {
    /// Determines file format from the specified format reader.
    #[allow(unused_variables)]
    #[inline]
    pub(crate) fn from_format_reader<R: Read + Seek>(format: Self, reader: R) -> Result<Self> {
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
    pub(crate) fn from_generic_reader<R: Read + Seek>(reader: R) -> Self {
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
    pub(crate) fn from_asf_reader<R: Read + Seek>(mut reader: R) -> Result<Self> {
        // UTF-16-encoded marker for DVR-MS file format.
        const DVR_MS_MARKER: &[u8] = b"D\0V\0R\0 \0F\0i\0l\0e\0 \0V\0e\0r\0s\0i\0o\0n\0";

        // Binary-encoded GUID for audio media (f8699e40-5b4d-11cf-a8fd-00805f5c442b).
        const AUDIO_MEDIA_GUID: &[u8] =
            b"\x40\x9E\x69\xF8\x4D\x5B\xCF\x11\xA8\xFD\x00\x80\x5F\x5C\x44\x2B";

        // Binary-encoded GUID for video media (bc19efc0-5b4d-11cf-a8fd-00805f5c442b).
        const VIDEO_MEDIA_GUID: &[u8] =
            b"\xC0\xEF\x19\xBC\x4D\x5B\xCF\x11\xA8\xFD\x00\x80\x5F\x5C\x44\x2B";

        // Rewinds to the beginning of the stream plus the size of the ASF file format signature.
        reader.seek(SeekFrom::Start(16))?;

        // Creates and fills a buffer.
        let mut buffer = [0; 8192];
        let bytes_read = reader.read(&mut buffer)?;

        // Searches for specific markers and GUIDs in the buffer.
        Ok(if contains(&buffer[..bytes_read], DVR_MS_MARKER) {
            Self::MicrosoftDigitalVideoRecording
        } else if contains(&buffer[..bytes_read], VIDEO_MEDIA_GUID) {
            Self::WindowsMediaVideo
        } else if contains(&buffer[..bytes_read], AUDIO_MEDIA_GUID) {
            Self::WindowsMediaAudio
        } else {
            Self::AdvancedSystemsFormat
        })
    }

    /// Determines file format from a CFB reader.
    #[cfg(feature = "reader-cfb")]
    pub(crate) fn from_cfb_reader<R: Read + Seek>(mut reader: R) -> Result<Self> {
        // UTF-16-encoded entry name for WPS file format.
        const WPS_ENTRY_NAME: &[u8] = b"M\0a\0t\0O\0S\0T\0";

        // UTF-16-encoded entry name for XLR file format.
        const XLR_ENTRY_NAME: &[u8] = b"W\0k\0s\0S\0S\0W\0o\0r\0k\0B\0o\0o\0k\0";

        // Reads the major version.
        reader.seek(SeekFrom::Start(26))?;
        let mut buffer = [0; 2];
        reader.read_exact(&mut buffer)?;
        let major_version = u16::from_le_bytes(buffer);

        // Reads the first directory sector location.
        reader.seek(SeekFrom::Current(20))?;
        let mut buffer = [0; 4];
        reader.read_exact(&mut buffer)?;
        let first_directory_sector_location = u32::from_le_bytes(buffer);

        // Seeks to the root entry CLSID.
        let offset = if major_version == 0x0003 { 512 } else { 4096 }
            * (1 + first_directory_sector_location as u64)
            + 80;
        reader.seek(SeekFrom::Start(offset))?;

        // Reads and decodes the CLSID.
        let mut buffer = [0; 16];
        reader.read_exact(&mut buffer)?;
        let clsid: String = [3, 2, 1, 0, 5, 4, 7, 6, 8, 9, 10, 11, 12, 13, 14, 15]
            .iter()
            .map(|&index| {
                if index == 5 || index == 7 || index == 8 || index == 10 {
                    format!("-{:02x}", buffer[index])
                } else {
                    format!("{:02x}", buffer[index])
                }
            })
            .collect();

        // Determines the file format based on the CLSID.
        Ok(match clsid.as_str() {
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
            "00000000-0000-0000-0000-000000000000" => {
                // Creates and fills a buffer.
                let mut buffer = [0; 512];
                let bytes_read = reader.read(&mut buffer)?;

                // Searches for specific entry names in the buffer.
                if contains(&buffer[..bytes_read], XLR_ENTRY_NAME) {
                    Self::MicrosoftWorks6Spreadsheet
                } else if contains(&buffer[..bytes_read], WPS_ENTRY_NAME) {
                    Self::MicrosoftWorksWordProcessor
                } else {
                    Self::CompoundFileBinary
                }
            }
            _ => Self::CompoundFileBinary,
        })
    }

    /// Determines file format from an EBML reader.
    #[cfg(feature = "reader-ebml")]
    pub(crate) fn from_ebml_reader<R: Read + Seek>(reader: R) -> Result<Self> {
        // Maximum number of EBML elements that can be processed by the reader.
        const ELEMENT_LIMIT: usize = 256;

        // Maximum size of a string that can be handled by the reader.
        const STRING_LIMIT: usize = 64;

        // DocType element ID.
        const DOC_TYPE_ELEMENT_ID: u32 = 0x4282;

        // EBML element ID.
        const EBML_ELEMENT_ID: u32 = 0x1A45DFA3;

        // Cluster element ID.
        const CLUSTER_ELEMENT_ID: u32 = 0x1F43B675;

        // CodecID element ID.
        const CODEC_ID_ELEMENT_ID: u32 = 0x86;

        // Segment element ID.
        const SEGMENT_ELEMENT_ID: u32 = 0x18538067;

        // StereoMode element ID.
        const STEREO_MODE_ELEMENT_ID: u32 = 0x53B8;

        // Tracks element ID.
        const TRACKS_ELEMENT_ID: u32 = 0x1654AE6B;

        // TrackEntry element ID.
        const TRACK_ENTRY_ELEMENT_ID: u32 = 0xAE;

        // Video element ID.
        const VIDEO_ELEMENT_ID: u32 = 0xE0;

        // Helper function to read the ID of an EBML element.
        #[inline]
        fn read_id<R: Read>(reader: &mut R) -> Result<u32> {
            // Reads the first byte.
            let mut first_byte = [0];
            reader.read_exact(&mut first_byte)?;

            // Determines the number of bytes used to represent the ID.
            let num_bytes = first_byte[0].leading_zeros() + 1;
            if num_bytes > 4 {
                return Err(Error::new(ErrorKind::InvalidData, "invalid EBML ID"));
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
        #[inline]
        fn read_size<R: Read>(reader: &mut R) -> Result<u64> {
            // Reads the first byte.
            let mut first_byte = [0];
            reader.read_exact(&mut first_byte)?;

            // Determines the number of bytes used to represent the size.
            let num_bytes = first_byte[0].leading_zeros() + 1;
            if num_bytes > 8 {
                return Err(Error::new(ErrorKind::InvalidData, "invalid EBML size"));
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

        // Creates a buffered reader.
        let mut reader = BufReader::new(reader);

        // Gets the stream length.
        let length = reader.seek(SeekFrom::End(0))?;

        // Rewinds to the beginning of the stream.
        reader.rewind()?;

        // Flags indicating the presence of audio, video and subtitle tracks.
        let mut audio_track = false;
        let mut video_track = false;
        let mut subtitle_track = false;

        // Iterates through the EBML elements in the reader.
        let mut element_count = 0;
        while element_count < ELEMENT_LIMIT && reader.stream_position()? < length {
            // Reads the element ID.
            let id = read_id(&mut reader)?;

            // Reads the element size.
            let size = read_size(&mut reader)?;

            // Checks the element ID to perform specific actions.
            match id {
                EBML_ELEMENT_ID
                | SEGMENT_ELEMENT_ID
                | TRACKS_ELEMENT_ID
                | TRACK_ENTRY_ELEMENT_ID
                | VIDEO_ELEMENT_ID => {}
                DOC_TYPE_ELEMENT_ID => {
                    // Reads the DocType.
                    let mut buffer = vec![0; std::cmp::min(STRING_LIMIT, size as usize)];
                    reader.read_exact(&mut buffer)?;
                    let doc_type = String::from_utf8_lossy(&buffer)
                        .trim_end_matches('\0')
                        .to_string();

                    // Checks the DocType.
                    match doc_type.as_str() {
                        "webm" => return Ok(Self::Webm),
                        "matroska" => {}
                        _ => return Ok(Self::ExtensibleBinaryMetaLanguage),
                    }
                }
                CODEC_ID_ELEMENT_ID => {
                    // Reads the Codec ID.
                    let mut buffer = vec![0; std::cmp::min(STRING_LIMIT, size as usize)];
                    reader.read_exact(&mut buffer)?;
                    let codec_id = String::from_utf8_lossy(&buffer).to_string();

                    // Checks the Codec ID.
                    if codec_id.starts_with("A_") {
                        audio_track = true;
                    } else if codec_id.starts_with("V_") {
                        video_track = true;
                    } else if codec_id.starts_with("S_") {
                        subtitle_track = true;
                    }
                }
                STEREO_MODE_ELEMENT_ID => {
                    // Reads the StereoMode.
                    let mut buffer = [0];
                    reader.read_exact(&mut buffer)?;
                    let stereo_mode = buffer[0];

                    // Checks the StereoMode.
                    if stereo_mode > 0 {
                        return Ok(Self::Matroska3dVideo);
                    }
                }
                CLUSTER_ELEMENT_ID => {
                    // No need to continue reading.
                    break;
                }
                _ => {
                    // Seeks to the next element.
                    reader.seek(SeekFrom::Current(size as i64))?;
                }
            }

            // Increments the element count.
            element_count += 1;
        }

        // Determines the file format based on the identified tracks.
        Ok(if video_track {
            Self::MatroskaVideo
        } else if audio_track {
            Self::MatroskaAudio
        } else if subtitle_track {
            Self::MatroskaSubtitles
        } else {
            Self::ExtensibleBinaryMetaLanguage
        })
    }

    /// Determines file format from an EXE reader.
    #[cfg(feature = "reader-exe")]
    pub(crate) fn from_exe_reader<R: Read + Seek>(mut reader: R) -> Result<Self> {
        // Signature for the LE file format.
        const LE_SIGNATURE_1: &[u8] = b"LE";

        // Signature for the LE file format.
        const LE_SIGNATURE_2: &[u8] = b"LX";

        // Signature for the NE file format.
        const NE_SIGNATURE: &[u8] = b"NE";

        // Signature for the PE file format.
        const PE_SIGNATURE: &[u8] = b"PE\0\0";

        // Gets the stream length.
        let length = reader.seek(SeekFrom::End(0))?;

        // Reads the extended header address.
        reader.seek(SeekFrom::Start(60))?;
        let mut buffer = [0; 4];
        reader.read_exact(&mut buffer)?;
        let offset = u32::from_le_bytes(buffer);

        // Checks that the offset value is not outside the stream's boundaries.
        if offset as u64 + 4 < length {
            // Seeks to the offset.
            reader.seek(SeekFrom::Start(offset as u64))?;

            // Reads the signature.
            let mut buffer = [0; 4];
            reader.read_exact(&mut buffer)?;

            // Checks the signature.
            if buffer == PE_SIGNATURE {
                // Reads the characteristics.
                reader.seek(SeekFrom::Current(18))?;
                let mut buffer = [0; 2];
                reader.read_exact(&mut buffer)?;

                // Checks the characteristics
                return Ok(if u16::from_le_bytes(buffer) & 0x2000 == 0x2000 {
                    Self::DynamicLinkLibrary
                } else {
                    Self::PortableExecutable
                });
            } else if &buffer[..2] == LE_SIGNATURE_1 || &buffer[..2] == LE_SIGNATURE_2 {
                return Ok(Self::LinearExecutable);
            } else if &buffer[..2] == NE_SIGNATURE {
                return Ok(Self::NewExecutable);
            }
        }

        // Returns the default value.
        Ok(Self::MsDosExecutable)
    }

    /// Determines file format from a MP4 reader.
    #[cfg(feature = "reader-mp4")]
    pub(crate) fn from_mp4_reader<R: Read + Seek>(reader: R) -> Result<Self> {
        // Maximum number of boxes that the can be processed by the reader.
        const BOX_LIMIT: usize = 256;

        // Creates a buffered reader.
        let mut reader = BufReader::new(reader);

        // Gets the stream length.
        let length = reader.seek(SeekFrom::End(0))?;

        // Rewinds to the beginning of the stream.
        reader.rewind()?;

        // Flags indicating the presence of audio, video and subtitle tracks.
        let mut audio_track = false;
        let mut video_track = false;
        let mut subtitle_track = false;

        // Iterates through boxes in the reader.
        let mut box_count = 0;
        while box_count < BOX_LIMIT && reader.stream_position()? < length {
            // Reads the box size.
            let mut buffer = [0; 4];
            reader.read_exact(&mut buffer)?;
            let size = u32::from_be_bytes(buffer);

            // Reads the box type.
            let mut buffer = [0; 4];
            reader.read_exact(&mut buffer)?;

            // Checks the box type to perform specific actions.
            match &buffer {
                b"moov" | b"trak" | b"mdia" => {}
                b"hdlr" => {
                    // Reads the handler type.
                    reader.seek(SeekFrom::Current(8))?;
                    let mut buffer = [0; 4];
                    reader.read_exact(&mut buffer)?;

                    // Checks the handler type.
                    match &buffer {
                        b"vide" => video_track = true,
                        b"soun" => audio_track = true,
                        b"sbtl" | b"subt" | b"text" => subtitle_track = true,
                        _ => {}
                    }

                    // Seeks to the next box.
                    reader.seek(SeekFrom::Current(size as i64 - 20))?;
                }
                _ => {
                    // Seeks to the next box.
                    reader.seek(SeekFrom::Current(size as i64 - 8))?;
                }
            }

            // Increments the box count.
            box_count += 1;
        }

        // Determines the file format based on the identified tracks.
        Ok(if video_track {
            Self::Mpeg4Part14Video
        } else if audio_track {
            Self::Mpeg4Part14Audio
        } else if subtitle_track {
            Self::Mpeg4Part14Subtitles
        } else {
            Self::Mpeg4Part14
        })
    }

    /// Determines file format from a PDF reader.
    #[cfg(feature = "reader-pdf")]
    pub(crate) fn from_pdf_reader<R: Read + Seek>(mut reader: R) -> Result<Self> {
        // Maximum number of bytes that can be read by the reader (16 MB).
        const READ_LIMIT: usize = 16_777_216;

        // Size of each chunk to read (32 KB).
        const CHUNK_SIZE: usize = 32_768;

        // Size of overlap to keep between chunks.
        const OVERLAP_SIZE: usize = AI_MARKER.len() - 1;

        // Marker for the AI file format.
        const AI_MARKER: &[u8] = b"AIPrivateData";

        // Rewinds to the beginning of the stream.
        reader.rewind()?;

        // Creates a buffer to hold the chunk of data being read.
        let mut buffer = [0; OVERLAP_SIZE + CHUNK_SIZE];

        // Reads the data from the stream in chunks.
        let mut total_bytes_read = 0;
        while total_bytes_read < READ_LIMIT {
            // Reads a chunk of the stream into the buffer.
            let bytes_read = reader.read(&mut buffer[OVERLAP_SIZE..])?;
            if bytes_read == 0 {
                break;
            }

            // Determines the start index for searching the buffer.
            let start = if total_bytes_read == 0 {
                OVERLAP_SIZE
            } else {
                0
            };

            // Checks if the buffer contains the AI file format marker.
            if contains(&buffer[start..OVERLAP_SIZE + bytes_read], AI_MARKER) {
                return Ok(Self::AdobeIllustratorArtwork);
            }

            // Rotates the buffer to the right by the overlap size.
            buffer[..OVERLAP_SIZE + bytes_read].rotate_right(OVERLAP_SIZE);

            // Updates the total bytes read.
            total_bytes_read += bytes_read;
        }

        // Returns the default value.
        Ok(Self::PortableDocumentFormat)
    }

    /// Determines file format from a RM reader.
    #[cfg(feature = "reader-rm")]
    pub(crate) fn from_rm_reader<R: Read + Seek>(mut reader: R) -> Result<Self> {
        // Media type for the RV file format.
        const RV_MEDIA_TYPE: &[u8] = b"video/x-pn-realvideo";

        // Media type for the RA file format.
        const RA_MEDIA_TYPE_1: &[u8] = b"audio/x-pn-realaudio";

        // Media type for the RA file format.
        const RA_MEDIA_TYPE_2: &[u8] = b"audio/x-pn-multirate-realaudio";

        // Rewinds to the beginning of the stream plus the size of the RM file format signature.
        reader.seek(SeekFrom::Start(8))?;

        // Creates and fills a buffer.
        let mut buffer = [0; 4096];
        let bytes_read = reader.read(&mut buffer)?;

        // Searches for specific media types in the buffer.
        Ok(if contains(&buffer[..bytes_read], RV_MEDIA_TYPE) {
            Self::Realvideo
        } else if contains(&buffer[..bytes_read], RA_MEDIA_TYPE_1)
            || contains(&buffer[..bytes_read], RA_MEDIA_TYPE_2)
        {
            Self::Realaudio
        } else {
            Self::Realmedia
        })
    }

    /// Determines file format from a TXT reader.
    #[cfg(feature = "reader-txt")]
    pub(crate) fn from_txt_reader<R: Read + Seek>(reader: R) -> Result<Self> {
        // Maximum number of lines that the can be processed by the reader.
        const LINE_LIMIT: usize = 8;

        // Maximum number of bytes that can be read by the reader (32 KB).
        const READ_LIMIT: u64 = 32_768;

        // Creates a buffered reader.
        let mut reader = BufReader::new(reader);

        // Rewinds to the beginning of the stream.
        reader.rewind()?;

        // Determines if the reader contains only ASCII/UTF-8-encoded text by checking the first
        // lines for control characters other than whitespaces.
        reader
            .take(READ_LIMIT)
            .lines()
            .take(LINE_LIMIT)
            .try_for_each(|line| {
                line?
                    .chars()
                    .find(|char| char.is_control() && !char.is_whitespace())
                    .map(|_| Err(Error::new(ErrorKind::InvalidData, "invalid characters")))
                    .unwrap_or(Ok(()))
            })
            .map(|_| Self::PlainText)
    }

    /// Determines file format from a XML reader.
    #[cfg(feature = "reader-xml")]
    pub(crate) fn from_xml_reader<R: Read + Seek>(reader: R) -> Result<Self> {
        // Maximum number of lines that the can be processed by the reader.
        const LINE_LIMIT: usize = 8;

        // Maximum number of bytes that can be read by the reader (32 KB).
        const READ_LIMIT: u64 = 32_768;

        // Creates a buffered reader.
        let mut reader = BufReader::new(reader);

        // Rewinds to the beginning of the stream.
        reader.rewind()?;

        // Searches the reader for lines indicating the presence of various file formats.
        for result in reader.take(READ_LIMIT).lines().take(LINE_LIMIT) {
            let line = result?;
            if line.contains("<abiword template=\"false\"") {
                return Ok(Self::Abiword);
            } else if line.contains("<abiword template=\"true\"") {
                return Ok(Self::AbiwordTemplate);
            } else if line.contains("<amf") {
                return Ok(Self::AdditiveManufacturingFormat);
            } else if line.contains("<ASX") || line.contains("<asx") {
                return Ok(Self::AdvancedStreamRedirector);
            } else if line.contains("<feed") {
                return Ok(Self::Atom);
            } else if line.contains("<COLLADA") || line.contains("<collada") {
                return Ok(Self::DigitalAssetExchange);
            } else if line.contains("<mxfile") {
                return Ok(Self::Drawio);
            } else if line.contains("<X3D") || line.contains("<x3d") {
                return Ok(Self::Extensible3d);
            } else if line.contains("<xsl") {
                return Ok(Self::ExtensibleStylesheetLanguageTransformations);
            } else if line.contains("<FictionBook") {
                return Ok(Self::Fictionbook);
            } else if line.contains("<gml") {
                return Ok(Self::GeographyMarkupLanguage);
            } else if line.contains("<gpx") {
                return Ok(Self::GpsExchangeFormat);
            } else if line.contains("<kml") {
                return Ok(Self::KeyholeMarkupLanguage);
            } else if line.contains("<math") {
                return Ok(Self::MathematicalMarkupLanguage);
            } else if line.contains("<MPD") {
                return Ok(Self::MpegDashManifest);
            } else if line.contains("<score-partwise") {
                return Ok(Self::Musicxml);
            } else if line.contains("<rss") {
                return Ok(Self::ReallySimpleSyndication);
            } else if line.contains("<SVG") || line.contains("<svg") {
                return Ok(Self::ScalableVectorGraphics);
            } else if line.contains("<soap") {
                return Ok(Self::SimpleObjectAccessProtocol);
            } else if line.contains("<map") {
                return Ok(Self::TiledMapXml);
            } else if line.contains("<tileset") {
                return Ok(Self::TiledTilesetXml);
            } else if line.contains("<tt") && line.contains("xmlns=\"http://www.w3.org/ns/ttml\"") {
                return Ok(Self::TimedTextMarkupLanguage);
            } else if line.contains("<TrainingCenterDatabase") {
                return Ok(Self::TrainingCenterXml);
            } else if line.contains("<USFSubtitles") {
                return Ok(Self::UniversalSubtitleFormat);
            } else if line.contains("<xliff") {
                return Ok(Self::XmlLocalizationInterchangeFileFormat);
            } else if line.contains("<playlist") {
                return Ok(Self::XmlShareablePlaylistFormat);
            }
        }

        // Returns the default value.
        Ok(Self::ExtensibleMarkupLanguage)
    }

    /// Determines file format from a ZIP reader.
    #[cfg(feature = "reader-zip")]
    pub(crate) fn from_zip_reader<R: Read + Seek>(reader: R) -> Result<Self> {
        // Maximum number of files that can be processed by the reader.
        const FILE_LIMIT: usize = 1024;

        // Signature of the central directory file header.
        const CENTRAL_DIRECTORY_FILE_HEADER_SIGNATURE: &[u8] = b"\x50\x4B\x01\x02";

        // Signature of the end of central directory.
        const END_OF_CENTRAL_DIRECTORY_SIGNATURE: &[u8] = b"\x50\x4B\x05\x06";

        // Maximum size of the end of central directory.
        const END_OF_CENTRAL_DIRECTORY_MAX_SIZE: usize = 22 + u16::MAX as usize;

        // Creates a buffered reader.
        let mut reader = BufReader::new(reader);

        // Gets the stream length.
        let length = reader.seek(SeekFrom::End(0))?;

        // Searches for the end of central directory.
        let offset = length.saturating_sub(END_OF_CENTRAL_DIRECTORY_MAX_SIZE as u64);
        reader.seek(SeekFrom::Start(offset))?;
        let mut buffer = vec![0; std::cmp::min(END_OF_CENTRAL_DIRECTORY_MAX_SIZE, length as usize)];
        reader.read_exact(&mut buffer)?;
        let buffer_index = find(&buffer, END_OF_CENTRAL_DIRECTORY_SIGNATURE)
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "cannot find the EOCD"))?;

        // Seeks to the end of central directory.
        reader.seek(SeekFrom::Start(offset + buffer_index as u64))?;

        // Reads the start of central directory offset.
        reader.seek(SeekFrom::Current(16))?;
        let mut buffer = [0; 4];
        reader.read_exact(&mut buffer)?;
        let offset = u32::from_le_bytes(buffer);

        // Seeks to the start of central directory.
        reader.seek(SeekFrom::Start(offset as u64))?;

        // Sets the default value.
        let mut format = Self::Zip;

        // Browses central directory file headers.
        let mut file_count = 0;
        while file_count < FILE_LIMIT && reader.stream_position()? < length {
            // Reads and checks the signature.
            let mut buffer = [0; 4];
            reader.read_exact(&mut buffer)?;
            if buffer != CENTRAL_DIRECTORY_FILE_HEADER_SIGNATURE {
                break;
            }

            // Reads the compressed size.
            reader.seek(SeekFrom::Current(16))?;
            let mut buffer = [0; 4];
            reader.read_exact(&mut buffer)?;
            let compressed_size = u32::from_le_bytes(buffer);

            // Reads the uncompressed size.
            let mut buffer = [0; 4];
            reader.read_exact(&mut buffer)?;
            let uncompressed_size = u32::from_le_bytes(buffer);

            // Reads the filename length.
            let mut buffer = [0; 2];
            reader.read_exact(&mut buffer)?;
            let filename_length = u16::from_le_bytes(buffer);

            // Reads the extra field length.
            let mut buffer = [0; 2];
            reader.read_exact(&mut buffer)?;
            let extra_field_length = u16::from_le_bytes(buffer);

            // Reads the file comment length.
            let mut buffer = [0; 2];
            reader.read_exact(&mut buffer)?;
            let file_comment_length = u16::from_le_bytes(buffer);

            // Reads the relative offset of local file header.
            reader.seek(SeekFrom::Current(8))?;
            let mut buffer = [0; 4];
            reader.read_exact(&mut buffer)?;
            let offset = u32::from_le_bytes(buffer);

            // Reads the filename.
            let mut buffer = vec![0; filename_length as usize];
            reader.read_exact(&mut buffer)?;
            let filename = String::from_utf8_lossy(&buffer).to_string();

            // Checks the filename.
            match filename.as_str() {
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
                "mimetype" if compressed_size == uncompressed_size => {
                    // Seeks to the filename of the local file header.
                    reader.seek(SeekFrom::Start(offset as u64 + 26))?;

                    // Reads the filename length.
                    let mut buffer = [0; 2];
                    reader.read_exact(&mut buffer)?;
                    let filename_length = u16::from_le_bytes(buffer);

                    // Reads the extra field length.
                    let mut buffer = [0; 2];
                    reader.read_exact(&mut buffer)?;
                    let extra_field_length = u16::from_le_bytes(buffer);

                    // Seeks to the data.
                    reader.seek(SeekFrom::Current(
                        filename_length as i64 + extra_field_length as i64,
                    ))?;

                    // Reads the data.
                    let mut buffer = vec![0; compressed_size as usize];
                    reader.read_exact(&mut buffer)?;
                    let data = String::from_utf8_lossy(&buffer).to_string();

                    // Checks the trimmed data.
                    return Ok(match data.trim() {
                        "application/epub+zip" => Self::ElectronicPublication,
                        "application/vnd.adobe.indesign-idml-package" => {
                            Self::IndesignMarkupLanguage
                        }
                        "application/vnd.oasis.opendocument.base"
                        | "application/vnd.oasis.opendocument.database" => {
                            Self::OpendocumentDatabase
                        }
                        "application/vnd.oasis.opendocument.formula" => Self::OpendocumentFormula,
                        "application/vnd.oasis.opendocument.formula-template" => {
                            Self::OpendocumentFormulaTemplate
                        }
                        "application/vnd.oasis.opendocument.graphics" => Self::OpendocumentGraphics,
                        "application/vnd.oasis.opendocument.graphics-template" => {
                            Self::OpendocumentGraphicsTemplate
                        }
                        "application/vnd.oasis.opendocument.presentation" => {
                            Self::OpendocumentPresentation
                        }
                        "application/vnd.oasis.opendocument.presentation-template" => {
                            Self::OpendocumentPresentationTemplate
                        }
                        "application/vnd.oasis.opendocument.spreadsheet" => {
                            Self::OpendocumentSpreadsheet
                        }
                        "application/vnd.oasis.opendocument.spreadsheet-template" => {
                            Self::OpendocumentSpreadsheetTemplate
                        }
                        "application/vnd.oasis.opendocument.text" => Self::OpendocumentText,
                        "application/vnd.oasis.opendocument.text-master" => {
                            Self::OpendocumentTextMaster
                        }
                        "application/vnd.oasis.opendocument.text-master-template" => {
                            Self::OpendocumentTextMasterTemplate
                        }
                        "application/vnd.oasis.opendocument.text-template" => {
                            Self::OpendocumentTextTemplate
                        }
                        "application/vnd.recordare.musicxml" => Self::MusicxmlZipped,
                        "application/vnd.sun.xml.calc" => Self::SunXmlCalc,
                        "application/vnd.sun.xml.calc.template" => Self::SunXmlCalcTemplate,
                        "application/vnd.sun.xml.draw" => Self::SunXmlDraw,
                        "application/vnd.sun.xml.draw.template" => Self::SunXmlDrawTemplate,
                        "application/vnd.sun.xml.impress" => Self::SunXmlImpress,
                        "application/vnd.sun.xml.impress.template" => Self::SunXmlImpressTemplate,
                        "application/vnd.sun.xml.math" => Self::SunXmlMath,
                        "application/vnd.sun.xml.writer" => Self::SunXmlWriter,
                        "application/vnd.sun.xml.writer.global" => Self::SunXmlWriterGlobal,
                        "application/vnd.sun.xml.writer.template" => Self::SunXmlWriterTemplate,
                        "image/openraster" => Self::Openraster,
                        _ => Self::Zip,
                    });
                }
                _ => {
                    if filename.starts_with("Fusion[Active]/") {
                        return Ok(Self::Autodesk123d);
                    } else if filename.starts_with("circuitdiagram/") {
                        return Ok(Self::CircuitDiagramDocument);
                    } else if filename.starts_with("dwf/") {
                        return Ok(Self::DesignWebFormatXps);
                    } else if filename.ends_with(".fb2") && !filename.contains('/') {
                        return Ok(Self::FictionbookZipped);
                    } else if filename.starts_with("FusionAssetName[Active]/") {
                        return Ok(Self::Fusion360);
                    } else if filename.starts_with("Payload/") && filename.contains(".app/") {
                        return Ok(Self::IosAppStorePackage);
                    } else if filename.starts_with("word/") {
                        return Ok(Self::OfficeOpenXmlDocument);
                    } else if filename.starts_with("visio/") {
                        return Ok(Self::OfficeOpenXmlDrawing);
                    } else if filename.starts_with("ppt/") {
                        return Ok(Self::OfficeOpenXmlPresentation);
                    } else if filename.starts_with("xl/") {
                        return Ok(Self::OfficeOpenXmlSpreadsheet);
                    } else if filename.starts_with("SpaceClaim/") {
                        return Ok(Self::SpaceclaimDocument);
                    } else if filename.starts_with("3D/") && filename.ends_with(".model") {
                        return Ok(Self::ThreeDimensionalManufacturingFormat);
                    } else if (filename.ends_with(".usd")
                        || filename.ends_with(".usda")
                        || filename.ends_with(".usdc"))
                        && !filename.contains('/')
                    {
                        return Ok(Self::UniversalSceneDescriptionZipped);
                    }
                }
            }

            // Seeks to the next central directory file header.
            reader.seek(SeekFrom::Current(
                extra_field_length as i64 + file_comment_length as i64,
            ))?;

            // Increments the file count.
            file_count += 1;
        }
        Ok(format)
    }
}

/// Checks if the `data` slice contains the `target` sequence.
#[cfg(any(
    feature = "reader-asf",
    feature = "reader-cfb",
    feature = "reader-pdf",
    feature = "reader-rm",
))]
#[inline]
fn contains(data: &[u8], target: &[u8]) -> bool {
    find(data, target).is_some()
}

/// Searches for the `target` byte sequence in the `data` slice using the Boyer-Moore algorithm.
///
/// If the sequence is found, the function returns the index of the first occurrence.
#[cfg(any(
    feature = "reader-asf",
    feature = "reader-cfb",
    feature = "reader-pdf",
    feature = "reader-rm",
    feature = "reader-zip",
))]
fn find(data: &[u8], target: &[u8]) -> Option<usize> {
    // An empty target sequence is always considered to be contained in the data.
    if target.is_empty() {
        return Some(0);
    }

    // The data array is shorter than the target sequence, so it cannot contain the target.
    if data.len() < target.len() {
        return None;
    }

    // Builds the bad character shift table.
    let mut bad_char_table = [0; 256];
    for (index, &char) in target.iter().enumerate().take(target.len() - 1) {
        bad_char_table[char as usize] = target.len() - 1 - index;
    }

    // Starts searching from the last possible position in the data array.
    let mut position = target.len() - 1;
    while position < data.len() {
        let mut target_index = target.len() - 1;
        let mut data_index = position;
        while data[data_index] == target[target_index] {
            if target_index == 0 {
                return Some(data_index);
            }
            target_index -= 1;
            data_index -= 1;
        }

        // Calculates the maximum shift based on the bad character rule and good suffix rule.
        let bad_char_shift = bad_char_table[data[position] as usize];
        let good_suffix_shift = target.len() - target_index;
        let shift = std::cmp::max(bad_char_shift, good_suffix_shift);
        position += shift;
    }
    None
}
