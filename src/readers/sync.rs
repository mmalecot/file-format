//! Readers for specific file formats.

use std::io::*;

use super::common::FindBytes;

impl crate::FileFormat {
    /// Determines file format from the specified format reader, if any.
    #[inline]
    pub(crate) fn from_fmt_reader<R: Read + Seek>(
        fmt: Self,
        #[allow(unused_variables)] reader: R,
    ) -> Result<Self> {
        Ok(match fmt {
            #[cfg(feature = "reader-asf")]
            Self::AdvancedSystemsFormat => Self::from_asf_reader(reader)?,
            #[cfg(feature = "reader-cfb")]
            Self::CompoundFileBinary => Self::from_cfb_reader(reader)?,
            #[cfg(feature = "reader-ebml")]
            Self::ExtensibleBinaryMetaLanguage => Self::from_ebml_reader(reader)?,
            #[cfg(feature = "reader-exe")]
            Self::MsDosExecutable => Self::from_exe_reader(reader)?,
            #[cfg(feature = "reader-id3v2")]
            Self::Id3v2 => Self::from_id3v2_reader(reader)?,
            #[cfg(feature = "reader-mp4")]
            Self::Mpeg4Part14 => Self::from_mp4_reader(reader)?,
            #[cfg(feature = "reader-pdf")]
            Self::PortableDocumentFormat => Self::from_pdf_reader(reader)?,
            #[cfg(feature = "reader-rm")]
            Self::Realmedia => Self::from_rm_reader(reader)?,
            #[cfg(feature = "reader-sqlite3")]
            Self::Sqlite3 => Self::from_sqlite3_reader(reader)?,
            #[cfg(feature = "reader-xml")]
            Self::ExtensibleMarkupLanguage => Self::from_xml_reader(reader)?,
            #[cfg(feature = "reader-zip")]
            Self::Zip => Self::from_zip_reader(reader)?,
            _ => fmt,
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
    fn from_asf_reader<R: Read + Seek>(reader: R) -> Result<Self> {
        use super::common::asf::{
            determine_file_format, AUDIO_MEDIA_GUID, DESCRIPTOR_LIMIT, DESCRIPTOR_NAME_LIMIT,
            EXTENDED_CONTENT_DESCRIPTION_OBJECT_GUID, OBJECT_LIMIT, STREAM_PROPERTIES_OBJECT_GUID,
            VIDEO_MEDIA_GUID,
        };

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
            let guid = reader.read_guid()?;

            // Reads the object size.
            let size = reader.read_u64_le()?;

            // Checks the object GUID.
            match guid.as_str() {
                STREAM_PROPERTIES_OBJECT_GUID => {
                    // Reads the stream type.
                    let stream_type = reader.read_guid()?;

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
                        let len = reader.read_u16_le()?;

                        // Reads the descriptor name.
                        let name = reader
                            .read_bytes(std::cmp::min(DESCRIPTOR_NAME_LIMIT, len as usize))?;

                        // Checks the descriptor name.
                        if name.starts_with(b"D\0V\0R\0 \0F\0i\0l\0e\0 \0V\0e\0r\0s\0i\0o\0n\0") {
                            return Ok(Self::MicrosoftDigitalVideoRecording);
                        }

                        // Calculates the remaining length.
                        let remaining_len = len.saturating_sub(DESCRIPTOR_NAME_LIMIT as u16);

                        // Reads the descriptor value length.
                        reader.seek(SeekFrom::Current(remaining_len as i64 + 2))?;
                        let len = reader.read_u16_le()?;

                        // Seeks to the next content descriptor.
                        reader.seek(SeekFrom::Current(len as i64))?;
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
        Ok(determine_file_format(video_stream, audio_stream))
    }

    /// Determines file format from a CFB reader.
    #[cfg(feature = "reader-cfb")]
    fn from_cfb_reader<R: Read + Seek>(mut reader: R) -> Result<Self> {
        // Reads the major version.

        use super::common::cfb::determine_file_format;
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
        let clsid = reader.read_guid()?;

        // Determines the file format based on the CLSID.
        Ok(if let Some(fmt) = determine_file_format(clsid.as_str()) {
            fmt
        } else {
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
        })
    }

    /// Determines file format from an EBML reader.
    #[cfg(feature = "reader-ebml")]
    fn from_ebml_reader<R: Read + Seek>(reader: R) -> Result<Self> {
        use super::common::ebml::{
            determine_file_format, CLUSTER_ELEMENT_ID, CODEC_ID_ELEMENT_ID, CODEC_ID_LIMIT,
            DOC_TYPE_ELEMENT_ID, DOC_TYPE_LIMIT, EBML_ELEMENT_ID, ELEMENT_LIMIT,
            SEGMENT_ELEMENT_ID, STEREO_MODE_ELEMENT_ID, TRACKS_ELEMENT_ID, TRACK_ENTRY_ELEMENT_ID,
            VIDEO_ELEMENT_ID,
        };

        // Creates a buffered reader.
        let mut reader = BufReader::new(reader);

        // Retrieves the stream length.
        let len = reader.seek(SeekFrom::End(0))?;

        // Rewinds to the beginning of the stream.
        reader.rewind()?;

        // Flags indicating the presence of audio, video and subtitle tracks.
        let mut audio_track = false;
        let mut video_track = false;
        let mut subtitle_track = false;

        // Iterates through the EBML elements.
        let mut element_count = 0;
        while element_count < ELEMENT_LIMIT && reader.stream_position()? < len {
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
        Ok(determine_file_format(
            video_track,
            audio_track,
            subtitle_track,
        ))
    }

    /// Determines file format from an EXE reader.
    #[cfg(feature = "reader-exe")]
    fn from_exe_reader<R: Read + Seek>(mut reader: R) -> Result<Self> {
        // Retrieves the stream length.
        let len = reader.seek(SeekFrom::End(0))?;

        // Reads the extended header address.
        reader.seek(SeekFrom::Start(60))?;
        let offset = reader.read_u32_le()?;

        // Checks that the offset value is not outside the stream's boundaries.
        if offset as u64 + 4 < len {
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

    /// Determines file format from an ID3v2 reader.
    #[cfg(feature = "reader-id3v2")]
    pub(crate) fn from_id3v2_reader<R: Read + Seek>(mut reader: R) -> Result<Self> {
        // Loops while in ID3v2 segment.
        let mut offset = 0;
        loop {
            // Reads first 10 bytes.
            reader.seek(SeekFrom::Start(offset))?;
            let buf = reader.read_bytes(10)?;

            // Checks for ID3 magic bytes.
            if &buf[..3] != b"ID3" {
                break;
            }

            // Decodes tag size.
            let tag_size = ((buf[6] as u64 & 0x7F) << 21)
                | ((buf[7] as u64 & 0x7F) << 14)
                | ((buf[8] as u64 & 0x7F) << 7)
                | (buf[9] as u64 & 0x7F);

            // Checks for extended header flag.
            let flags = buf[5];
            let extended_header_present = (flags & 0x40) != 0;
            let footer_present = (flags & 0x10) != 0;

            // Calculates next offset.
            let mut next_offset = offset + 10 + tag_size;

            // Skips extended header if present.
            if extended_header_present {
                next_offset += reader.read_u32_be()? as u64;
            }

            // Adds footer size if present.
            if footer_present {
                next_offset += 10;
            }

            // Sets new offset for next potential tag.
            offset = next_offset;
        }

        // Skips ID3v2 segment.
        reader.seek(SeekFrom::Start(offset))?;

        // Checks if the file contains the FLAC file format signature.
        let buf = reader.read_bytes(4)?;
        if buf.eq(b"fLaC") {
            return Ok(Self::FreeLosslessAudioCodec);
        }

        // Checks if the file contains one of the MP3 file format signature.
        match &buf[..2] {
            b"\xFF\xE2" => return Ok(Self::Mpeg12AudioLayer3),
            b"\xFF\xE3" => return Ok(Self::Mpeg12AudioLayer3),
            b"\xFF\xF2" => return Ok(Self::Mpeg12AudioLayer3),
            b"\xFF\xF3" => return Ok(Self::Mpeg12AudioLayer3),
            b"\xFF\xFA" => return Ok(Self::Mpeg12AudioLayer3),
            b"\xFF\xFB" => return Ok(Self::Mpeg12AudioLayer3),
            _ => {}
        }

        // Returns the default value.
        Ok(Self::Id3v2)
    }

    /// Determines file format from a MP4 reader.
    #[cfg(feature = "reader-mp4")]
    fn from_mp4_reader<R: Read + Seek>(reader: R) -> Result<Self> {
        use super::common::mp4::{determine_file_format, BOX_LIMIT};

        // Creates a buffered reader.
        let mut reader = BufReader::new(reader);

        // Retrieves the stream length.
        let len = reader.seek(SeekFrom::End(0))?;

        // Rewinds to the beginning of the stream.
        reader.rewind()?;

        // Flags indicating the presence of audio, video and subtitle tracks.
        let mut audio_track = false;
        let mut video_track = false;
        let mut subtitle_track = false;

        // Iterates through boxes.
        let mut box_count = 0;
        while box_count < BOX_LIMIT && reader.stream_position()? < len {
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
        Ok(determine_file_format(
            video_track,
            audio_track,
            subtitle_track,
        ))
    }

    /// Determines file format from a PDF reader.
    #[cfg(feature = "reader-pdf")]
    fn from_pdf_reader<R: Read + Seek>(mut reader: R) -> Result<Self> {
        use super::common::pdf::{AI_MARKER, CHUNK_SIZE, OVERLAP_SIZE, READ_LIMIT};

        // Rewinds to the beginning of the stream plus the size of the PDF file format signature.
        reader.seek(SeekFrom::Start(5))?;

        // Creates a buffer to hold the chunk of data being read.
        let mut buf = [0; OVERLAP_SIZE + CHUNK_SIZE];

        // Reads the data from the stream in chunks.
        let mut total_nread = 0;
        while total_nread < READ_LIMIT {
            // Reads a chunk of the stream into the buffer.
            let nread = reader.read(&mut buf[OVERLAP_SIZE..])?;
            if nread == 0 {
                break;
            }

            // Determines the start index for searching the buffer.
            let start = if total_nread == 0 { OVERLAP_SIZE } else { 0 };

            // Checks if the buffer holds the AI file format marker.
            if buf[start..OVERLAP_SIZE + nread].holds(AI_MARKER) {
                return Ok(Self::AdobeIllustratorArtwork);
            }

            // Rotates the buffer to the right by the overlap size.
            buf[..OVERLAP_SIZE + nread].rotate_right(OVERLAP_SIZE);

            // Updates the total bytes read.
            total_nread += nread;
        }

        // Returns the default value.
        Ok(Self::PortableDocumentFormat)
    }

    /// Determines file format from a RM reader.
    #[cfg(feature = "reader-rm")]
    fn from_rm_reader<R: Read + Seek>(reader: R) -> Result<Self> {
        use super::common::rm::{determine_file_format, CHUNK_LIMIT};

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
        Ok(determine_file_format(video_stream, audio_stream))
    }

    /// Determines file format from a SQLite 3 reader.
    #[cfg(feature = "reader-sqlite3")]
    fn from_sqlite3_reader<R: Read + Seek>(mut reader: R) -> Result<Self> {
        use super::common::sqlite3::check_if_buffer_holds_sketch_marker;

        // Rewinds to the beginning of the stream plus the size of the SQLite 3 file format header.
        reader.seek(SeekFrom::Start(100))?;

        // Creates and fills a buffer.
        let mut buf = [0; 32_768];
        let nread = reader.read(&mut buf)?;

        // Checks if the buffer holds the Sketch file format marker.
        if let Some(fmt) = check_if_buffer_holds_sketch_marker(&buf, nread) {
            return Ok(fmt);
        }

        // Returns the default value.
        Ok(Self::Sqlite3)
    }

    /// Determines file format from a TXT reader.
    #[cfg(feature = "reader-txt")]
    fn from_txt_reader<R: Read + Seek>(reader: R) -> Result<Self> {
        use super::common::txt::{check_for_non_whitespace_control_char, LINE_LIMIT, READ_LIMIT};

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
            .try_for_each(check_for_non_whitespace_control_char)
            .map(|_| Self::PlainText)
    }

    /// Determines file format from a XML reader.
    #[cfg(feature = "reader-xml")]
    fn from_xml_reader<R: Read + Seek>(mut reader: R) -> Result<Self> {
        use super::common::xml::check_if_buffer_holds_markers;

        // Rewinds to the beginning of the stream plus the size of the XML file format signature.
        reader.seek(SeekFrom::Start(5))?;

        // Creates and fills a buffer.
        let mut buf = [0; 8192];
        let nread = reader.read(&mut buf)?;
        let buf = &buf[..nread];

        // Checks if the buffer holds markers indicating the presence of various file formats.
        Ok(check_if_buffer_holds_markers(&buf))
    }

    /// Determines file format from a ZIP reader.
    #[cfg(feature = "reader-zip")]
    fn from_zip_reader<R: Read + Seek>(reader: R) -> Result<Self> {
        use super::common::zip::{
            check_filename, check_filename_starts_with, check_trimmed_data, ENTRY_LIMIT,
            EOCD64_LOCATOR_SIGNATURE, EOCD64_LOCATOR_SIZE, EOCD_MAX_SIZE, EOCD_MIN_SIZE,
            EOCD_SIGNATURE,
        };

        // Creates a buffered reader.
        let mut reader = BufReader::new(reader);

        // Retrieves the stream length.
        let len = reader.seek(SeekFrom::End(0))?;

        // Searches for the end of central directory record.
        let offset = len.saturating_sub(EOCD_MAX_SIZE as u64);
        reader.seek(SeekFrom::Start(offset))?;
        let buf_index = reader
            .read_bytes((len as usize).clamp(EOCD_MIN_SIZE, EOCD_MAX_SIZE))?
            .rfind(EOCD_SIGNATURE)
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "cannot find the EOCD"))?;
        let eocd_offset = offset + buf_index as u64;

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
        let mut fmt = Self::Zip;

        // Browses central directory headers.
        for _ in 0..std::cmp::min(ENTRY_LIMIT, number_of_entries) {
            // Reads the compressed size.
            reader.seek(SeekFrom::Current(20))?;
            let compressed_size = reader.read_u32_le()?;

            // Reads the uncompressed size.
            let uncompressed_size = reader.read_u32_le()?;

            // Reads the filename length.
            let filename_len = reader.read_u16_le()?;

            // Reads the extra field length.
            let extra_field_len = reader.read_u16_le()?;

            // Reads the file comment length.
            let file_comment_len = reader.read_u16_le()?;

            // Reads the relative offset of local file header.
            reader.seek(SeekFrom::Current(8))?;
            let offset = reader.read_u32_le()?;

            // Reads the filename.
            let filename = reader.read_string(filename_len as usize)?;

            // Checks the filename.
            if let Some(fmt) = check_filename(filename.as_str()) {
                return Ok(fmt);
            } else {
                match filename.as_str() {
                    "META-INF/MANIFEST.MF" => fmt = Self::JavaArchive,
                    "mimetype" if compressed_size == uncompressed_size => {
                        // Seeks to the filename of the local file header.
                        reader.seek(SeekFrom::Start(offset as u64 + 26))?;

                        // Reads the filename length.
                        let filename_len = reader.read_u16_le()?;

                        // Reads the extra field length.
                        let extra_field_len = reader.read_u16_le()?;

                        // Seeks to the data.
                        reader.seek(SeekFrom::Current(
                            filename_len as i64 + extra_field_len as i64,
                        ))?;

                        // Reads the data.
                        let data = reader.read_string(compressed_size as usize)?;

                        // Checks the trimmed data.
                        return Ok(check_trimmed_data(data));
                    }
                    _ => {
                        if let Some(fmt) = check_filename_starts_with(filename) {
                            return Ok(fmt);
                        }
                    }
                }
            }

            // Seeks to the next central directory entry.
            reader.seek(SeekFrom::Current(
                extra_field_len as i64 + file_comment_len as i64,
            ))?;
        }
        Ok(fmt)
    }
}

/// A trait for convenient data reading.
#[allow(dead_code)]
trait ReadData: Read {
    /// Reads a specified number of bytes into a `Vec<u8>`.
    #[inline]
    fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; size];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// Reads a GUID and converts it into a hyphenated `String`.
    #[inline]
    fn read_guid(&mut self) -> Result<String> {
        let buf = self.read_bytes(16)?;
        Ok([3, 2, 1, 0, 5, 4, 7, 6, 8, 9, 10, 11, 12, 13, 14, 15]
            .iter()
            .map(|&index| {
                if index == 5 || index == 7 || index == 8 || index == 10 {
                    format!("-{:02x}", buf[index])
                } else {
                    format!("{:02x}", buf[index])
                }
            })
            .collect())
    }

    /// Reads a specified number of bytes and converts them into a `String`.
    #[inline]
    fn read_string(&mut self, size: usize) -> Result<String> {
        Ok(String::from_utf8_lossy(&self.read_bytes(size)?).to_string())
    }

    /// Reads a single `u8` value.
    #[inline]
    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    /// Reads a `u16` value in little-endian byte order.
    #[inline]
    fn read_u16_le(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    /// Reads a `u32` value in big-endian byte order.
    #[inline]
    fn read_u32_be(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    /// Reads a `u32` value in little-endian byte order.
    #[inline]
    fn read_u32_le(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    /// Reads a `u64` value in big-endian byte order.
    #[inline]
    fn read_u64_be(&mut self) -> Result<u64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }

    /// Reads a `u64` value in little-endian byte order.
    #[inline]
    fn read_u64_le(&mut self) -> Result<u64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }
}

/// Allows any type `R` that implements the `Read` trait to automatically benefit from the
/// additional methods provided by the `ReadData` trait.
impl<R: Read> ReadData for R {}
