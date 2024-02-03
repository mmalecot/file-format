use file_format::FileFormat;

#[test]
fn test_ace() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.ace").unwrap();
    assert_eq!(fmt, FileFormat::Ace);
}

#[test]
fn test_alz() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.alz").unwrap();
    assert_eq!(fmt, FileFormat::Alz);
}

#[test]
fn test_archived_by_robert_jung() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.arj").unwrap();
    assert_eq!(fmt, FileFormat::ArchivedByRobertJung);
}

#[test]
fn test_cabinet() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.cab").unwrap();
    assert_eq!(fmt, FileFormat::Cabinet);
}

#[test]
fn test_cpio() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.cpio").unwrap();
    assert_eq!(fmt, FileFormat::Cpio);
}

#[test]
fn test_extensible_archive() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.xar").unwrap();
    assert_eq!(fmt, FileFormat::ExtensibleArchive);
}

#[test]
fn test_larc() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.lzs").unwrap();
    assert_eq!(fmt, FileFormat::Larc);
}

#[test]
fn test_lha() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.lzh").unwrap();
    assert_eq!(fmt, FileFormat::Lha);
}

#[test]
fn test_mozilla_archive() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.mar").unwrap();
    assert_eq!(fmt, FileFormat::MozillaArchive);
}

#[test]
fn test_multi_layer_archive() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.mla").unwrap();
    assert_eq!(fmt, FileFormat::MultiLayerArchive);
}

#[test]
fn test_pmarc() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.pma").unwrap();
    assert_eq!(fmt, FileFormat::Pmarc);
}

#[test]
fn test_roshal_archive() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.rar").unwrap();
    assert_eq!(fmt, FileFormat::RoshalArchive);
}

#[test]
fn test_seqbox() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.sbx").unwrap();
    assert_eq!(fmt, FileFormat::Seqbox);
}

#[test]
fn test_seven_zip() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.7z").unwrap();
    assert_eq!(fmt, FileFormat::SevenZip);
}

#[test]
fn test_squashfs() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.squashfs").unwrap();
    assert_eq!(fmt, FileFormat::Squashfs);
}

#[test]
fn test_stuffit() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.sit").unwrap();
    assert_eq!(fmt, FileFormat::Stuffit);
}

#[test]
fn test_stuffit_x() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.sitx").unwrap();
    assert_eq!(fmt, FileFormat::StuffitX);
}

#[test]
fn test_tape_archive() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.tar").unwrap();
    assert_eq!(fmt, FileFormat::TapeArchive);
}

#[test]
fn test_unix_archiver() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.a").unwrap();
    assert_eq!(fmt, FileFormat::UnixArchiver);
}

#[test]
fn test_windows_imaging_format() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.wim").unwrap();
    assert_eq!(fmt, FileFormat::WindowsImagingFormat);
}

#[test]
fn test_zip() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.zip").unwrap();
    assert_eq!(fmt, FileFormat::Zip);
}

#[test]
fn test_zoo() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.zoo").unwrap();
    assert_eq!(fmt, FileFormat::Zoo);
}

#[test]
fn test_zpaq() {
    let fmt = FileFormat::from_file("fixtures/archive/sample.zpaq").unwrap();
    assert_eq!(fmt, FileFormat::Zpaq);
}
