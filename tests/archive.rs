use file_format::FileFormat;

#[test]
fn test_advanced_compression_engine() {
    let format = FileFormat::from_file("fixtures/archive/sample.ace").unwrap();
    assert_eq!(format, FileFormat::AdvancedCompressionEngine);
}

#[test]
fn test_alz() {
    let format = FileFormat::from_file("fixtures/archive/sample.alz").unwrap();
    assert_eq!(format, FileFormat::Alz);
}

#[test]
fn test_archived_by_robert_jung() {
    let format = FileFormat::from_file("fixtures/archive/sample.arj").unwrap();
    assert_eq!(format, FileFormat::ArchivedByRobertJung);
}

#[test]
fn test_cabinet() {
    let format = FileFormat::from_file("fixtures/archive/sample.cab").unwrap();
    assert_eq!(format, FileFormat::Cabinet);
}

#[test]
fn test_cpio() {
    let format = FileFormat::from_file("fixtures/archive/sample.cpio").unwrap();
    assert_eq!(format, FileFormat::Cpio);
}

#[test]
fn test_extensible_archive() {
    let format = FileFormat::from_file("fixtures/archive/sample.xar").unwrap();
    assert_eq!(format, FileFormat::ExtensibleArchive);
}

#[test]
fn test_larc() {
    let format = FileFormat::from_file("fixtures/archive/sample.lzs").unwrap();
    assert_eq!(format, FileFormat::Larc);
}

#[test]
fn test_lha() {
    let format = FileFormat::from_file("fixtures/archive/sample.lzh").unwrap();
    assert_eq!(format, FileFormat::Lha);
}

#[test]
fn test_pmarc() {
    let format = FileFormat::from_file("fixtures/archive/sample.pma").unwrap();
    assert_eq!(format, FileFormat::Pmarc);
}

#[test]
fn test_roshal_archive() {
    let format = FileFormat::from_file("fixtures/archive/sample.rar").unwrap();
    assert_eq!(format, FileFormat::RoshalArchive);
}

#[test]
fn test_seqbox() {
    let format = FileFormat::from_file("fixtures/archive/sample.sbx").unwrap();
    assert_eq!(format, FileFormat::Seqbox);
}

#[test]
fn test_seven_zip() {
    let format = FileFormat::from_file("fixtures/archive/sample.7z").unwrap();
    assert_eq!(format, FileFormat::SevenZip);
}

#[test]
fn test_tape_archive() {
    let format = FileFormat::from_file("fixtures/archive/sample.tar").unwrap();
    assert_eq!(format, FileFormat::TapeArchive);
}

#[test]
fn test_unix_archiver() {
    let format = FileFormat::from_file("fixtures/archive/sample.a").unwrap();
    assert_eq!(format, FileFormat::UnixArchiver);
}

#[test]
fn test_zip() {
    let format = FileFormat::from_file("fixtures/archive/sample.zip").unwrap();
    assert_eq!(format, FileFormat::Zip);
}

#[test]
fn test_zoo() {
    let format = FileFormat::from_file("fixtures/archive/sample.zoo").unwrap();
    assert_eq!(format, FileFormat::Zoo);
}
