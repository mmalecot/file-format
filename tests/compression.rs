use file_format::FileFormat;

#[test]
fn test_bzip2() {
    let format = FileFormat::from_file("fixtures/compression/sample.bz2").unwrap();
    assert_eq!(format, FileFormat::Bzip2);
}

#[test]
fn test_gzip() {
    let format = FileFormat::from_file("fixtures/compression/sample.gz").unwrap();
    assert_eq!(format, FileFormat::Gzip);
}

#[test]
fn test_lempel_ziv_finite_state_entropy() {
    let format = FileFormat::from_file("fixtures/compression/sample.lzfse").unwrap();
    assert_eq!(format, FileFormat::LempelZivFiniteStateEntropy);
}

#[test]
fn test_long_range_zip() {
    let format = FileFormat::from_file("fixtures/compression/sample.lrz").unwrap();
    assert_eq!(format, FileFormat::LongRangeZip);
}

#[test]
fn test_lz4() {
    let format = FileFormat::from_file("fixtures/compression/sample.lz4").unwrap();
    assert_eq!(format, FileFormat::Lz4);
}

#[test]
fn test_lzip() {
    let format = FileFormat::from_file("fixtures/compression/sample.lz").unwrap();
    assert_eq!(format, FileFormat::Lzip);
}

#[test]
fn test_lzop() {
    let format = FileFormat::from_file("fixtures/compression/sample.lzo").unwrap();
    assert_eq!(format, FileFormat::Lzop);
}

#[test]
fn test_snappy() {
    let format = FileFormat::from_file("fixtures/compression/sample.sz").unwrap();
    assert_eq!(format, FileFormat::Snappy);
}

#[test]
fn test_unix_compress() {
    let format = FileFormat::from_file("fixtures/compression/sample.Z").unwrap();
    assert_eq!(format, FileFormat::UnixCompress);
}

#[test]
fn test_xz() {
    let format = FileFormat::from_file("fixtures/compression/sample.xz").unwrap();
    assert_eq!(format, FileFormat::Xz);
}

#[test]
fn test_zstandard() {
    let format = FileFormat::from_file("fixtures/compression/sample.zst").unwrap();
    assert_eq!(format, FileFormat::Zstandard);
}
