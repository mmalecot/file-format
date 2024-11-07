#[cfg(feature = "reader-asf")]
pub(super) mod asf {
    use crate::FileFormat;

    // Maximum number of descriptors that can be processed by the reader.
    pub(crate) const DESCRIPTOR_LIMIT: usize = 32;

    // Maximum size of a descriptor name that can be handled by the reader.
    pub(crate) const DESCRIPTOR_NAME_LIMIT: usize = 64;

    // Maximum number of objects that can be processed by the reader.
    pub(crate) const OBJECT_LIMIT: usize = 256;

    // GUID for extended content description object.
    pub(crate) const EXTENDED_CONTENT_DESCRIPTION_OBJECT_GUID: &str =
        "d2d0a440-e307-11d2-97f0-00a0c95ea850";

    // GUID for stream properties object.
    pub(crate) const STREAM_PROPERTIES_OBJECT_GUID: &str = "b7dc0791-a9b7-11cf-8ee6-00c00c205365";

    // GUID for audio media.
    pub(crate) const AUDIO_MEDIA_GUID: &str = "f8699e40-5b4d-11cf-a8fd-00805f5c442b";

    // GUID for video media.
    pub(crate) const VIDEO_MEDIA_GUID: &str = "bc19efc0-5b4d-11cf-a8fd-00805f5c442b";

    // Stream type.
    pub(crate) enum Stream {
        Video,
        Audio,
        Other,
    }

    // Determines the file format based on the identified stream.
    pub(crate) fn determine_file_format(stream: Stream) -> FileFormat {
        use FileFormat as F;
        match stream {
            Stream::Video => F::WindowsMediaVideo,
            Stream::Audio => F::WindowsMediaAudio,
            Stream::Other => F::AdvancedSystemsFormat,
        }
    }
}

#[cfg(feature = "reader-cfb")]
pub(super) mod cfb {
    use crate::FileFormat;

    // Determines the file format based on the CLSID.
    pub(crate) fn determine_file_format(clsid: &str) -> Option<FileFormat> {
        use FileFormat as F;
        match clsid {
            "e60f81e1-49b3-11d0-93c3-7e0706000000" => Some(F::AutodeskInventorAssembly),
            "bbf9fdf1-52dc-11d0-8c04-0800090be8ec" => Some(F::AutodeskInventorDrawing),
            "4d29b490-49b2-11d0-93c3-7e0706000000" => Some(F::AutodeskInventorPart),
            "76283a80-50dd-11d3-a7e3-00c04f79d7bc" => Some(F::AutodeskInventorPresentation),
            "402efe62-1999-101b-99ae-04021c007002" => Some(F::CorelPresentations7),
            "597caa70-72aa-11cf-831e-524153480000" => Some(F::FlashProject),
            "b4f235fe-dca6-4803-b7b2-c25a453c836b" => Some(F::FlashProject),
            "00020810-0000-0000-c000-000000000046" => Some(F::MicrosoftExcelSpreadsheet),
            "00020820-0000-0000-c000-000000000046" => Some(F::MicrosoftExcelSpreadsheet),
            "00044851-0000-0000-c000-000000000046" => Some(F::MicrosoftPowerpointPresentation),
            "64818d10-4f9b-11cf-86ea-00aa00b929e8" => Some(F::MicrosoftPowerpointPresentation),
            "ea7bae70-fb3b-11cd-a903-00aa00510ea3" => Some(F::MicrosoftPowerpointPresentation),
            "74b78f3a-c8c8-11d1-be11-00c04fb6faf1" => Some(F::MicrosoftProjectPlan),
            "00021201-0000-0000-00c0-000000000046" => Some(F::MicrosoftPublisherDocument),
            "000c1084-0000-0000-c000-000000000046" => Some(F::MicrosoftSoftwareInstaller),
            "00021a13-0000-0000-c000-000000000046" => Some(F::MicrosoftVisioDrawing),
            "00021a14-0000-0000-c000-000000000046" => Some(F::MicrosoftVisioDrawing),
            "00020900-0000-0000-c000-000000000046" => Some(F::MicrosoftWordDocument),
            "00020906-0000-0000-c000-000000000046" => Some(F::MicrosoftWordDocument),
            "00021303-0000-0000-c000-000000000046" => Some(F::MicrosoftWorksDatabase),
            "28cddbc3-0ae2-11ce-a29a-00aa004a1a72" => Some(F::MicrosoftWorksDatabase),
            "00021302-0000-0000-c000-000000000046" => Some(F::MicrosoftWorksWordProcessor),
            "0ea45ab2-9e0a-11d1-a407-00c04fb932ba" => Some(F::MicrosoftWorksWordProcessor),
            "28cddbc2-0ae2-11ce-a29a-00aa004a1a72" => Some(F::MicrosoftWorksWordProcessor),
            "83a33d36-27c5-11ce-bfd4-00400513bb57" => Some(F::SolidworksAssembly),
            "83a33d34-27c5-11ce-bfd4-00400513bb57" => Some(F::SolidworksDrawing),
            "83a33d30-27c5-11ce-bfd4-00400513bb57" => Some(F::SolidworksPart),
            "3f543fa0-b6a6-101b-9961-04021c007002" => Some(F::Starcalc),
            "6361d441-4235-11d0-89cb-008029e4b0b1" => Some(F::Starcalc),
            "c6a5b861-85d6-11d1-89cb-008029e4b0b1" => Some(F::Starcalc),
            "02b3b7e0-4225-11d0-89ca-008029e4b0b1" => Some(F::Starchart),
            "bf884321-85dd-11d1-89d0-008029e4b0b1" => Some(F::Starchart),
            "fb9c99e0-2c6d-101c-8e2c-00001b4cc711" => Some(F::Starchart),
            "2e8905a0-85bd-11d1-89d0-008029e4b0b1" => Some(F::Stardraw),
            "af10aae0-b36d-101b-9961-04021c007002" => Some(F::Stardraw),
            "012d3cc0-4216-11d0-89cb-008029e4b0b1" => Some(F::Starimpress),
            "565c7221-85bc-11d1-89d0-008029e4b0b1" => Some(F::Starimpress),
            "02b3b7e1-4225-11d0-89ca-008029e4b0b1" => Some(F::Starmath),
            "d4590460-35fd-101c-b12a-04021c007002" => Some(F::Starmath),
            "ffb5e640-85de-11d1-89d0-008029e4b0b1" => Some(F::Starmath),
            "8b04e9b0-420e-11d0-a45e-00a0249d57b1" => Some(F::Starwriter),
            "c20cf9d1-85ae-11d1-aab4-006097da561a" => Some(F::Starwriter),
            "dc5c7e40-b35c-101b-9961-04021c007002" => Some(F::Starwriter),
            "1cdd8c7b-81c0-45a0-9fed-04143144cc1e" => Some(F::ThreeDimensionalStudioMax),
            "519873ff-2dad-0220-1937-0000929679cd" => Some(F::WordperfectDocument),
            "402efe60-1999-101b-99ae-04021c007002" => Some(F::WordperfectGraphics),
            _ => None,
        }
    }
}

#[cfg(feature = "reader-ebml")]
pub(super) mod ebml {
    use crate::FileFormat;

    // Maximum number of EBML elements that can be processed by the reader.
    pub(crate) const ELEMENT_LIMIT: usize = 256;

    // Maximum size of a Codec ID that can be processed by the reader.
    pub(crate) const CODEC_ID_LIMIT: usize = 64;

    // Maximum size of a DocType that can be processed by the reader.
    pub(crate) const DOC_TYPE_LIMIT: usize = 8;

    // DocType element ID.
    pub(crate) const DOC_TYPE_ELEMENT_ID: u32 = 0x4282;

    // EBML element ID.
    pub(crate) const EBML_ELEMENT_ID: u32 = 0x1A45DFA3;

    // Cluster element ID.
    pub(crate) const CLUSTER_ELEMENT_ID: u32 = 0x1F43B675;

    // CodecID element ID.
    pub(crate) const CODEC_ID_ELEMENT_ID: u32 = 0x86;

    // Segment element ID.
    pub(crate) const SEGMENT_ELEMENT_ID: u32 = 0x18538067;

    // StereoMode element ID.
    pub(crate) const STEREO_MODE_ELEMENT_ID: u32 = 0x53B8;

    // Tracks element ID.
    pub(crate) const TRACKS_ELEMENT_ID: u32 = 0x1654AE6B;

    // TrackEntry element ID.
    pub(crate) const TRACK_ENTRY_ELEMENT_ID: u32 = 0xAE;

    // Video element ID.
    pub(crate) const VIDEO_ELEMENT_ID: u32 = 0xE0;

    pub(crate) enum Track {
        Video,
        Audio,
        Subtitle,
        Other,
    }

    // Determines the file format based on the identified track.
    pub(crate) fn determine_file_format(track: Track) -> FileFormat {
        use FileFormat as F;
        match track {
            Track::Video => F::MatroskaVideo,
            Track::Audio => F::MatroskaAudio,
            Track::Subtitle => F::MatroskaSubtitles,
            Track::Other => F::ExtensibleBinaryMetaLanguage,
        }
    }
}

#[cfg(feature = "reader-mp4")]
pub(super) mod mp4 {
    use crate::FileFormat;

    // Maximum number of boxes that can be processed by the reader.
    pub(crate) const BOX_LIMIT: usize = 256;

    pub(crate) enum Track {
        Video,
        Audio,
        Subtitle,
        Other,
    }

    // Determines the file format based on the identified track.
    pub(crate) fn determine_file_format(track: Track) -> FileFormat {
        use FileFormat as F;
        match track {
            Track::Video => F::Mpeg4Part14Video,
            Track::Audio => F::Mpeg4Part14Audio,
            Track::Subtitle => F::Mpeg4Part14Subtitles,
            Track::Other => F::Mpeg4Part14,
        }
    }
}

#[cfg(feature = "reader-pdf")]
pub(super) mod pdf {
    // Maximum number of bytes that can be processed by the reader (32 MB).
    pub(crate) const READ_LIMIT: usize = 33_554_432;

    // Size of each chunk to read (32 KB).
    pub(crate) const CHUNK_SIZE: usize = 32_768;

    // Size of overlap to keep between chunks.
    pub(crate) const OVERLAP_SIZE: usize = AI_MARKER.len() - 1;

    // Marker for the AI file format.
    pub(crate) const AI_MARKER: &[u8] = b"AIPrivateData";
}

#[cfg(feature = "reader-rm")]
pub(super) mod rm {
    use crate::FileFormat;

    // Maximum number of chunks that can be processed by the reader.
    pub(crate) const CHUNK_LIMIT: usize = 64;

    // Stream type.
    pub(crate) enum Stream {
        Video,
        Audio,
        Other,
    }

    // Determines the file format based on the identified stream.
    pub(crate) fn determine_file_format(stream: Stream) -> FileFormat {
        use FileFormat as F;
        match stream {
            Stream::Video => F::Realvideo,
            Stream::Audio => F::Realaudio,
            Stream::Other => F::Realmedia,
        }
    }
}

#[cfg(feature = "reader-sqlite3")]
pub(super) mod sqlite3 {
    use crate::FileFormat;

    use super::FindBytes;

    // Marker for the Sketch file format.
    const SKETCH_MARKER: &[u8] = b"com.bohemiancoding.sketch3";

    // Checks if the buffer holds the Sketch file format marker.
    pub(crate) fn check_if_buffer_holds_sketch_marker(
        buf: &[u8],
        nread: usize,
    ) -> Option<FileFormat> {
        if buf[..nread].holds(SKETCH_MARKER) {
            Some(FileFormat::Sketch)
        } else {
            None
        }
    }
}

#[cfg(feature = "reader-txt")]
pub(super) mod txt {
    // Maximum number of lines that can be processed by the reader.
    pub(crate) const LINE_LIMIT: usize = 16;

    // Maximum number of bytes that can be processed by the reader (64 KB).
    pub(crate) const READ_LIMIT: u64 = 65_536;

    // Checks if character is control character other than whitespace.
    pub(crate) fn is_non_whitespace_control_char(char: &char) -> bool {
        char.is_control() && !char.is_whitespace()
    }
}

#[cfg(feature = "reader-xml")]
pub(super) mod xml {
    use crate::{readers::common::FindBytes, FileFormat};

    // Checks if the buffer holds markers indicating the presence of various file formats.
    pub(crate) fn check_if_buffer_holds_markers(buf: &[u8]) -> FileFormat {
        use FileFormat as F;
        if buf.holds("<abiword template=\"false\"") {
            F::Abiword
        } else if buf.holds("<abiword template=\"true\"") {
            F::AbiwordTemplate
        } else if buf.holds("<amf") {
            F::AdditiveManufacturingFormat
        } else if buf.holds("<ASX") || buf.holds("<asx") {
            F::AdvancedStreamRedirector
        } else if buf.holds("<feed") {
            F::Atom
        } else if buf.holds("<COLLADA") {
            F::CollaborativeDesignActivity
        } else if buf.holds("<mxfile") {
            F::Drawio
        } else if buf.holds("<X3D") {
            F::Extensible3d
        } else if buf.holds("<xsl") {
            F::ExtensibleStylesheetLanguageTransformations
        } else if buf.holds("<FictionBook") {
            F::Fictionbook
        } else if buf.holds("<gml") {
            F::GeographyMarkupLanguage
        } else if buf.holds("<gpx") {
            F::GpsExchangeFormat
        } else if buf.holds("<kml") {
            F::KeyholeMarkupLanguage
        } else if buf.holds("<math") {
            F::MathematicalMarkupLanguage
        } else if buf.holds("<MPD") {
            F::MpegDashMpd
        } else if buf.holds("<score-partwise") {
            F::Musicxml
        } else if buf.holds("<rss") {
            F::ReallySimpleSyndication
        } else if buf.holds("<SVG") || buf.holds("<svg") {
            F::ScalableVectorGraphics
        } else if buf.holds("<soap") {
            F::SimpleObjectAccessProtocol
        } else if buf.holds("<map") {
            F::TiledMapXml
        } else if buf.holds("<tileset") {
            F::TiledTilesetXml
        } else if buf.holds("<tt") && buf.holds("xmlns=\"http://www.w3.org/ns/ttml\"") {
            F::TimedTextMarkupLanguage
        } else if buf.holds("<TrainingCenterDatabase") {
            F::TrainingCenterXml
        } else if buf.holds("<uof:UOF") && buf.holds("uof:mimetype=\"vnd.uof.presentation\"") {
            F::UniformOfficeFormatPresentation
        } else if buf.holds("<uof:UOF") && buf.holds("uof:mimetype=\"vnd.uof.spreadsheet\"") {
            F::UniformOfficeFormatSpreadsheet
        } else if buf.holds("<uof:UOF") && buf.holds("uof:mimetype=\"vnd.uof.text\"") {
            F::UniformOfficeFormatText
        } else if buf.holds("<USFSubtitles") {
            F::UniversalSubtitleFormat
        } else if buf.holds("<xliff") {
            F::XmlLocalizationInterchangeFileFormat
        } else if buf.holds("<playlist") {
            F::XmlShareablePlaylistFormat
        } else {
            F::ExtensibleMarkupLanguage
        }
    }
}

#[cfg(feature = "reader-zip")]
pub(super) mod zip {
    use crate::FileFormat;

    // Maximum number of entries that can be processed by the reader.
    pub(crate) const ENTRY_LIMIT: usize = 1024;

    // Signature of the ZIP64 end of central directory locator.
    pub(crate) const EOCD64_LOCATOR_SIGNATURE: &[u8] = b"PK\x06\x07";

    // Signature of the end of central directory record.
    pub(crate) const EOCD_SIGNATURE: &[u8] = b"PK\x05\x06";

    // Size of the ZIP64 end of central directory locator.
    pub(crate) const EOCD64_LOCATOR_SIZE: usize = 20;

    // Maximum size of the end of central directory record.
    pub(crate) const EOCD_MAX_SIZE: usize = EOCD_MIN_SIZE + u16::MAX as usize;

    // Minimum size of the end of central directory record.
    pub(crate) const EOCD_MIN_SIZE: usize = 22;

    // Checks the trimmed data.
    pub(crate) fn check_trimmed_data(data: String) -> FileFormat {
        use FileFormat as F;
        match data.trim() {
            "application/epub+zip" => F::ElectronicPublication,
            "application/vnd.adobe.indesign-idml-package" => F::IndesignMarkupLanguage,
            "application/vnd.oasis.opendocument.base"
            | "application/vnd.oasis.opendocument.database" => F::OpendocumentDatabase,
            "application/vnd.oasis.opendocument.formula" => F::OpendocumentFormula,
            "application/vnd.oasis.opendocument.formula-template" => F::OpendocumentFormulaTemplate,
            "application/vnd.oasis.opendocument.graphics" => F::OpendocumentGraphics,
            "application/vnd.oasis.opendocument.graphics-template" => {
                F::OpendocumentGraphicsTemplate
            }
            "application/vnd.oasis.opendocument.presentation" => F::OpendocumentPresentation,
            "application/vnd.oasis.opendocument.presentation-template" => {
                F::OpendocumentPresentationTemplate
            }
            "application/vnd.oasis.opendocument.spreadsheet" => F::OpendocumentSpreadsheet,
            "application/vnd.oasis.opendocument.spreadsheet-template" => {
                F::OpendocumentSpreadsheetTemplate
            }
            "application/vnd.oasis.opendocument.text" => F::OpendocumentText,
            "application/vnd.oasis.opendocument.text-master" => F::OpendocumentTextMaster,
            "application/vnd.oasis.opendocument.text-master-template" => {
                F::OpendocumentTextMasterTemplate
            }
            "application/vnd.oasis.opendocument.text-template" => F::OpendocumentTextTemplate,
            "application/vnd.recordare.musicxml" => F::MusicxmlZip,
            "application/vnd.sun.xml.calc" => F::SunXmlCalc,
            "application/vnd.sun.xml.calc.template" => F::SunXmlCalcTemplate,
            "application/vnd.sun.xml.draw" => F::SunXmlDraw,
            "application/vnd.sun.xml.draw.template" => F::SunXmlDrawTemplate,
            "application/vnd.sun.xml.impress" => F::SunXmlImpress,
            "application/vnd.sun.xml.impress.template" => F::SunXmlImpressTemplate,
            "application/vnd.sun.xml.math" => F::SunXmlMath,
            "application/vnd.sun.xml.writer" => F::SunXmlWriter,
            "application/vnd.sun.xml.writer.global" => F::SunXmlWriterGlobal,
            "application/vnd.sun.xml.writer.template" => F::SunXmlWriterTemplate,
            "image/openraster" => F::Openraster,
            _ => F::Zip,
        }
    }

    // Checks the filename.
    pub(crate) fn check_filename(filename: &str) -> Option<FileFormat> {
        use FileFormat as F;
        match filename {
            "AndroidManifest.xml" => Some(F::AndroidPackage),
            "AppManifest.xaml" => Some(F::Xap),
            "AppxManifest.xml" => Some(F::WindowsAppPackage),
            "AppxMetadata/AppxBundleManifest.xml" => Some(F::WindowsAppBundle),
            "BundleConfig.pb" => Some(F::AndroidAppBundle),
            "DOMDocument.xml" => Some(F::FlashCs5Project),
            "META-INF/AIR/application.xml" => Some(F::AdobeIntegratedRuntime),
            "META-INF/application.xml" => Some(F::EnterpriseApplicationArchive),
            "META-INF/mozilla.rsa" => Some(F::Xpinstall),
            "WEB-INF/web.xml" => Some(F::WebApplicationArchive),
            "doc.kml" => Some(F::KeyholeMarkupLanguageZip),
            "document.json" => Some(F::Sketch43),
            "extension.vsixmanifest" => Some(F::MicrosoftVisualStudioExtension),
            _ => None,
        }
    }

    pub(crate) fn check_filename_starts_with(filename: String) -> Option<FileFormat> {
        use FileFormat as F;
        if filename.starts_with("Fusion[Active]/") {
            Some(F::Autodesk123d)
        } else if filename.starts_with("circuitdiagram/") {
            Some(F::CircuitDiagramDocument)
        } else if filename.starts_with("dwf/") {
            Some(F::DesignWebFormatXps)
        } else if filename.ends_with(".fb2") && !filename.contains('/') {
            Some(F::FictionbookZip)
        } else if filename.starts_with("FusionAssetName[Active]/") {
            Some(F::Fusion360)
        } else if filename.starts_with("Payload/") && filename.contains(".app/") {
            Some(F::IosAppStorePackage)
        } else if filename.starts_with("word/") {
            Some(F::OfficeOpenXmlDocument)
        } else if filename.starts_with("visio/") {
            Some(F::OfficeOpenXmlDrawing)
        } else if filename.starts_with("ppt/") {
            Some(F::OfficeOpenXmlPresentation)
        } else if filename.starts_with("xl/") {
            Some(F::OfficeOpenXmlSpreadsheet)
        } else if filename.starts_with("Documents/") && filename.ends_with(".fpage") {
            Some(F::Openxps)
        } else if filename.starts_with("SpaceClaim/") {
            Some(F::SpaceclaimDocument)
        } else if filename.starts_with("3D/") && filename.ends_with(".model") {
            Some(F::ThreeDimensionalManufacturingFormat)
        } else if (filename.ends_with(".usd")
            || filename.ends_with(".usda")
            || filename.ends_with(".usdc"))
            && !filename.contains('/')
        {
            Some(F::UniversalSceneDescriptionZip)
        } else {
            None
        }
    }
}

/// A trait for finding a byte pattern within data.
#[allow(dead_code)]
pub(super) trait FindBytes: AsRef<[u8]> {
    /// Searches for the specified byte pattern and returns the index of the first occurrence.
    fn find<P: AsRef<[u8]>>(&self, pat: P) -> Option<usize> {
        // Retrieves references to data and pattern.
        let data = self.as_ref();
        let pat = pat.as_ref();

        // An empty pattern is always considered to be contained in the data.
        if pat.is_empty() {
            return Some(0);
        }

        // The data is shorter than the pattern, so it cannot contain it.
        if data.len() < pat.len() {
            return None;
        }

        // Searches for the byte pattern using a modified Boyer-Moore-Horspool algorithm for forward
        // search.
        let mut shift_table = [pat.len(); 256];
        for (index, &byte) in pat.iter().enumerate().take(pat.len() - 1) {
            shift_table[byte as usize] = pat.len() - 1 - index;
        }
        let mut data_index = pat.len() - 1;
        while data_index < data.len() {
            let mut pat_index = pat.len() - 1;
            while pat[pat_index] == data[data_index - (pat.len() - 1 - pat_index)] {
                if pat_index == 0 {
                    return Some(data_index - (pat.len() - 1));
                }
                pat_index -= 1;
            }
            data_index += shift_table[data[data_index] as usize];
        }
        None
    }

    /// Returns `true` if the data holds the specified byte pattern.
    #[inline]
    fn holds<P: AsRef<[u8]>>(&self, pat: P) -> bool {
        self.find(pat).is_some()
    }

    /// Searches for the specified byte pattern and returns the index of the last occurrence.
    fn rfind<P: AsRef<[u8]>>(&self, pat: P) -> Option<usize> {
        // Retrieves references to data and pattern.
        let data = self.as_ref();
        let pat = pat.as_ref();

        // An empty pattern is always considered to be contained in the data.
        if pat.is_empty() {
            return Some(data.len());
        }

        // The data is shorter than the pattern, so it cannot contain it.
        if data.len() < pat.len() {
            return None;
        }

        // Searches for the byte pattern using a modified Boyer-Moore-Horspool algorithm for reverse
        // search.
        let mut shift_table = [pat.len(); 256];
        for (index, &byte) in pat.iter().rev().enumerate().take(pat.len() - 1) {
            shift_table[byte as usize] = pat.len() - 1 - index;
        }
        let mut data_index = data.len() - pat.len();
        loop {
            let mut pat_index = 0;
            while pat[pat_index] == data[data_index + pat_index] {
                if pat_index == pat.len() - 1 {
                    return Some(data_index);
                }
                pat_index += 1;
            }
            data_index = match data_index.checked_sub(shift_table[data[data_index] as usize]) {
                Some(data_index) => data_index,
                _ => break,
            }
        }
        None
    }
}

/// Allows any type `B` that implements the `AsRef<[u8]>` trait to benefit from the additional
/// methods provided by the `FindBytes` trait.
impl<B: AsRef<[u8]> + ?Sized> FindBytes for B {}
