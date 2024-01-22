use file_format::FileFormat;

#[test]
fn test_adaptable_scalable_texture_compression() {
    let fmt = FileFormat::from_file("fixtures/image/sample.astc").unwrap();
    assert_eq!(fmt, FileFormat::AdaptableScalableTextureCompression);
}

#[cfg(feature = "reader-pdf")]
#[test]
fn test_adobe_illustrator_artwork() {
    let fmt = FileFormat::from_file("fixtures/image/sample.ai").unwrap();
    assert_eq!(fmt, FileFormat::AdobeIllustratorArtwork);
}

#[test]
fn test_adobe_photoshop_document() {
    let fmt = FileFormat::from_file("fixtures/image/sample.psd").unwrap();
    assert_eq!(fmt, FileFormat::AdobePhotoshopDocument);
}

#[test]
fn test_animated_portable_network_graphics() {
    let fmt = FileFormat::from_file("fixtures/image/sample.apng").unwrap();
    assert_eq!(fmt, FileFormat::AnimatedPortableNetworkGraphics);
}

#[test]
fn test_apple_icon_image() {
    let fmt = FileFormat::from_file("fixtures/image/sample.icns").unwrap();
    assert_eq!(fmt, FileFormat::AppleIconImage);
}

#[test]
fn test_av1_image_file_format() {
    let fmt = FileFormat::from_file("fixtures/image/sample.avif").unwrap();
    assert_eq!(fmt, FileFormat::Av1ImageFileFormat);
}

#[test]
fn test_av1_image_file_format_sequence() {
    let fmt = FileFormat::from_file("fixtures/image/sample.avifs").unwrap();
    assert_eq!(fmt, FileFormat::Av1ImageFileFormatSequence);
}

#[test]
fn test_better_portable_graphics() {
    let fmt = FileFormat::from_file("fixtures/image/sample.bpg").unwrap();
    assert_eq!(fmt, FileFormat::BetterPortableGraphics);
}

#[test]
fn test_canon_raw() {
    let fmt = FileFormat::from_file("fixtures/image/sample.crw").unwrap();
    assert_eq!(fmt, FileFormat::CanonRaw);
}

#[test]
fn test_canon_raw2() {
    let fmt = FileFormat::from_file("fixtures/image/sample.cr2").unwrap();
    assert_eq!(fmt, FileFormat::CanonRaw2);
}

#[test]
fn test_canon_raw3() {
    let fmt = FileFormat::from_file("fixtures/image/sample.cr3").unwrap();
    assert_eq!(fmt, FileFormat::CanonRaw3);
}

#[test]
fn test_cineon() {
    let fmt = FileFormat::from_file("fixtures/image/sample.cin").unwrap();
    assert_eq!(fmt, FileFormat::Cineon);
}

#[test]
fn test_digital_picture_exchange() {
    let fmt = FileFormat::from_file("fixtures/image/sample.dpx").unwrap();
    assert_eq!(fmt, FileFormat::DigitalPictureExchange);
}

#[test]
fn test_djvu() {
    let fmt = FileFormat::from_file("fixtures/image/sample.djvu").unwrap();
    assert_eq!(fmt, FileFormat::Djvu);
}

#[test]
fn test_encapsulated_postscript() {
    let fmt = FileFormat::from_file("fixtures/image/sample.eps").unwrap();
    assert_eq!(fmt, FileFormat::EncapsulatedPostscript);
}

#[test]
fn test_experimental_computing_facility() {
    let fmt = FileFormat::from_file("fixtures/image/sample.xcf").unwrap();
    assert_eq!(fmt, FileFormat::ExperimentalComputingFacility);
}

#[test]
fn test_farbfeld() {
    let fmt = FileFormat::from_file("fixtures/image/sample.ff").unwrap();
    assert_eq!(fmt, FileFormat::Farbfeld);
}

#[test]
fn test_free_lossless_image_format() {
    let fmt = FileFormat::from_file("fixtures/image/sample.flif").unwrap();
    assert_eq!(fmt, FileFormat::FreeLosslessImageFormat);
}

#[test]
fn test_fujifilm_raw() {
    let fmt = FileFormat::from_file("fixtures/image/sample.raf").unwrap();
    assert_eq!(fmt, FileFormat::FujifilmRaw);
}

#[test]
fn test_graphics_interchange_format() {
    let fmt = FileFormat::from_file("fixtures/image/sample.gif").unwrap();
    assert_eq!(fmt, FileFormat::GraphicsInterchangeFormat);
}

#[test]
fn test_high_efficiency_image_coding() {
    let fmt = FileFormat::from_file("fixtures/image/sample.heic").unwrap();
    assert_eq!(fmt, FileFormat::HighEfficiencyImageCoding);
}

#[test]
fn test_high_efficiency_image_coding_sequence() {
    let fmt = FileFormat::from_file("fixtures/image/sample.heics").unwrap();
    assert_eq!(fmt, FileFormat::HighEfficiencyImageCodingSequence);
}
#[test]
fn test_high_efficiency_image_file_format() {
    let fmt = FileFormat::from_file("fixtures/image/sample.heif").unwrap();
    assert_eq!(fmt, FileFormat::HighEfficiencyImageFileFormat);
}

#[test]
fn test_high_efficiency_image_file_format_sequence() {
    let fmt = FileFormat::from_file("fixtures/image/sample.heifs").unwrap();
    assert_eq!(fmt, FileFormat::HighEfficiencyImageFileFormatSequence);
}

#[test]
fn test_joint_photographic_experts_group() {
    let fmt = FileFormat::from_file("fixtures/image/sample.jpg").unwrap();
    assert_eq!(fmt, FileFormat::JointPhotographicExpertsGroup);
}

#[test]
fn test_jpeg2000_codestream() {
    let fmt = FileFormat::from_file("fixtures/image/sample.j2c").unwrap();
    assert_eq!(fmt, FileFormat::Jpeg2000Codestream);
}

#[test]
fn test_jpeg2000_part1() {
    let fmt = FileFormat::from_file("fixtures/image/sample.jp2").unwrap();
    assert_eq!(fmt, FileFormat::Jpeg2000Part1);
}

#[test]
fn test_jpeg2000_part2() {
    let fmt = FileFormat::from_file("fixtures/image/sample.jpx").unwrap();
    assert_eq!(fmt, FileFormat::Jpeg2000Part2);
}

#[test]
fn test_jpeg2000_part6() {
    let fmt = FileFormat::from_file("fixtures/image/sample.jpm").unwrap();
    assert_eq!(fmt, FileFormat::Jpeg2000Part6);
}

#[test]
fn test_jpeg_extended_range() {
    let fmt = FileFormat::from_file("fixtures/image/sample.jxr").unwrap();
    assert_eq!(fmt, FileFormat::JpegExtendedRange);
}

#[test]
fn test_jpeg_ls() {
    let fmt = FileFormat::from_file("fixtures/image/sample.jls").unwrap();
    assert_eq!(fmt, FileFormat::JpegLs);
}

#[test]
fn test_jpeg_network_graphics() {
    let fmt = FileFormat::from_file("fixtures/image/sample.jng").unwrap();
    assert_eq!(fmt, FileFormat::JpegNetworkGraphics);
}

#[test]
fn test_jpeg_xl() {
    let fmt = FileFormat::from_file("fixtures/image/sample.jxl").unwrap();
    assert_eq!(fmt, FileFormat::JpegXl);
}

#[test]
fn test_khronos_texture() {
    let fmt = FileFormat::from_file("fixtures/image/sample.ktx").unwrap();
    assert_eq!(fmt, FileFormat::KhronosTexture);
}

#[test]
fn test_khronos_texture2() {
    let fmt = FileFormat::from_file("fixtures/image/sample.ktx2").unwrap();
    assert_eq!(fmt, FileFormat::KhronosTexture2);
}

#[test]
fn test_magick_image_file_format() {
    let fmt = FileFormat::from_file("fixtures/image/sample.miff").unwrap();
    assert_eq!(fmt, FileFormat::MagickImageFileFormat);
}

#[test]
fn test_microsoft_directdraw_surface() {
    let fmt = FileFormat::from_file("fixtures/image/sample.dds").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftDirectdrawSurface);
}

#[test]
fn test_multiple_image_network_graphics() {
    let fmt = FileFormat::from_file("fixtures/image/sample.mng").unwrap();
    assert_eq!(fmt, FileFormat::MultipleImageNetworkGraphics);
}

#[test]
fn test_nikon_electronic_file() {
    let fmt = FileFormat::from_file("fixtures/image/sample.nef").unwrap();
    assert_eq!(fmt, FileFormat::NikonElectronicFile);
}

#[test]
fn test_olympus_raw_format() {
    let fmt = FileFormat::from_file("fixtures/image/sample.orf").unwrap();
    assert_eq!(fmt, FileFormat::OlympusRawFormat);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_graphics() {
    let fmt = FileFormat::from_file("fixtures/image/sample.odg").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentGraphics);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_graphics_template() {
    let fmt = FileFormat::from_file("fixtures/image/sample.otg").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentGraphicsTemplate);
}

#[test]
fn test_openexr() {
    let fmt = FileFormat::from_file("fixtures/image/sample.exr").unwrap();
    assert_eq!(fmt, FileFormat::Openexr);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_openraster() {
    let fmt = FileFormat::from_file("fixtures/image/sample.ora").unwrap();
    assert_eq!(fmt, FileFormat::Openraster);
}

#[test]
fn test_panasonic_raw() {
    let fmt = FileFormat::from_file("fixtures/image/sample.rw2").unwrap();
    assert_eq!(fmt, FileFormat::PanasonicRaw);
}

#[test]
fn test_picture_exchange() {
    let fmt = FileFormat::from_file("fixtures/image/sample.pcx").unwrap();
    assert_eq!(fmt, FileFormat::PictureExchange);
}

#[test]
fn test_portable_arbitrary_map() {
    let fmt = FileFormat::from_file("fixtures/image/sample.pam").unwrap();
    assert_eq!(fmt, FileFormat::PortableArbitraryMap);
}

#[test]
fn test_portable_bitmap() {
    let fmt = FileFormat::from_file("fixtures/image/sample.pbm").unwrap();
    assert_eq!(fmt, FileFormat::PortableBitmap);
}

#[test]
fn test_portable_floatmap() {
    let fmt = FileFormat::from_file("fixtures/image/sample.pfm").unwrap();
    assert_eq!(fmt, FileFormat::PortableFloatmap);
}

#[test]
fn test_portable_graymap() {
    let fmt = FileFormat::from_file("fixtures/image/sample.pgm").unwrap();
    assert_eq!(fmt, FileFormat::PortableGraymap);
}

#[test]
fn test_portable_network_graphics() {
    let fmt = FileFormat::from_file("fixtures/image/sample.png").unwrap();
    assert_eq!(fmt, FileFormat::PortableNetworkGraphics);
}

#[test]
fn test_portable_pixmap() {
    let fmt = FileFormat::from_file("fixtures/image/sample.ppm").unwrap();
    assert_eq!(fmt, FileFormat::PortablePixmap);
}

#[test]
fn test_quite_ok_image() {
    let fmt = FileFormat::from_file("fixtures/image/sample.qoi").unwrap();
    assert_eq!(fmt, FileFormat::QuiteOkImage);
}

#[test]
fn test_radiance_hdr() {
    let fmt = FileFormat::from_file("fixtures/image/sample.hdr").unwrap();
    assert_eq!(fmt, FileFormat::RadianceHdr);
}

#[test]
fn test_scalable_vector_graphics_1() {
    let fmt = FileFormat::from_file("fixtures/image/sample1.svg").unwrap();
    assert_eq!(fmt, FileFormat::ScalableVectorGraphics);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_scalable_vector_graphics_2() {
    let fmt = FileFormat::from_file("fixtures/image/sample2.svg").unwrap();
    assert_eq!(fmt, FileFormat::ScalableVectorGraphics);
}

#[test]
fn test_silicon_graphics_image() {
    let fmt = FileFormat::from_file("fixtures/image/sample.sgi").unwrap();
    assert_eq!(fmt, FileFormat::SiliconGraphicsImage);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_stardraw() {
    let fmt = FileFormat::from_file("fixtures/image/sample.sda").unwrap();
    assert_eq!(fmt, FileFormat::Stardraw);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_sun_xml_draw() {
    let fmt = FileFormat::from_file("fixtures/image/sample.sxd").unwrap();
    assert_eq!(fmt, FileFormat::SunXmlDraw);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_sun_xml_draw_template() {
    let fmt = FileFormat::from_file("fixtures/image/sample.std").unwrap();
    assert_eq!(fmt, FileFormat::SunXmlDrawTemplate);
}

#[test]
fn test_tag_image_file_format() {
    let fmt = FileFormat::from_file("fixtures/image/sample.tiff").unwrap();
    assert_eq!(fmt, FileFormat::TagImageFileFormat);
}

#[test]
fn test_webp() {
    let fmt = FileFormat::from_file("fixtures/image/sample.webp").unwrap();
    assert_eq!(fmt, FileFormat::Webp);
}

#[test]
fn test_windows_animated_cursor() {
    let fmt = FileFormat::from_file("fixtures/image/sample.ani").unwrap();
    assert_eq!(fmt, FileFormat::WindowsAnimatedCursor);
}

#[test]
fn test_windows_bitmap() {
    let fmt = FileFormat::from_file("fixtures/image/sample.bmp").unwrap();
    assert_eq!(fmt, FileFormat::WindowsBitmap);
}

#[test]
fn test_windows_cursor() {
    let fmt = FileFormat::from_file("fixtures/image/sample.cur").unwrap();
    assert_eq!(fmt, FileFormat::WindowsCursor);
}

#[test]
fn test_windows_icon() {
    let fmt = FileFormat::from_file("fixtures/image/sample.ico").unwrap();
    assert_eq!(fmt, FileFormat::WindowsIcon);
}

#[test]
fn test_windows_metafile() {
    let fmt = FileFormat::from_file("fixtures/image/sample.wmf").unwrap();
    assert_eq!(fmt, FileFormat::WindowsMetafile);
}

#[test]
fn test_wordperfect_graphics_1() {
    let fmt = FileFormat::from_file("fixtures/image/sample1.wpg").unwrap();
    assert_eq!(fmt, FileFormat::WordperfectGraphics);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_wordperfect_graphics_2() {
    let fmt = FileFormat::from_file("fixtures/image/sample2.wpg").unwrap();
    assert_eq!(fmt, FileFormat::WordperfectGraphics);
}

#[test]
fn test_x_pixmap() {
    let fmt = FileFormat::from_file("fixtures/image/sample.xpm").unwrap();
    assert_eq!(fmt, FileFormat::XPixmap);
}
