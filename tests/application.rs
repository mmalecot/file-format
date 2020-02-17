use file_format::{FileFormat, Kind};

#[test]
fn test_bin() {
    let format = FileFormat::from_file("fixtures/application/sample.bin").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/octet-stream");
    assert_eq!(format.preferred_extension(), Some("bin"));
}

#[test]
fn test_bz2() {
    let format = FileFormat::from_file("fixtures/application/sample.bz2").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-bzip2");
    assert_eq!(format.preferred_extension(), Some("bz2"));
}

#[test]
fn test_gz() {
    let format = FileFormat::from_file("fixtures/application/sample.gz").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/gzip");
    assert_eq!(format.preferred_extension(), Some("gz"));
}

#[test]
fn test_lrz() {
    let format = FileFormat::from_file("fixtures/application/sample.lrz").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-lrzip");
    assert_eq!(format.preferred_extension(), Some("lrz"));
}

#[test]
fn test_lz() {
    let format = FileFormat::from_file("fixtures/application/sample.lz").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-lzip");
    assert_eq!(format.preferred_extension(), Some("lz"));
}

#[test]
fn test_lz4() {
    let format = FileFormat::from_file("fixtures/application/sample.lz4").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-lz4");
    assert_eq!(format.preferred_extension(), Some("lz4"));
}

#[test]
fn test_lzo() {
    let format = FileFormat::from_file("fixtures/application/sample.lzo").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-lzop");
    assert_eq!(format.preferred_extension(), Some("lzo"));
}

#[test]
fn test_xz() {
    let format = FileFormat::from_file("fixtures/application/sample.xz").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-xz");
    assert_eq!(format.preferred_extension(), Some("xz"));
}

#[test]
fn test_z() {
    let format = FileFormat::from_file("fixtures/application/sample.Z").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-compress");
    assert_eq!(format.preferred_extension(), Some("Z"));
}

#[test]
fn test_zst() {
    let format = FileFormat::from_file("fixtures/application/sample.zst").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/zstd");
    assert_eq!(format.preferred_extension(), Some("zst"));
}
