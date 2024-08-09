use file_format::FileFormat;

#[test]
#[cfg(feature = "from-media-type")]
fn test_adaptable_scalable_texture_compression() {
    let fmt = FileFormat::from_media_type("image/x-astc");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AdaptableScalableTextureCompression)), "{:?} does not contain {}", fmt, FileFormat::AdaptableScalableTextureCompression);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_adobe_illustrator_artwork() {
    let fmt = FileFormat::from_media_type("application/vnd.adobe.illustrator");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AdobeIllustratorArtwork)), "{:?} does not contain {}", fmt, FileFormat::AdobeIllustratorArtwork);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_adobe_photoshop_document() {
    let fmt = FileFormat::from_media_type("image/vnd.adobe.photoshop");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AdobePhotoshopDocument)), "{:?} does not contain {}", fmt, FileFormat::AdobePhotoshopDocument);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_animated_portable_network_graphics() {
    let fmt = FileFormat::from_media_type("image/apng");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AnimatedPortableNetworkGraphics)), "{:?} does not contain {}", fmt, FileFormat::AnimatedPortableNetworkGraphics);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_apple_icon_image() {
    let fmt = FileFormat::from_media_type("image/x-icns");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AppleIconImage)), "{:?} does not contain {}", fmt, FileFormat::AppleIconImage);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_av1_image_file_format() {
    let fmt = FileFormat::from_media_type("image/avif");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Av1ImageFileFormat)), "{:?} does not contain {}", fmt, FileFormat::Av1ImageFileFormat);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_av1_image_file_format_sequence() {
    let fmt = FileFormat::from_media_type("image/avif-sequence");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Av1ImageFileFormatSequence)), "{:?} does not contain {}", fmt, FileFormat::Av1ImageFileFormatSequence);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_better_portable_graphics() {
    let fmt = FileFormat::from_media_type("image/bpg");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::BetterPortableGraphics)), "{:?} does not contain {}", fmt, FileFormat::BetterPortableGraphics);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_canon_raw() {
    let fmt = FileFormat::from_media_type("image/x-canon-crw");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::CanonRaw)), "{:?} does not contain {}", fmt, FileFormat::CanonRaw);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_canon_raw2() {
    let fmt = FileFormat::from_media_type("image/x-canon-cr2");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::CanonRaw2)), "{:?} does not contain {}", fmt, FileFormat::CanonRaw2);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_canon_raw3() {
    let fmt = FileFormat::from_media_type("image/x-canon-cr3");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::CanonRaw3)), "{:?} does not contain {}", fmt, FileFormat::CanonRaw3);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_cineon(){
    let fmt = FileFormat::from_media_type("image/cineon");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Cineon)), "{:?} does not contain {}", fmt, FileFormat::Cineon);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_digital_picture_exchange() {
    let fmt = FileFormat::from_media_type("image/x-dpx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::DigitalPictureExchange)), "{:?} does not contain {}", fmt, FileFormat::DigitalPictureExchange);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_encapsulated_postscript() {
    let fmt = FileFormat::from_media_type("application/eps");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::EncapsulatedPostscript)), "{:?} does not contain {}", fmt, FileFormat::EncapsulatedPostscript);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_experimental_computing_facility() {
    let fmt = FileFormat::from_media_type("image/x-xcf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ExperimentalComputingFacility)), "{:?} does not contain {}", fmt, FileFormat::ExperimentalComputingFacility);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_farbfeld(){
    let fmt = FileFormat::from_media_type("image/x-ff");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Farbfeld)), "{:?} does not contain {}", fmt, FileFormat::Farbfeld);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_free_lossless_image_format() {
    let fmt = FileFormat::from_media_type("image/flif");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::FreeLosslessImageFormat)), "{:?} does not contain {}", fmt, FileFormat::FreeLosslessImageFormat);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_fujifilm_raw() {
    let fmt = FileFormat::from_media_type("image/x-fuji-raf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::FujifilmRaw)), "{:?} does not contain {}", fmt, FileFormat::FujifilmRaw);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_graphics_interchange_format() {
    let fmt = FileFormat::from_media_type("image/gif");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::GraphicsInterchangeFormat)), "{:?} does not contain {}", fmt, FileFormat::GraphicsInterchangeFormat);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_high_efficiency_image_coding() {
    let fmt = FileFormat::from_media_type("image/heic");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::HighEfficiencyImageCoding)), "{:?} does not contain {}", fmt, FileFormat::HighEfficiencyImageCoding);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_high_efficiency_image_coding_sequence() {
    let fmt = FileFormat::from_media_type("image/heic-sequence");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::HighEfficiencyImageCodingSequence)), "{:?} does not contain {}", fmt, FileFormat::HighEfficiencyImageCodingSequence);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_high_efficiency_image_file_format() {
    let fmt = FileFormat::from_media_type("image/heif");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::HighEfficiencyImageFileFormat)), "{:?} does not contain {}", fmt, FileFormat::HighEfficiencyImageFileFormat);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_high_efficiency_image_file_format_sequence() {
    let fmt = FileFormat::from_media_type("image/heif-sequence");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::HighEfficiencyImageFileFormatSequence)), "{:?} does not contain {}", fmt, FileFormat::HighEfficiencyImageFileFormatSequence);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_joint_photographic_experts_group() {
    let fmt = FileFormat::from_media_type("image/jpeg");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::JointPhotographicExpertsGroup)), "{:?} does not contain {}", fmt, FileFormat::JointPhotographicExpertsGroup);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_jpeg2000_codestream() {
    let fmt = FileFormat::from_media_type("image/x-jp2-codestream");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Jpeg2000Codestream)), "{:?} does not contain {}", fmt, FileFormat::Jpeg2000Codestream);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_jpeg2000_part1() {
    let fmt = FileFormat::from_media_type("image/jp2");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Jpeg2000Part1)), "{:?} does not contain {}", fmt, FileFormat::Jpeg2000Part1);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_jpeg2000_part2() {
    let fmt = FileFormat::from_media_type("image/jpx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Jpeg2000Part2)), "{:?} does not contain {}", fmt, FileFormat::Jpeg2000Part2);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_jpeg2000_part6() {
    let fmt = FileFormat::from_media_type("image/jpm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Jpeg2000Part6)), "{:?} does not contain {}", fmt, FileFormat::Jpeg2000Part6);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_jpeg_extended_range() {
    let fmt = FileFormat::from_media_type("image/jxr");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::JpegExtendedRange)), "{:?} does not contain {}", fmt, FileFormat::JpegExtendedRange);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_jpeg_ls() {
    let fmt = FileFormat::from_media_type("image/jls");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::JpegLs)), "{:?} does not contain {}", fmt, FileFormat::JpegLs);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_jpeg_network_graphics() {
    let fmt = FileFormat::from_media_type("image/x-jng");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::JpegNetworkGraphics)), "{:?} does not contain {}", fmt, FileFormat::JpegNetworkGraphics);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_jpeg_xl() {
    let fmt = FileFormat::from_media_type("image/jxl");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::JpegXl)), "{:?} does not contain {}", fmt, FileFormat::JpegXl);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_khronos_texture() {
    let fmt = FileFormat::from_media_type("image/ktx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::KhronosTexture)), "{:?} does not contain {}", fmt, FileFormat::KhronosTexture);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_khronos_texture2() {
    let fmt = FileFormat::from_media_type("image/ktx2");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::KhronosTexture2)), "{:?} does not contain {}", fmt, FileFormat::KhronosTexture2);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_magick_image_file_format() {
    let fmt = FileFormat::from_media_type("image/x-miff");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MagickImageFileFormat)), "{:?} does not contain {}", fmt, FileFormat::MagickImageFileFormat);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_microsoft_directdraw_surface() {
    let fmt = FileFormat::from_media_type("image/vnd.ms-dds");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftDirectdrawSurface)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftDirectdrawSurface);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_multiple_image_network_graphics() {
    let fmt = FileFormat::from_media_type("image/x-mng");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MultipleImageNetworkGraphics)), "{:?} does not contain {}", fmt, FileFormat::MultipleImageNetworkGraphics);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_nikon_electronic_file() {
    let fmt = FileFormat::from_media_type("image/x-nikon-nef");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::NikonElectronicFile)), "{:?} does not contain {}", fmt, FileFormat::NikonElectronicFile);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_olympus_raw_format() {
    let fmt = FileFormat::from_media_type("image/x-olympus-orf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OlympusRawFormat)), "{:?} does not contain {}", fmt, FileFormat::OlympusRawFormat);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_opendocument_graphics() {
    let fmt = FileFormat::from_media_type("application/vnd.oasis.opendocument.graphics");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OpendocumentGraphics)), "{:?} does not contain {}", fmt, FileFormat::OpendocumentGraphics);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_opendocument_graphics_template() {
    let fmt = FileFormat::from_media_type("application/vnd.oasis.opendocument.graphics-template");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OpendocumentGraphicsTemplate)), "{:?} does not contain {}", fmt, FileFormat::OpendocumentGraphicsTemplate);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_openexr(){
    let fmt = FileFormat::from_media_type("image/x-exr");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Openexr)), "{:?} does not contain {}", fmt, FileFormat::Openexr);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_openraster(){
    let fmt = FileFormat::from_media_type("image/openraster");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Openraster)), "{:?} does not contain {}", fmt, FileFormat::Openraster);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_panasonic_raw() {
    let fmt = FileFormat::from_media_type("image/x-panasonic-rw2");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PanasonicRaw)), "{:?} does not contain {}", fmt, FileFormat::PanasonicRaw);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_picture_exchange() {
    let fmt = FileFormat::from_media_type("image/x-pcx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PictureExchange)), "{:?} does not contain {}", fmt, FileFormat::PictureExchange);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_portable_arbitrary_map() {
    let fmt = FileFormat::from_media_type("image/x-portable-arbitrarymap");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PortableArbitraryMap)), "{:?} does not contain {}", fmt, FileFormat::PortableArbitraryMap);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_portable_bitmap() {
    let fmt = FileFormat::from_media_type("image/x-portable-bitmap");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PortableBitmap)), "{:?} does not contain {}", fmt, FileFormat::PortableBitmap);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_portable_floatmap() {
    let fmt = FileFormat::from_media_type("image/x-pfm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PortableFloatmap)), "{:?} does not contain {}", fmt, FileFormat::PortableFloatmap);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_portable_graymap() {
    let fmt = FileFormat::from_media_type("image/x-portable-graymap");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PortableGraymap)), "{:?} does not contain {}", fmt, FileFormat::PortableGraymap);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_portable_network_graphics() {
    let fmt = FileFormat::from_media_type("image/png");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PortableNetworkGraphics)), "{:?} does not contain {}", fmt, FileFormat::PortableNetworkGraphics);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_portable_pixmap() {
    let fmt = FileFormat::from_media_type("image/x-portable-pixmap");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PortablePixmap)), "{:?} does not contain {}", fmt, FileFormat::PortablePixmap);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_quite_ok_image() {
    let fmt = FileFormat::from_media_type("image/x-qoi");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::QuiteOkImage)), "{:?} does not contain {}", fmt, FileFormat::QuiteOkImage);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_radiance_hdr() {
    let fmt = FileFormat::from_media_type("image/vnd.radiance");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::RadianceHdr)), "{:?} does not contain {}", fmt, FileFormat::RadianceHdr);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_scalable_vector_graphics(){
    let fmt = FileFormat::from_media_type("image/svg+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ScalableVectorGraphics)), "{:?} does not contain {}", fmt, FileFormat::ScalableVectorGraphics);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_silicon_graphics_image() {
    let fmt = FileFormat::from_media_type("image/x-sgi");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SiliconGraphicsImage)), "{:?} does not contain {}", fmt, FileFormat::SiliconGraphicsImage);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_sketch(){
    let fmt = FileFormat::from_media_type("image/x-sketch");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Sketch)), "{:?} does not contain {}", fmt, FileFormat::Sketch);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_sketch43(){
    let fmt = FileFormat::from_media_type("image/x-sketch");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Sketch43)), "{:?} does not contain {}", fmt, FileFormat::Sketch43);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_stardraw(){
    let fmt = FileFormat::from_media_type("application/vnd.stardivision.draw");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Stardraw)), "{:?} does not contain {}", fmt, FileFormat::Stardraw);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_sun_xml_draw() {
    let fmt = FileFormat::from_media_type("application/vnd.sun.xml.draw");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SunXmlDraw)), "{:?} does not contain {}", fmt, FileFormat::SunXmlDraw);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_sun_xml_draw_template() {
    let fmt = FileFormat::from_media_type("application/vnd.sun.xml.draw.template");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SunXmlDrawTemplate)), "{:?} does not contain {}", fmt, FileFormat::SunXmlDrawTemplate);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_tag_image_file_format() {
    let fmt = FileFormat::from_media_type("image/tiff");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::TagImageFileFormat)), "{:?} does not contain {}", fmt, FileFormat::TagImageFileFormat);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_webp(){
    let fmt = FileFormat::from_media_type("image/webp");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Webp)), "{:?} does not contain {}", fmt, FileFormat::Webp);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_windows_animated_cursor() {
    let fmt = FileFormat::from_media_type("application/x-navi-animation");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WindowsAnimatedCursor)), "{:?} does not contain {}", fmt, FileFormat::WindowsAnimatedCursor);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_windows_bitmap() {
    let fmt = FileFormat::from_media_type("image/bmp");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WindowsBitmap)), "{:?} does not contain {}", fmt, FileFormat::WindowsBitmap);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_windows_cursor() {
    let fmt = FileFormat::from_media_type("image/x-icon");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WindowsCursor)), "{:?} does not contain {}", fmt, FileFormat::WindowsCursor);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_windows_icon() {
    let fmt = FileFormat::from_media_type("image/x-icon");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WindowsIcon)), "{:?} does not contain {}", fmt, FileFormat::WindowsIcon);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_windows_metafile() {
    let fmt = FileFormat::from_media_type("image/wmf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WindowsMetafile)), "{:?} does not contain {}", fmt, FileFormat::WindowsMetafile);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_word_perfect_graphics(){
    let fmt = FileFormat::from_media_type("application/vnd.wordperfect");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WordperfectGraphics)), "{:?} does not contain {}", fmt, FileFormat::WordperfectGraphics);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_x_pixmap() {
    let fmt = FileFormat::from_media_type("image/x-xpixmap");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::XPixmap)), "{:?} does not contain {}", fmt, FileFormat::XPixmap);
}

