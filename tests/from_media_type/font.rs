use file_format::FileFormat;

#[test]
#[cfg(feature = "from-media-type")]
fn test_bmfont_ascii() {
    let fmt = FileFormat::from_media_type("application/x-angelcode-bmfont");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::BmfontAscii)), "{:?} does not contain {}", fmt, FileFormat::BmfontAscii);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_bmfont_binary() {
    let fmt = FileFormat::from_media_type("application/x-angelcode-bmfont");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::BmfontBinary)), "{:?} does not contain {}", fmt, FileFormat::BmfontBinary);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_embedded_opentype() {
    let fmt = FileFormat::from_media_type("application/vnd.ms-fontobject");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::EmbeddedOpentype)), "{:?} does not contain {}", fmt, FileFormat::EmbeddedOpentype);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_glyphs(){
    let fmt = FileFormat::from_media_type("font/x-glyphs");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Glyphs)), "{:?} does not contain {}", fmt, FileFormat::Glyphs);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_opentype(){
    let fmt = FileFormat::from_media_type("font/otf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Opentype)), "{:?} does not contain {}", fmt, FileFormat::Opentype);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_truetype(){
    let fmt = FileFormat::from_media_type("font/ttf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Truetype)), "{:?} does not contain {}", fmt, FileFormat::Truetype);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_web_open_font_format() {
    let fmt = FileFormat::from_media_type("font/woff");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WebOpenFontFormat)), "{:?} does not contain {}", fmt, FileFormat::WebOpenFontFormat);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_web_open_font_format2() {
    let fmt = FileFormat::from_media_type("font/woff2");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WebOpenFontFormat2)), "{:?} does not contain {}", fmt, FileFormat::WebOpenFontFormat2);
}

