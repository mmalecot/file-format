use file_format::FileFormat;

#[test]
fn test_android_binary_xml() {
    let fmt = FileFormat::from_file("fixtures/metadata/sample.xml").unwrap();
    assert_eq!(fmt, FileFormat::AndroidBinaryXml);
}

#[test]
fn test_bittorrent_file() {
    let fmt = FileFormat::from_file("fixtures/metadata/sample.torrent").unwrap();
    assert_eq!(fmt, FileFormat::BittorrentFile);
}

#[test]
fn test_cd_audio() {
    let fmt = FileFormat::from_file("fixtures/metadata/sample.cda").unwrap();
    assert_eq!(fmt, FileFormat::CdAudio);
}

#[test]
fn test_macos_alias() {
    let fmt = FileFormat::from_file("fixtures/metadata/sample.alias").unwrap();
    assert_eq!(fmt, FileFormat::MacosAlias);
}

#[test]
fn test_meta_information_encapsulation() {
    let fmt = FileFormat::from_file("fixtures/metadata/sample.mie").unwrap();
    assert_eq!(fmt, FileFormat::MetaInformationEncapsulation);
}

#[test]
fn test_tasty() {
    let fmt = FileFormat::from_file("fixtures/metadata/sample.tasty").unwrap();
    assert_eq!(fmt, FileFormat::Tasty);
}

#[test]
fn test_windows_shortcut() {
    let fmt = FileFormat::from_file("fixtures/metadata/sample.lnk").unwrap();
    assert_eq!(fmt, FileFormat::WindowsShortcut);
}
