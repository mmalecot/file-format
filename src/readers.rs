//! Readers for specific file formats.

use std::io::*;

impl crate::FileFormat {
    /// Determines file format from the specified format reader, if any.
    #[inline]
    pub(crate) fn from_format_reader<R: Read + Seek>(
        format: Self,
        #[allow(unused_variables)] reader: R,
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
    #[inline]
    pub(crate) fn from_generic_reader<R: Read + Seek>(
        #[allow(unused_variables)] reader: R,
    ) -> Self {
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

        // GUID for extended content description object.
        const EXTENDED_CONTENT_DESCRIPTION_OBJECT_GUID: &str =
            "d2d0a440-e307-11d2-97f0-00a0c95ea850";

        // GUID for stream properties object.
        const STREAM_PROPERTIES_OBJECT_GUID: &str = "b7dc0791-a9b7-11cf-8ee6-00c00c205365";

        // GUID for audio media.
        const AUDIO_MEDIA_GUID: &str = "f8699e40-5b4d-11cf-a8fd-00805f5c442b";

        // GUID for video media.
        const VIDEO_MEDIA_GUID: &str = "bc19efc0-5b4d-11cf-a8fd-00805f5c442b";

        // Creates a buffered reader.
        let mut reader = BufReader::new(reader);

        // Reads the number of header objects.
        reader.seek(SeekFrom::Start(24))?;
        let number_of_header_objects = reader.read_u32_le()?;

        // Skips the reserved fields.
        reader.seek(SeekFrom::Current(2))?;

        // Flags indicating the presence of audio and video streams.
        let mut audio_stream = false;
        let mut video_stream = false;

        // Iterates through the header objects.
        for _ in 0..std::cmp::min(OBJECT_LIMIT, number_of_header_objects as usize) {
            // Reads the object GUID.
            let guid = reader.read_uuid()?;

            // Reads the object size.
            let size = reader.read_u64_le()?;

            // Checks the object GUID.
            match guid.as_str() {
                STREAM_PROPERTIES_OBJECT_GUID => {
                    // Reads the stream type.
                    let stream_type = reader.read_uuid()?;

                    // Checks the stream type.
                    match stream_type.as_str() {
                        AUDIO_MEDIA_GUID => audio_stream = true,
                        VIDEO_MEDIA_GUID => video_stream = true,
                        _ => {}
                    }

                    // Seeks to the next object.
                    reader.seek(SeekFrom::Current(size as i64 - 40))?;
                }
                EXTENDED_CONTENT_DESCRIPTION_OBJECT_GUID => {
                    // Reads the content descriptors count.
                    let count = reader.read_u16_le()?;

                    // Calculates the offset of the content descriptors.
                    let offset = reader.stream_position()?;

                    // Iterates through the content descriptors.
                    for _ in 0..std::cmp::min(DESCRIPTOR_LIMIT, count as usize) {
                        // Reads the descriptor name length.
                        let length = reader.read_u16_le()?;

                        // Reads the descriptor name.
                        let name = reader
                            .read_bytes(std::cmp::min(DESCRIPTOR_NAME_LIMIT, length as usize))?;

                        // Checks the descriptor name.
                        if name == b"D\0V\0R\0 \0F\0i\0l\0e\0 \0V\0e\0r\0s\0i\0o\0n\0\0\0" {
                            return Ok(Self::MicrosoftDigitalVideoRecording);
                        }

                        // Calculates the remaining length.
                        let remaining_length = length.saturating_sub(DESCRIPTOR_NAME_LIMIT as u16);

                        // Reads the descriptor value length.
                        reader.seek(SeekFrom::Current(remaining_length as i64 + 2))?;
                        let length = reader.read_u16_le()?;

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
        // Reads the major version.
        reader.seek(SeekFrom::Start(26))?;
        let major_version = reader.read_u16_le()?;

        // Calculates the directory sector size based on the major version.
        let directory_sector_size = if major_version == 0x0003 { 512 } else { 4096 };

        // Reads the first directory sector location.
        reader.seek(SeekFrom::Current(20))?;
        let first_directory_sector_location = reader.read_u32_le()?;

        // Seeks to the root entry CLSID.
        let offset = directory_sector_size * (1 + first_directory_sector_location as u64) + 80;
        reader.seek(SeekFrom::Start(offset))?;

        // Reads the CLSID.
        let clsid = reader.read_uuid()?;

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
                let directory_entry_name = reader.read_bytes(64)?;

                // Checks the second directory entry name.
                if directory_entry_name.starts_with(b"M\0a\0t\0O\0S\0T\0") {
                    return Ok(Self::MicrosoftWorksWordProcessor);
                }

                // Reads the third directory entry name.
                reader.seek(SeekFrom::Current(64))?;
                let directory_entry_name = reader.read_bytes(64)?;

                // Checks the third directory entry name.
                if directory_entry_name.starts_with(b"W\0k\0s\0S\0S\0W\0o\0r\0k\0B\0o\0o\0k\0") {
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

        // Retrieves the stream length.
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
            let first_byte = reader.read_u8()?;

            // Calculates the number of bytes used to represent the element ID.
            let number_of_bytes = first_byte.leading_zeros() + 1;
            if number_of_bytes > 4 {
                return Err(Error::new(ErrorKind::InvalidData, "invalid EBML ID"));
            }

            // Calculates the element ID value based on the number of bytes.
            let mut id = first_byte as u32;
            for _ in 1..number_of_bytes {
                let byte = reader.read_u8()?;
                id = (id << 8) | (byte as u32);
            }

            // Reads the first byte of the element size.
            let first_byte = reader.read_u8()?;

            // Calculates the number of bytes used to represent the element size.
            let number_of_bytes = first_byte.leading_zeros() + 1;
            if number_of_bytes > 8 {
                return Err(Error::new(ErrorKind::InvalidData, "invalid EBML size"));
            }

            // Calculates the element size value based on the number of bytes.
            let mut size = u64::from(first_byte & ((128 >> first_byte.leading_zeros()) - 1));
            for _ in 1..number_of_bytes {
                let byte = reader.read_u8()?;
                size = (size << 8) | (byte as u64);
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
                    let doc_type =
                        reader.read_bytes(std::cmp::min(DOC_TYPE_LIMIT, size as usize))?;

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
                    let codec_id =
                        reader.read_bytes(std::cmp::min(CODEC_ID_LIMIT, size as usize))?;

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
                    let stereo_mode = reader.read_u8()?;

                    // Checks the StereoMode.
                    if stereo_mode > 0 {
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
        // Retrieves the stream length.
        let length = reader.seek(SeekFrom::End(0))?;

        // Reads the extended header address.
        reader.seek(SeekFrom::Start(60))?;
        let offset = reader.read_u32_le()?;

        // Checks that the offset value is not outside the stream's boundaries.
        if offset as u64 + 4 < length {
            // Seeks to the offset.
            reader.seek(SeekFrom::Start(offset as u64))?;

            // Reads the signature.
            let signature = reader.read_bytes(4)?;

            // Checks the signature.
            if &signature == b"PE\0\0" {
                // Reads the characteristics.
                reader.seek(SeekFrom::Current(18))?;
                let characteristics = reader.read_u16_le()?;

                // Checks the characteristics
                return Ok(if characteristics & 0x2000 == 0x2000 {
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

        // Retrieves the stream length.
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
            let size = reader.read_u32_be()?;

            // Reads the box type.
            let box_type = reader.read_bytes(4)?;

            // Handles the extended box size.
            let size = if size == 1 {
                reader.read_u64_be()?
            } else {
                size as u64
            };

            // Checks the box type.
            match box_type.as_slice() {
                b"moov" | b"trak" | b"mdia" => {}
                b"hdlr" => {
                    // Reads the handler type.
                    reader.seek(SeekFrom::Current(8))?;
                    let handler_type = reader.read_bytes(4)?;

                    // Checks the handler type.
                    match handler_type.as_slice() {
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
            if buffer[start..OVERLAP_SIZE + bytes_read]
                .find(AI_MARKER)
                .is_some()
            {
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
        let number_of_headers = reader.read_u32_be()?;

        // Flags indicating the presence of audio and video streams.
        let mut audio_stream = false;
        let mut video_stream = false;

        // Iterates through the chunks.
        for _ in 0..std::cmp::min(CHUNK_LIMIT, number_of_headers.saturating_sub(1) as usize) {
            // Reads the chunk type.
            let chunk_type = reader.read_bytes(4)?;

            // Reads the chunk size.
            let chunk_size = reader.read_u32_be()?;

            // Checks the chunk type.
            if &chunk_type == b"MDPR" {
                // Calculates the offset of the media properties.
                let offset = reader.stream_position()?;

                // Reads the size of the stream name.
                reader.seek(SeekFrom::Current(32))?;
                let stream_name_size = reader.read_u8()?;

                // Skips the stream name.
                reader.seek(SeekFrom::Current(stream_name_size as i64))?;

                // Reads the size of stream mime type.
                let mime_type_size = reader.read_u8()?;

                // Reads the mime type.
                let mime_type = reader.read_bytes(mime_type_size as usize)?;

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
        // Maximum number of entries that can be processed by the reader.
        const ENTRY_LIMIT: usize = 1024;

        // Signature of the ZIP64 end of central directory locator.
        const EOCD64_LOCATOR_SIGNATURE: &[u8] = b"PK\x06\x07";

        // Signature of the end of central directory record.
        const EOCD_SIGNATURE: &[u8] = b"PK\x05\x06";

        // Size of the ZIP64 end of central directory locator.
        const EOCD64_LOCATOR_SIZE: usize = 20;

        // Maximum size of the end of central directory record.
        const EOCD_MAX_SIZE: usize = EOCD_MIN_SIZE + u16::MAX as usize;

        // Minimum size of the end of central directory record.
        const EOCD_MIN_SIZE: usize = 22;

        // Creates a buffered reader.
        let mut reader = BufReader::new(reader);

        // Retrieves the stream length.
        let length = reader.seek(SeekFrom::End(0))?;

        // Searches for the end of central directory record.
        let offset = length.saturating_sub(EOCD_MAX_SIZE as u64);
        reader.seek(SeekFrom::Start(offset))?;
        let buffer_index = reader
            .read_bytes((length as usize).clamp(EOCD_MIN_SIZE, EOCD_MAX_SIZE))?
            .find(EOCD_SIGNATURE)
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "cannot find the EOCD record"))?;
        let eocd_offset = offset + buffer_index as u64;

        // Checks for ZIP64 end of central directory locator.
        let mut zip64 = false;
        if eocd_offset as usize >= EOCD64_LOCATOR_SIZE {
            // Seeks to the ZIP64 end of central directory locator.
            reader.seek(SeekFrom::Start(eocd_offset - EOCD64_LOCATOR_SIZE as u64))?;

            // Reads the signature.
            let signature = reader.read_bytes(4)?;

            // Checks the signature.
            if signature == EOCD64_LOCATOR_SIGNATURE {
                zip64 = true;
            }
        }

        // Reads the number of entries and the start of central directory offset.
        let (number_of_entries, socd_offset) = if zip64 {
            // Reads the offset of the ZIP64 end of central directory record.
            reader.seek(SeekFrom::Current(4))?;
            let eocd64_offset = reader.read_u64_le()?;

            // Reads the number of entries.
            reader.seek(SeekFrom::Start(eocd64_offset + 32))?;
            let number_of_entries = reader.read_u64_le()?;

            // Reads the start of central directory offset.
            reader.seek(SeekFrom::Current(8))?;
            let socd_offset = reader.read_u64_le()?;

            // Returns the result.
            (number_of_entries as usize, socd_offset)
        } else {
            // Reads the number of entries.
            reader.seek(SeekFrom::Start(eocd_offset + 10))?;
            let number_of_entries = reader.read_u16_le()?;

            // Reads the start of central directory offset.
            reader.seek(SeekFrom::Current(4))?;
            let socd_offset = reader.read_u32_le()?;

            // Returns the result.
            (number_of_entries as usize, socd_offset as u64)
        };

        // Seeks to the start of central directory.
        reader.seek(SeekFrom::Start(socd_offset))?;

        // Sets the default value.
        let mut format = Self::Zip;

        // Browses central directory headers.
        for _ in 0..std::cmp::min(ENTRY_LIMIT, number_of_entries) {
            // Reads the compressed size.
            reader.seek(SeekFrom::Current(20))?;
            let compressed_size = reader.read_u32_le()?;

            // Reads the uncompressed size.
            let uncompressed_size = reader.read_u32_le()?;

            // Reads the filename length.
            let filename_length = reader.read_u16_le()?;

            // Reads the extra field length.
            let extra_field_length = reader.read_u16_le()?;

            // Reads the file comment length.
            let file_comment_length = reader.read_u16_le()?;

            // Reads the relative offset of local file header.
            reader.seek(SeekFrom::Current(8))?;
            let offset = reader.read_u32_le()?;

            // Reads the filename.
            let filename = reader.read_string(filename_length as usize)?;

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
                    let filename_length = reader.read_u16_le()?;

                    // Reads the extra field length.
                    let extra_field_length = reader.read_u16_le()?;

                    // Seeks to the data.
                    reader.seek(SeekFrom::Current(
                        filename_length as i64 + extra_field_length as i64,
                    ))?;

                    // Reads the data.
                    let data = reader.read_string(compressed_size as usize)?;

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

            // Seeks to the next central directory entry.
            reader.seek(SeekFrom::Current(
                extra_field_length as i64 + file_comment_length as i64,
            ))?;
        }
        Ok(format)
    }
}

/// A trait extending the standard `Read` trait with methods for convenient data reading.
trait ReadData: Read {
    /// Reads a specified number of bytes into a `Vec<u8>`.
    #[inline]
    fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0; count];
        self.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    /// Reads a specified number of bytes and converts them into a `String`.
    #[inline]
    fn read_string(&mut self, count: usize) -> Result<String> {
        Ok(String::from_utf8_lossy(&self.read_bytes(count)?).to_string())
    }

    /// Reads a UUID and decodes it into a hyphenated `String`.
    #[inline]
    fn read_uuid(&mut self) -> Result<String> {
        let buffer = self.read_bytes(16)?;
        Ok([3, 2, 1, 0, 5, 4, 7, 6, 8, 9, 10, 11, 12, 13, 14, 15]
        .iter()
        .map(|&index| {
            if index == 5 || index == 7 || index == 8 || index == 10 {
                format!("-{:02x}", buffer[index])
            } else {
                format!("{:02x}", buffer[index])
            }
        })
        .collect())
    }

    /// Reads a single `u8` value.
    #[inline]
    fn read_u8(&mut self) -> Result<u8> {
        let mut buffer = [0; 1];
        self.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }

    /// Reads a `u16` value in little-endian byte order.
    #[inline]
    fn read_u16_le(&mut self) -> Result<u16> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    /// Reads a `u32` value in big-endian byte order.
    #[inline]
    fn read_u32_be(&mut self) -> Result<u32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;
        Ok(u32::from_be_bytes(buffer))
    }

    /// Reads a `u32` value in little-endian byte order.
    #[inline]
    fn read_u32_le(&mut self) -> Result<u32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    /// Reads a `u64` value in big-endian byte order.
    #[inline]
    fn read_u64_be(&mut self) -> Result<u64> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)?;
        Ok(u64::from_be_bytes(buffer))
    }

    /// Reads a `u64` value in little-endian byte order.
    #[inline]
    fn read_u64_le(&mut self) -> Result<u64> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)?;
        Ok(u64::from_le_bytes(buffer))
    }
}

/// Allows any type `R` that implements the `Read` trait to automatically benefit from the
/// additional methods provided by the `ReadData` trait.
impl<R: Read> ReadData for R {}

/// A trait for finding a byte pattern within data that can be represented as a slice of bytes.
trait FindBytes: AsRef<[u8]> {
    /// Searches for the specified byte pattern and returns the index of the first occurrence.
    fn find<P: AsRef<[u8]>>(&self, pattern: P) -> Option<usize> {
        // Retrieves references to data and pattern.
        let data = self.as_ref();
        let pattern = pattern.as_ref();

        // An empty pattern is always considered to be contained in the data.
        if pattern.is_empty() {
            return Some(0);
        }

        // The data is shorter than the pattern, so it cannot contain it.
        if data.len() < pattern.len() {
            return None;
        }

        // Creates a shift table for efficient pattern matching.
        let mut shift_table = [pattern.len(); 256];
        for (index, &byte) in pattern.iter().enumerate().take(pattern.len() - 1) {
            shift_table[byte as usize] = pattern.len() - 1 - index;
        }

        // Searches for the pattern using the Boyer-Moore-Horspool algorithm.
        let mut data_index = pattern.len() - 1;
        while data_index < data.len() {
            let mut target_index = pattern.len() - 1;
            while pattern[target_index] == data[data_index - (pattern.len() - 1 - target_index)] {
                if target_index == 0 {
                    return Some(data_index - (pattern.len() - 1));
                }
                target_index -= 1;
            }
            data_index += shift_table[data[data_index] as usize];
        }
        None
    }
}

/// Allows any type `A` that implements the `AsRef<[u8]>` trait to benefit from the additional
/// `find` method provided by the `FindBytes` trait.
impl<A: AsRef<[u8]> + ?Sized> FindBytes for A {}
