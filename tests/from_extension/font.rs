use file_format::FileFormat;

#[test]
#[cfg(feature = "from-extension")]
fn test_bmfont_ascii() {
    let fmt = FileFormat::from_extension("fnt");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::BmfontAscii)), "{:?} does not contain {}", fmt, FileFormat::BmfontAscii);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_bmfont_binary() {
    let fmt = FileFormat::from_extension("fnt");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::BmfontBinary)), "{:?} does not contain {}", fmt, FileFormat::BmfontBinary);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_embedded_opentype() {
    let fmt = FileFormat::from_extension("eot");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::EmbeddedOpentype)), "{:?} does not contain {}", fmt, FileFormat::EmbeddedOpentype);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_glyphs(){
    let fmt = FileFormat::from_extension("glyphs");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Glyphs)), "{:?} does not contain {}", fmt, FileFormat::Glyphs);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_opentype(){
    let fmt = FileFormat::from_extension("otf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Opentype)), "{:?} does not contain {}", fmt, FileFormat::Opentype);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_truetype(){
    let fmt = FileFormat::from_extension("ttf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Truetype)), "{:?} does not contain {}", fmt, FileFormat::Truetype);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_web_open_font_format() {
    let fmt = FileFormat::from_extension("woff");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WebOpenFontFormat)), "{:?} does not contain {}", fmt, FileFormat::WebOpenFontFormat);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_web_open_font_format2() {
    let fmt = FileFormat::from_extension("woff2");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WebOpenFontFormat2)), "{:?} does not contain {}", fmt, FileFormat::WebOpenFontFormat2);
}

