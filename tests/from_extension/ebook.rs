use file_format::FileFormat;

#[test]
#[cfg(feature = "from-extension")]
fn test_broad_band_ebook() {
    let fmt = FileFormat::from_extension("lrf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::BroadBandEbook)), "{:?} does not contain {}", fmt, FileFormat::BroadBandEbook);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_electronic_publication() {
    let fmt = FileFormat::from_extension("epub");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ElectronicPublication)), "{:?} does not contain {}", fmt, FileFormat::ElectronicPublication);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_fiction_book(){
    let fmt = FileFormat::from_extension("fb2");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Fictionbook)), "{:?} does not contain {}", fmt, FileFormat::Fictionbook);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_fictionbook_zip() {
    let fmt = FileFormat::from_extension("fbz");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::FictionbookZip)), "{:?} does not contain {}", fmt, FileFormat::FictionbookZip);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_microsoft_reader() {
    let fmt = FileFormat::from_extension("lit");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftReader)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftReader);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_mobipocket(){
    let fmt = FileFormat::from_extension("mobi");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Mobipocket)), "{:?} does not contain {}", fmt, FileFormat::Mobipocket);
}

