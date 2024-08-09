use file_format::FileFormat;

#[test]
#[cfg(feature = "from-media-type")]
fn test_broad_band_ebook() {
    let fmt = FileFormat::from_media_type("application/x-lrf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::BroadBandEbook)), "{:?} does not contain {}", fmt, FileFormat::BroadBandEbook);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_electronic_publication() {
    let fmt = FileFormat::from_media_type("application/epub+zip");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ElectronicPublication)), "{:?} does not contain {}", fmt, FileFormat::ElectronicPublication);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_fiction_book(){
    let fmt = FileFormat::from_media_type("application/x-fb2+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Fictionbook)), "{:?} does not contain {}", fmt, FileFormat::Fictionbook);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_fictionbook_zip() {
    let fmt = FileFormat::from_media_type("application/x-fbz");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::FictionbookZip)), "{:?} does not contain {}", fmt, FileFormat::FictionbookZip);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_microsoft_reader() {
    let fmt = FileFormat::from_media_type("application/x-ms-reader");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftReader)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftReader);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_mobipocket(){
    let fmt = FileFormat::from_media_type("application/x-mobipocket-ebook");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Mobipocket)), "{:?} does not contain {}", fmt, FileFormat::Mobipocket);
}

