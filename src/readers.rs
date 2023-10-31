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
        // Constants for limits.
        const SEARCH_LIMIT: usize = 32768;

        // Constants for CLSIDs.
        const AUTODESK_INVENTORY_ASSEMBLY_CLSID: &[u8] =
            b"\xE1\x81\x0F\xE6\xB3\x49\xD0\x11\x93\xC3\x7E\x07\x06\x00\x00\x00";
        const AUTODESK_INVENTOR_DRAWING_CLSID: &[u8] =
            b"\xF1\xFD\xF9\xBB\xDC\x52\xD0\x11\x8C\x04\x08\x00\x09\x0B\xE8\xEC";
        const AUTODESK_INVENTOR_PART_CLSID: &[u8] =
            b"\x90\xB4\x29\x4D\xB2\x49\xD0\x11\x93\xC3\x7E\x07\x06\x00\x00\x00";
        const AUTODESK_INVENTOR_PRESENTATION_CLSID: &[u8] =
            b"\x80\x3A\x28\x76\xDD\x50\xD3\x11\xA7\xE3\x00\xC0\x4F\x79\xD7\xBC";
        const MICROSOFT_EXCEL_SPREADSHEET_CLSID_1: &[u8] =
            b"\x10\x08\x02\x00\x00\x00\x00\x00\xC0\x00\x00\x00\x00\x00\x00\x46";
        const MICROSOFT_EXCEL_SPREADSHEET_CLSID_2: &[u8] =
            b"\x20\x08\x02\x00\x00\x00\x00\x00\xC0\x00\x00\x00\x00\x00\x00\x46";
        const MICROSOFT_POWERPOINT_PRESENTATION_CLSID_1: &[u8] =
            b"\x51\x48\x04\x00\x00\x00\x00\x00\xC0\x00\x00\x00\x00\x00\x00\x46";
        const MICROSOFT_POWERPOINT_PRESENTATION_CLSID_2: &[u8] =
            b"\x10\x8D\x81\x64\x9B\x4F\xCF\x11\x86\xEA\x00\xAA\x00\xB9\x29\xE8";
        const MICROSOFT_POWERPOINT_PRESENTATION_CLSID_3: &[u8] =
            b"\x70\xAE\x7B\xEA\x3B\xFB\xCD\x11\xA9\x03\x00\xAA\x00\x51\x0E\xA3";
        const MICROSOFT_PROJECT_PLAN_CLSID: &[u8] =
            b"\x3A\x8F\xB7\x74\xC8\xC8\xD1\x11\xBE\x11\x00\xC0\x4F\xB6\xFA\xF1";
        const MICROSOFT_PUBLISHER_DOCUMENT_CLSID: &[u8] =
            b"\x01\x12\x02\x00\x00\x00\x00\x00\x00\xC0\x00\x00\x00\x00\x00\x46";
        const MICROSOFT_SOFTWARE_INSTALLER_CLSID: &[u8] =
            b"\x84\x10\x0C\x00\x00\x00\x00\x00\xC0\x00\x00\x00\x00\x00\x00\x46";
        const MICROSOFT_VISIO_DRAWING_CLSID_1: &[u8] =
            b"\x13\x1A\x02\x00\x00\x00\x00\x00\xC0\x00\x00\x00\x00\x00\x00\x46";
        const MICROSOFT_VISIO_DRAWING_CLSID_2: &[u8] =
            b"\x14\x1A\x02\x00\x00\x00\x00\x00\xC0\x00\x00\x00\x00\x00\x00\x46";
        const MICROSOFT_WORD_DOCUMENT_CLSID_1: &[u8] =
            b"\x00\x90\x20\x00\x00\x00\x00\x00\xC0\x00\x00\x00\x00\x00\x00\x46";
        const MICROSOFT_WORD_DOCUMENT_CLSID_2: &[u8] =
            b"\x06\x09\x02\x00\x00\x00\x00\x00\xC0\x00\x00\x00\x00\x00\x00\x46";
        const MICROSOFT_WORKS_DATABASE_CLSID_1: &[u8] =
            b"\x03\x13\x02\x00\x00\x00\x00\x00\xC0\x00\x00\x00\x00\x00\x00\x46";
        const MICROSOFT_WORKS_DATABASE_CLSID_2: &[u8] =
            b"\xC3\xDB\xCD\x28\xE2\x0A\xCE\x11\xA2\x9A\x00\xAA\x00\x4A\x1A\x72";
        const MICROSOFT_WORKS_WORD_PROCESSOR_CLSID_1: &[u8] =
            b"\x02\x13\x02\x00\x00\x00\x00\x00\xC0\x00\x00\x00\x00\x00\x00\x46";
        const MICROSOFT_WORKS_WORD_PROCESSOR_CLSID_2: &[u8] =
            b"\xB2\x5A\xA4\x0E\x0A\x9E\xD1\x11\xA4\x07\x00\xC0\x4F\xB9\x32\xBA";
        const MICROSOFT_WORKS_WORD_PROCESSOR_CLSID_3: &[u8] =
            b"\xC2\xDB\xCD\x28\xE2\x0A\xCE\x11\xA2\x9A\x00\xAA\x00\x4A\x1A\x72";
        const SOLIDWORKS_ASSEMBLY_CLSID: &[u8] =
            b"\x36\x3D\xA3\x83\xC5\x27\xCE\x11\xBF\xD4\x00\x40\x05\x13\xBB\x57";
        const SOLIDWORKS_DRAWING_CLSID: &[u8] =
            b"\x34\x3D\xA3\x83\xC5\x27\xCE\x11\xBF\xD4\x00\x40\x05\x13\xBB\x57";
        const SOLIDWORKS_PART_CLSID: &[u8] =
            b"\x30\x3D\xA3\x83\xC5\x27\xCE\x11\xBF\xD4\x00\x40\x05\x13\xBB\x57";
        const STARCALC_CLSID_1: &[u8] =
            b"\xA0\x3F\x54\x3F\xA6\xB6\x1B\x10\x99\x61\x04\x02\x1C\x00\x70\x02";
        const STARCALC_CLSID_2: &[u8] =
            b"\x41\xD4\x61\x63\x35\x42\xD0\x11\x89\xCB\x00\x80\x29\xE4\xB0\xB1";
        const STARCALC_CLSID_3: &[u8] =
            b"\x61\xB8\xA5\xC6\xD6\x85\x1D\x11\x89\xCB\x00\x80\x29\xE4\xB0\xB1";
        const STARCHART_CLSID_1: &[u8] =
            b"\xE0\xB7\xB3\x02\x25\x42\xD0\x11\x89\xCA\x00\x80\x29\xE4\xB0\xB1";
        const STARCHART_CLSID_2: &[u8] =
            b"\x21\x43\x88\xBF\xDD\x85\x1D\x11\x89\xD0\x00\x80\x29\xE4\xB0\xB1";
        const STARCHART_CLSID_3: &[u8] =
            b"\xE0\x99\x9C\xFB\x6D\x2C\x1C\x10\x8E\x2C\x00\x00\x1B\x4C\xC7\x11";
        const STARDRAW_CLSID_1: &[u8] =
            b"\xA0\x05\x89\x2E\xBD\x85\xD1\x11\x89\xD0\x00\x80\x29\xE4\xB0\xB1";
        const STARDRAW_CLSID_2: &[u8] =
            b"\xE0\xAA\x10\xAF\x6D\xB3\x1B\x10\x99\x61\x04\x02\x1C\x00\x70\x02";
        const STARIMPRESS_CLSID_1: &[u8] =
            b"\xC0\x3C\x2D\x01\x16\x42\xD0\x11\x89\xCB\x00\x80\x29\xE4\xB0\xB1";
        const STARIMPRESS_CLSID_2: &[u8] =
            b"\x21\x72\x5C\x56\xBC\x85\x1D\x11\x89\xD0\x00\x80\x29\xE4\xB0\xB1";
        const STARMATH_CLSID_1: &[u8] =
            b"\xE1\xB7\xB3\x02\x25\x42\xD0\x11\x89\xCA\x00\x80\x29\xE4\xB0\xB1";
        const STARMATH_CLSID_2: &[u8] =
            b"\x60\x04\x59\xD4\xFD\x35\x1C\x10\xB1\x2A\x04\x02\x1C\x00\x70\x02";
        const STARMATH_CLSID_3: &[u8] =
            b"\x40\xE6\xB5\xFF\xDE\x85\x1D\x11\x89\xD0\x00\x80\x29\xE4\xB0\xB1";
        const STARWRITER_CLSID_1: &[u8] =
            b"\xB0\xE9\x04\x8B\x0E\x42\xD0\x11\xA4\x5E\x00\xA0\x24\x9D\x57\xB1";
        const STARWRITER_CLSID_2: &[u8] =
            b"\xD1\xF9\x0C\xC2\xAE\x85\x1D\x11\xAA\xB4\x00\x60\x97\xDA\x56\x1A";
        const STARWRITER_CLSID_3: &[u8] =
            b"\x40\x7E\x5C\xDC\x5C\xB3\x1B\x10\x99\x61\x04\x02\x1C\x00\x70\x02";
        const THREE_DIMENSIONAL_STUDIO_MAX_CLSID: &[u8] =
            b"\x7B\x8C\xDD\x1C\xC0\x81\xA0\x45\x9F\xED\x04\x14\x31\x44\xCC\x1E";
        const WORDPERFECT_DOCUMENT_CLSID: &[u8] =
            b"\xFF\x73\x98\x51\xAD\x2D\x20\x02\x19\x37\x00\x00\x92\x96\x79\xCD";
        const WORDPERFECT_GRAPHICS_CLSID: &[u8] =
            b"\x60\xFE\x2E\x40\x99\x19\x1B\x10\x99\xAE\x04\x02\x1C\x00\x70\x02";

        // Constants for UTF-16-encoded filenames.
        const MICROSOFT_WORKS6_SPREADSHEET_FILENAME: &[u8] =
            b"\x00W\x00k\x00s\x00S\x00S\x00W\x00o\x00r\x00k\x00B\x00o\x00o\x00k";
        const MICROSOFT_WORKS_WORD_PROCESSOR_FILENAME: &[u8] = b"\x00M\x00a\x00t\x00O\x00S\x00T";

        // Rewinds to the beginning of the stream.
        reader.rewind()?;

        // Gets the stream length.
        let length = reader.seek(SeekFrom::End(0))?;
        reader.rewind()?;

        // Skips the CFB header.
        reader.seek(SeekFrom::Start(512))?;

        // Fills the buffer.
        let mut buffer = vec![0; std::cmp::min(SEARCH_LIMIT, (length - 512) as usize)];
        reader.read_exact(&mut buffer)?;

        // Searches for specific CLSIDs or filenames in the buffer.
        Ok(if contains(&buffer, AUTODESK_INVENTORY_ASSEMBLY_CLSID) {
            Self::AutodeskInventorAssembly
        } else if contains(&buffer, AUTODESK_INVENTOR_DRAWING_CLSID) {
            Self::AutodeskInventorDrawing
        } else if contains(&buffer, AUTODESK_INVENTOR_PART_CLSID) {
            Self::AutodeskInventorPart
        } else if contains(&buffer, AUTODESK_INVENTOR_PRESENTATION_CLSID) {
            Self::AutodeskInventorPresentation
        } else if contains(&buffer, MICROSOFT_EXCEL_SPREADSHEET_CLSID_1)
            || contains(&buffer, MICROSOFT_EXCEL_SPREADSHEET_CLSID_2)
        {
            Self::MicrosoftExcelSpreadsheet
        } else if contains(&buffer, MICROSOFT_POWERPOINT_PRESENTATION_CLSID_1)
            || contains(&buffer, MICROSOFT_POWERPOINT_PRESENTATION_CLSID_2)
            || contains(&buffer, MICROSOFT_POWERPOINT_PRESENTATION_CLSID_3)
        {
            Self::MicrosoftPowerpointPresentation
        } else if contains(&buffer, MICROSOFT_PROJECT_PLAN_CLSID) {
            Self::MicrosoftProjectPlan
        } else if contains(&buffer, MICROSOFT_PUBLISHER_DOCUMENT_CLSID) {
            Self::MicrosoftPublisherDocument
        } else if contains(&buffer, MICROSOFT_SOFTWARE_INSTALLER_CLSID) {
            Self::MicrosoftSoftwareInstaller
        } else if contains(&buffer, MICROSOFT_VISIO_DRAWING_CLSID_1)
            || contains(&buffer, MICROSOFT_VISIO_DRAWING_CLSID_2)
        {
            Self::MicrosoftVisioDrawing
        } else if contains(&buffer, MICROSOFT_WORD_DOCUMENT_CLSID_1)
            || contains(&buffer, MICROSOFT_WORD_DOCUMENT_CLSID_2)
        {
            Self::MicrosoftWordDocument
        } else if contains(&buffer, MICROSOFT_WORKS_DATABASE_CLSID_1)
            || contains(&buffer, MICROSOFT_WORKS_DATABASE_CLSID_2)
        {
            Self::MicrosoftWorksDatabase
        } else if contains(&buffer, MICROSOFT_WORKS_WORD_PROCESSOR_CLSID_1)
            || contains(&buffer, MICROSOFT_WORKS_WORD_PROCESSOR_CLSID_2)
            || contains(&buffer, MICROSOFT_WORKS_WORD_PROCESSOR_CLSID_3)
        {
            Self::MicrosoftWorksWordProcessor
        } else if contains(&buffer, SOLIDWORKS_ASSEMBLY_CLSID) {
            Self::SolidworksAssembly
        } else if contains(&buffer, SOLIDWORKS_DRAWING_CLSID) {
            Self::SolidworksDrawing
        } else if contains(&buffer, SOLIDWORKS_PART_CLSID) {
            Self::SolidworksPart
        } else if contains(&buffer, STARCALC_CLSID_1)
            || contains(&buffer, STARCALC_CLSID_2)
            || contains(&buffer, STARCALC_CLSID_3)
        {
            Self::Starcalc
        } else if contains(&buffer, STARCHART_CLSID_1)
            || contains(&buffer, STARCHART_CLSID_2)
            || contains(&buffer, STARCHART_CLSID_3)
        {
            Self::Starchart
        } else if contains(&buffer, STARDRAW_CLSID_1) || contains(&buffer, STARDRAW_CLSID_2) {
            Self::Stardraw
        } else if contains(&buffer, STARIMPRESS_CLSID_1) || contains(&buffer, STARIMPRESS_CLSID_2) {
            Self::Starimpress
        } else if contains(&buffer, STARMATH_CLSID_1)
            || contains(&buffer, STARMATH_CLSID_2)
            || contains(&buffer, STARMATH_CLSID_3)
        {
            Self::Starmath
        } else if contains(&buffer, STARWRITER_CLSID_1)
            || contains(&buffer, STARWRITER_CLSID_2)
            || contains(&buffer, STARWRITER_CLSID_3)
        {
            Self::Starwriter
        } else if contains(&buffer, THREE_DIMENSIONAL_STUDIO_MAX_CLSID) {
            Self::ThreeDimensionalStudioMax
        } else if contains(&buffer, WORDPERFECT_DOCUMENT_CLSID) {
            Self::WordperfectDocument
        } else if contains(&buffer, WORDPERFECT_GRAPHICS_CLSID) {
            Self::WordperfectGraphics
        } else if contains(&buffer, MICROSOFT_WORKS6_SPREADSHEET_FILENAME) {
            Self::MicrosoftWorks6Spreadsheet
        } else if contains(&buffer, MICROSOFT_WORKS_WORD_PROCESSOR_FILENAME) {
            Self::MicrosoftWorksWordProcessor
        } else {
            Self::CompoundFileBinary
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

        // Flags indicating the presence of audio, video and subtitle codecs.
        let mut audio_codec = false;
        let mut video_codec = false;
        let mut subtitle_codec = false;

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
                    } else if codec_id.starts_with("V_") {
                        video_codec = true;
                    } else if codec_id.starts_with("S_") {
                        subtitle_codec = true;
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
        } else if subtitle_codec {
            Self::MatroskaSubtitles
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

        // Flags indicating the presence of audio, video and subtitle tracks.
        let mut audio_track = false;
        let mut video_track = false;
        let mut subtitle_track = false;

        // Iterates through boxes in the reader.
        let mut iteration_count = 0;
        let mut box_header = [0; 8];
        while reader.read_exact(&mut box_header).is_ok() {
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
                        subtitle_track = true
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
        } else if subtitle_track {
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
        // Constants.
        const CENTRAL_DIRECTORY_FILE_HEADER_SIGNATURE: &[u8] = b"\x50\x4B\x01\x02";
        const END_OF_CENTRAL_DIRECTORY_SIGNATURE: &[u8] = b"\x50\x4B\x05\x06";
        const FILE_LIMIT: usize = 4096;

        // Rewinds to the beginning of the stream.
        reader.rewind()?;

        // Gets the stream length.
        let length = reader.seek(SeekFrom::End(0))?;
        reader.rewind()?;

        // Searches for the end of central directory.
        let mut buffer = [0; 4];
        let mut pos = length.saturating_sub(22);
        while pos >= length.saturating_sub(22 + u16::MAX as u64)
            && buffer != END_OF_CENTRAL_DIRECTORY_SIGNATURE
        {
            reader.seek(SeekFrom::Start(pos))?;
            reader.read_exact(&mut buffer)?;
            pos = match pos.checked_sub(1) {
                Some(position) => position,
                None => break,
            }
        }

        // Reads the start of central directory offset.
        reader.seek(SeekFrom::Current(12))?;
        let mut buffer = [0; 4];
        reader.read_exact(&mut buffer)?;
        let offset = u32::from_le_bytes(buffer);

        // Seeks to the start of central directory.
        reader.seek(SeekFrom::Start(offset as u64))?;

        // Sets the default variant.
        let mut format = Self::Zip;

        // Browses central directory file headers.
        let mut buffer = [0; 4];
        let mut file_count = 0;
        while file_count < FILE_LIMIT
            && reader.read_exact(&mut buffer).is_ok()
            && buffer == CENTRAL_DIRECTORY_FILE_HEADER_SIGNATURE
        {
            // Reads compressed size.
            reader.seek(SeekFrom::Current(16))?;
            let mut buffer = [0; 4];
            reader.read_exact(&mut buffer)?;
            let compressed_size = u32::from_le_bytes(buffer);

            // Reads uncompressed size.
            let mut buffer = [0; 4];
            reader.read_exact(&mut buffer)?;
            let uncompressed_size = u32::from_le_bytes(buffer);

            // Reads filename length.
            let mut buffer = [0; 2];
            reader.read_exact(&mut buffer)?;
            let filename_length = u16::from_le_bytes(buffer);

            // Reads extra field length.
            let mut buffer = [0; 2];
            reader.read_exact(&mut buffer)?;
            let extra_field_length = u16::from_le_bytes(buffer);

            // Reads file comment length.
            let mut buffer = [0; 2];
            reader.read_exact(&mut buffer)?;
            let file_comment_length = u16::from_le_bytes(buffer);

            // Reads relative offset of local file header.
            reader.seek(SeekFrom::Current(8))?;
            let mut buffer = [0; 4];
            reader.read_exact(&mut buffer)?;
            let offset = u32::from_le_bytes(buffer);

            // Reads filename.
            let mut buffer = vec![0; filename_length as usize];
            reader.read_exact(&mut buffer)?;
            let filename = String::from_utf8_lossy(&buffer).to_string();

            // Checks filename.
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

                    // Reads filename length.
                    let mut buffer = [0; 2];
                    reader.read_exact(&mut buffer)?;
                    let filename_length = u16::from_le_bytes(buffer);

                    // Reads extra field length.
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

/// Checks if the `data` array contains the `target` sequence using the Boyer-Moore algorithm.
#[cfg(any(
    feature = "reader-asf",
    feature = "reader-cfb",
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
