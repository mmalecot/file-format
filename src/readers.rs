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
    pub(crate) fn from_asf_reader<R: Read + Seek>(reader: R) -> Result<Self> {
        // Maximum number of descriptors that can be processed by the reader.
        const DESCRIPTOR_LIMIT: usize = 32;

        // Maximum size of a descriptor name that can be handled by the reader.
        const DESCRIPTOR_NAME_LIMIT: usize = 64;

        // Maximum number of objects that can be processed by the reader.
        const OBJECT_LIMIT: usize = 256;

        // UTF-16-encoded descriptor name for DVR-MS file format.
        const DVR_MS_DESCRIPTOR_NAME: &[u8] =
            b"D\0V\0R\0 \0F\0i\0l\0e\0 \0V\0e\0r\0s\0i\0o\0n\0\0\0";

        // Binary-encoded GUID for extended content description object
        // (d2d0a440-e307-11d2-97f0-00a0c95ea850).
        const EXTENDED_CONTENT_DESCRIPTION_OBJECT_GUID: &[u8; 16] =
            b"\x40\xA4\xD0\xD2\x07\xE3\xD2\x11\x97\xF0\x00\xA0\xC9\x5E\xA8\x50";

        // Binary-encoded GUID for stream properties object (b7dc0791-a9b7-11cf-8ee6-00c00c205365).
        const STREAM_PROPERTIES_OBJECT_GUID: &[u8; 16] =
            b"\x91\x07\xDC\xB7\xB7\xA9\xCF\x11\x8E\xE6\x00\xC0\x0C\x20\x53\x65";

        // Binary-encoded GUID for audio media (f8699e40-5b4d-11cf-a8fd-00805f5c442b).
        const AUDIO_MEDIA_GUID: &[u8; 16] =
            b"\x40\x9E\x69\xF8\x4D\x5B\xCF\x11\xA8\xFD\x00\x80\x5F\x5C\x44\x2B";

        // Binary-encoded GUID for video media (bc19efc0-5b4d-11cf-a8fd-00805f5c442b).
        const VIDEO_MEDIA_GUID: &[u8; 16] =
            b"\xC0\xEF\x19\xBC\x4D\x5B\xCF\x11\xA8\xFD\x00\x80\x5F\x5C\x44\x2B";

        // Creates a buffered reader.
        let mut reader = BufReader::new(reader);

        // Reads the number of header objects.
        reader.seek(SeekFrom::Start(24))?;
        let mut number_of_header_objects = [0; 4];
        reader.read_exact(&mut number_of_header_objects)?;
        let number_of_header_objects = u32::from_le_bytes(number_of_header_objects);

        // Skips the reserved fields.
        reader.seek(SeekFrom::Current(2))?;

        // Flags indicating the presence of audio and video streams.
        let mut audio_stream = false;
        let mut video_stream = false;

        // Iterates through the header objects.
        for _ in 0..std::cmp::min(OBJECT_LIMIT, number_of_header_objects as usize) {
            // Reads the object GUID.
            let mut guid = [0; 16];
            reader.read_exact(&mut guid)?;

            // Reads the object size.
            let mut size = [0; 8];
            reader.read_exact(&mut size)?;
            let size = u64::from_le_bytes(size);

            // Checks the object GUID.
            match &guid {
                STREAM_PROPERTIES_OBJECT_GUID => {
                    // Reads the stream type.
                    let mut stream_type = [0; 16];
                    reader.read_exact(&mut stream_type)?;

                    // Checks the stream type.
                    match &stream_type {
                        AUDIO_MEDIA_GUID => audio_stream = true,
                        VIDEO_MEDIA_GUID => video_stream = true,
                        _ => {}
                    }

                    // Seeks to the next object.
                    reader.seek(SeekFrom::Current(size as i64 - 40))?;
                }
                EXTENDED_CONTENT_DESCRIPTION_OBJECT_GUID => {
                    // Reads the content descriptors count.
                    let mut count = [0; 2];
                    reader.read_exact(&mut count)?;
                    let count = u16::from_le_bytes(count);

                    // Calculates the offset of the content descriptors.
                    let offset = reader.stream_position()?;

                    // Iterates through the content descriptors.
                    for _ in 0..std::cmp::min(DESCRIPTOR_LIMIT, count as usize) {
                        // Reads the descriptor name length.
                        let mut length = [0; 2];
                        reader.read_exact(&mut length)?;
                        let length = u16::from_le_bytes(length);

                        // Reads the descriptor name.
                        let mut name =
                            vec![0; std::cmp::min(DESCRIPTOR_NAME_LIMIT, length as usize)];
                        reader.read_exact(&mut name)?;

                        // Checks the descriptor name.
                        if name == DVR_MS_DESCRIPTOR_NAME {
                            return Ok(Self::MicrosoftDigitalVideoRecording);
                        }

                        // Calculates the remaining length.
                        let remaining_length = length.saturating_sub(DESCRIPTOR_NAME_LIMIT as u16);

                        // Reads the descriptor value length.
                        reader.seek(SeekFrom::Current(remaining_length as i64 + 2))?;
                        let mut length = [0; 2];
                        reader.read_exact(&mut length)?;
                        let length = u16::from_le_bytes(length);

                        // Seeks to the next content descriptor.
                        reader.seek(SeekFrom::Current(length as i64))?;
                    }

                    // Seeks to the next object.
                    reader.seek(SeekFrom::Start(offset + size - 26))?;
                }
                _ => {
                    // Seeks to the next object.
                    reader.seek(SeekFrom::Current(size as i64 - 24))?;
                }
            }
        }

        // Determines the file format based on the identified streams.
        Ok(if video_stream {
            Self::WindowsMediaVideo
        } else if audio_stream {
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
        let mut version = [0; 2];
        reader.read_exact(&mut version)?;
        let version = u16::from_le_bytes(version);

        // Calculates the directory sector size based on the major version.
        let directory_sector_size = if version == 0x0003 { 512 } else { 4096 };

        // Reads the first directory sector location.
        reader.seek(SeekFrom::Current(20))?;
        let mut first_directory_sector_location = [0; 4];
        reader.read_exact(&mut first_directory_sector_location)?;
        let first_directory_sector_location = u32::from_le_bytes(first_directory_sector_location);

        // Seeks to the root entry CLSID.
        let offset = directory_sector_size * (1 + first_directory_sector_location as u64) + 80;
        reader.seek(SeekFrom::Start(offset))?;

        // Reads the CLSID.
        let mut clsid = [0; 16];
        reader.read_exact(&mut clsid)?;
        let clsid: String = [3, 2, 1, 0, 5, 4, 7, 6, 8, 9, 10, 11, 12, 13, 14, 15]
            .iter()
            .map(|&index| {
                if index == 5 || index == 7 || index == 8 || index == 10 {
                    format!("-{:02x}", clsid[index])
                } else {
                    format!("{:02x}", clsid[index])
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
            _ => {
                // Reads the second directory entry name.
                reader.seek(SeekFrom::Current(32))?;
                let mut second_directory_entry_name = [0; 64];
                reader.read_exact(&mut second_directory_entry_name)?;

                // Checks the second directory entry name.
                if second_directory_entry_name.starts_with(WPS_ENTRY_NAME) {
                    return Ok(Self::MicrosoftWorksWordProcessor);
                }

                // Reads the third directory entry name.
                reader.seek(SeekFrom::Current(64))?;
                let mut third_directory_entry_name = [0; 64];
                reader.read_exact(&mut third_directory_entry_name)?;

                // Checks the third directory entry name.
                if third_directory_entry_name.starts_with(XLR_ENTRY_NAME) {
                    return Ok(Self::MicrosoftWorks6Spreadsheet);
                }

                // Returns the default value.
                Self::CompoundFileBinary
            }
        })
    }

    /// Determines file format from an EBML reader.
    #[cfg(feature = "reader-ebml")]
    pub(crate) fn from_ebml_reader<R: Read + Seek>(reader: R) -> Result<Self> {
        // Maximum number of EBML elements that can be processed by the reader.
        const ELEMENT_LIMIT: usize = 256;

        // Maximum size of a Codec ID that can be processed by the reader.
        const CODEC_ID_LIMIT: usize = 64;

        // Maximum size of a DocType that can be processed by the reader.
        const DOC_TYPE_LIMIT: usize = 8;

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

        // Iterates through the EBML elements.
        let mut element_count = 0;
        while element_count < ELEMENT_LIMIT && reader.stream_position()? < length {
            // Reads the first byte of the element ID.
            let mut first_byte = [0];
            reader.read_exact(&mut first_byte)?;

            // Determines the number of bytes used to represent the element ID.
            let num_bytes = first_byte[0].leading_zeros() + 1;
            if num_bytes > 4 {
                return Err(Error::new(ErrorKind::InvalidData, "invalid EBML ID"));
            }

            // Calculates the element ID value based on the number of bytes.
            let mut id = first_byte[0] as u32;
            for _ in 1..num_bytes {
                let mut byte = [0];
                reader.read_exact(&mut byte)?;
                id = (id << 8) | (byte[0] as u32);
            }

            // Reads the first byte of the element size.
            let mut first_byte = [0];
            reader.read_exact(&mut first_byte)?;

            // Determines the number of bytes used to represent the element size.
            let num_bytes = first_byte[0].leading_zeros() + 1;
            if num_bytes > 8 {
                return Err(Error::new(ErrorKind::InvalidData, "invalid EBML size"));
            }

            // Calculates the element size value based on the number of bytes.
            let mut size = u64::from(first_byte[0] & ((128 >> first_byte[0].leading_zeros()) - 1));
            for _ in 1..num_bytes {
                let mut byte = [0];
                reader.read_exact(&mut byte)?;
                size = (size << 8) | (byte[0] as u64);
            }

            // Checks the element ID.
            match id {
                EBML_ELEMENT_ID
                | SEGMENT_ELEMENT_ID
                | TRACKS_ELEMENT_ID
                | TRACK_ENTRY_ELEMENT_ID
                | VIDEO_ELEMENT_ID => {}
                DOC_TYPE_ELEMENT_ID => {
                    // Reads the DocType.
                    let mut doc_type = vec![0; std::cmp::min(DOC_TYPE_LIMIT, size as usize)];
                    reader.read_exact(&mut doc_type)?;

                    // Checks the DocType.
                    if doc_type.starts_with(b"webm") {
                        return Ok(Self::Webm);
                    } else if !doc_type.starts_with(b"matroska") {
                        return Ok(Self::ExtensibleBinaryMetaLanguage);
                    }

                    // Skips the remaining size.
                    let remaining_size = size.saturating_sub(DOC_TYPE_LIMIT as u64);
                    reader.seek(SeekFrom::Current(remaining_size as i64))?;
                }
                CODEC_ID_ELEMENT_ID => {
                    // Reads the Codec ID.
                    let mut codec_id = vec![0; std::cmp::min(CODEC_ID_LIMIT, size as usize)];
                    reader.read_exact(&mut codec_id)?;

                    // Checks the Codec ID.
                    if codec_id.starts_with(b"A_") {
                        audio_track = true;
                    } else if codec_id.starts_with(b"V_") {
                        video_track = true;
                    } else if codec_id.starts_with(b"S_") {
                        subtitle_track = true;
                    }

                    // Skips the remaining size.
                    let remaining_size = size.saturating_sub(CODEC_ID_LIMIT as u64);
                    reader.seek(SeekFrom::Current(remaining_size as i64))?;
                }
                STEREO_MODE_ELEMENT_ID => {
                    // Reads the StereoMode.
                    let mut stereo_mode = [0];
                    reader.read_exact(&mut stereo_mode)?;

                    // Checks the StereoMode.
                    if stereo_mode[0] > 0 {
                        return Ok(Self::Matroska3dVideo);
                    }
                }
                CLUSTER_ELEMENT_ID => break,
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
        // Gets the stream length.
        let length = reader.seek(SeekFrom::End(0))?;

        // Reads the extended header address.
        reader.seek(SeekFrom::Start(60))?;
        let mut offset = [0; 4];
        reader.read_exact(&mut offset)?;
        let offset = u32::from_le_bytes(offset);

        // Checks that the offset value is not outside the stream's boundaries.
        if offset as u64 + 4 < length {
            // Seeks to the offset.
            reader.seek(SeekFrom::Start(offset as u64))?;

            // Reads the signature.
            let mut signature = [0; 4];
            reader.read_exact(&mut signature)?;

            // Checks the signature.
            if &signature == b"PE\0\0" {
                // Reads the characteristics.
                reader.seek(SeekFrom::Current(18))?;
                let mut characteristics = [0; 2];
                reader.read_exact(&mut characteristics)?;

                // Checks the characteristics
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

        // Iterates through boxes.
        let mut box_count = 0;
        while box_count < BOX_LIMIT && reader.stream_position()? < length {
            // Reads the box size.
            let mut size = [0; 4];
            reader.read_exact(&mut size)?;
            let size = u32::from_be_bytes(size);

            // Reads the box type.
            let mut box_type = [0; 4];
            reader.read_exact(&mut box_type)?;

            // Checks the box type.
            match &box_type {
                b"moov" | b"trak" | b"mdia" => {}
                b"hdlr" => {
                    // Reads the handler type.
                    reader.seek(SeekFrom::Current(8))?;
                    let mut handler_type = [0; 4];
                    reader.read_exact(&mut handler_type)?;

                    // Checks the handler type.
                    match &handler_type {
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
        // Maximum number of bytes that can be processed by the reader (32 MB).
        const READ_LIMIT: usize = 33_554_432;

        // Size of each chunk to read (32 KB).
        const CHUNK_SIZE: usize = 32_768;

        // Size of overlap to keep between chunks.
        const OVERLAP_SIZE: usize = AI_MARKER.len() - 1;

        // Marker for the AI file format.
        const AI_MARKER: &[u8] = b"AIPrivateData";

        // Rewinds to the beginning of the stream plus the size of the PDF file format signature.
        reader.seek(SeekFrom::Start(5))?;

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
            if find(&buffer[start..OVERLAP_SIZE + bytes_read], AI_MARKER).is_some() {
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
    pub(crate) fn from_rm_reader<R: Read + Seek>(reader: R) -> Result<Self> {
        // Maximum number of chunks that can be processed by the reader.
        const CHUNK_LIMIT: usize = 64;

        // Creates a buffered reader.
        let mut reader = BufReader::new(reader);

        // Reads the number of headers.
        reader.seek(SeekFrom::Start(14))?;
        let mut number_of_headers = [0; 4];
        reader.read_exact(&mut number_of_headers)?;
        let number_of_headers = u32::from_be_bytes(number_of_headers);

        // Flags indicating the presence of audio and video streams.
        let mut audio_stream = false;
        let mut video_stream = false;

        // Iterates through the chunks.
        for _ in 0..std::cmp::min(CHUNK_LIMIT, number_of_headers.saturating_sub(1) as usize) {
            // Reads the chunk type.
            let mut chunk_type = [0; 4];
            reader.read_exact(&mut chunk_type)?;

            // Reads the chunk size.
            let mut chunk_size = [0; 4];
            reader.read_exact(&mut chunk_size)?;
            let chunk_size = u32::from_be_bytes(chunk_size);

            // Checks the chunk type.
            if &chunk_type == b"MDPR" {
                // Calculates the offset of the media properties.
                let offset = reader.stream_position()?;

                // Reads the size of the stream name.
                reader.seek(SeekFrom::Current(32))?;
                let mut stream_name_size = [0; 1];
                reader.read_exact(&mut stream_name_size)?;

                // Skips the stream name.
                reader.seek(SeekFrom::Current(stream_name_size[0] as i64))?;

                // Reads the size of stream mime type.
                let mut mime_type_size = [0; 1];
                reader.read_exact(&mut mime_type_size)?;

                // Reads the mime type.
                let mut mime_type = vec![0; mime_type_size[0] as usize];
                reader.read_exact(&mut mime_type)?;

                // Checks the mime type.
                if mime_type.starts_with(b"audio/") {
                    audio_stream = true;
                } else if mime_type.starts_with(b"video/") {
                    video_stream = true;
                }

                // Rewinds to the offset of the media properties.
                reader.seek(SeekFrom::Start(offset))?;
            }

            // Seeks to the next chunk.
            reader.seek(SeekFrom::Current(chunk_size as i64 - 8))?;
        }

        // Determines the file format based on the identified streams.
        Ok(if video_stream {
            Self::Realvideo
        } else if audio_stream {
            Self::Realaudio
        } else {
            Self::Realmedia
        })
    }

    /// Determines file format from a TXT reader.
    #[cfg(feature = "reader-txt")]
    pub(crate) fn from_txt_reader<R: Read + Seek>(reader: R) -> Result<Self> {
        // Maximum number of lines that the can be processed by the reader.
        const LINE_LIMIT: usize = 16;

        // Maximum number of bytes that can be processed by the reader (64 KB).
        const READ_LIMIT: u64 = 65_536;

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

        // Maximum number of bytes that can be processed by the reader (32 KB).
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
        let mut offset = [0; 4];
        reader.read_exact(&mut offset)?;
        let offset = u32::from_le_bytes(offset);

        // Seeks to the start of central directory.
        reader.seek(SeekFrom::Start(offset as u64))?;

        // Sets the default value.
        let mut format = Self::Zip;

        // Browses central directory file headers.
        let mut file_count = 0;
        while file_count < FILE_LIMIT && reader.stream_position()? < length {
            // Reads and checks the signature.
            let mut signature = [0; 4];
            reader.read_exact(&mut signature)?;
            if signature != CENTRAL_DIRECTORY_FILE_HEADER_SIGNATURE {
                break;
            }

            // Reads the compressed size.
            reader.seek(SeekFrom::Current(16))?;
            let mut compressed_size = [0; 4];
            reader.read_exact(&mut compressed_size)?;
            let compressed_size = u32::from_le_bytes(compressed_size);

            // Reads the uncompressed size.
            let mut uncompressed_size = [0; 4];
            reader.read_exact(&mut uncompressed_size)?;
            let uncompressed_size = u32::from_le_bytes(uncompressed_size);

            // Reads the filename length.
            let mut filename_length = [0; 2];
            reader.read_exact(&mut filename_length)?;
            let filename_length = u16::from_le_bytes(filename_length);

            // Reads the extra field length.
            let mut extra_field_length = [0; 2];
            reader.read_exact(&mut extra_field_length)?;
            let extra_field_length = u16::from_le_bytes(extra_field_length);

            // Reads the file comment length.
            let mut file_comment_length = [0; 2];
            reader.read_exact(&mut file_comment_length)?;
            let file_comment_length = u16::from_le_bytes(file_comment_length);

            // Reads the relative offset of local file header.
            reader.seek(SeekFrom::Current(8))?;
            let mut offset = [0; 4];
            reader.read_exact(&mut offset)?;
            let offset = u32::from_le_bytes(offset);

            // Reads the filename.
            let mut filename = vec![0; filename_length as usize];
            reader.read_exact(&mut filename)?;
            let filename = String::from_utf8_lossy(&filename).to_string();

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
                    let mut filename_length = [0; 2];
                    reader.read_exact(&mut filename_length)?;
                    let filename_length = u16::from_le_bytes(filename_length);

                    // Reads the extra field length.
                    let mut extra_field_length = [0; 2];
                    reader.read_exact(&mut extra_field_length)?;
                    let extra_field_length = u16::from_le_bytes(extra_field_length);

                    // Seeks to the data.
                    reader.seek(SeekFrom::Current(
                        filename_length as i64 + extra_field_length as i64,
                    ))?;

                    // Reads the data.
                    let mut data = vec![0; compressed_size as usize];
                    reader.read_exact(&mut data)?;
                    let data = String::from_utf8_lossy(&data).to_string();

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

/// Searches for the `target` byte sequence in the `data` slice using the Boyer-Moore algorithm.
///
/// If the sequence is found, the function returns the index of the first occurrence.
#[cfg(any(feature = "reader-pdf", feature = "reader-zip"))]
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
