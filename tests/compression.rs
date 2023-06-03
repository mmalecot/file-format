use file_format::FileFormat;

#[test]
fn test_bzip() {
    let fmt = FileFormat::from_file("fixtures/compression/sample.bz").unwrap();
    assert_eq!(fmt, FileFormat::Bzip);
}

#[test]
fn test_bzip2() {
    let fmt = FileFormat::from_file("fixtures/compression/sample.bz2").unwrap();
    assert_eq!(fmt, FileFormat::Bzip2);
}

#[test]
fn test_bzip3() {
    let fmt = FileFormat::from_file("fixtures/compression/sample.bz3").unwrap();
    assert_eq!(fmt, FileFormat::Bzip3);
}

#[test]
fn test_gzip() {
    let fmt = FileFormat::from_file("fixtures/compression/sample.gz").unwrap();
    assert_eq!(fmt, FileFormat::Gzip);
}

#[test]
fn test_lempel_ziv_finite_state_entropy() {
    let fmt = FileFormat::from_file("fixtures/compression/sample.lzfse").unwrap();
    assert_eq!(fmt, FileFormat::LempelZivFiniteStateEntropy);
}

#[test]
fn test_long_range_zip() {
    let fmt = FileFormat::from_file("fixtures/compression/sample.lrz").unwrap();
    assert_eq!(fmt, FileFormat::LongRangeZip);
}

#[test]
fn test_lz4() {
    let fmt = FileFormat::from_file("fixtures/compression/sample.lz4").unwrap();
    assert_eq!(fmt, FileFormat::Lz4);
}

#[test]
fn test_lzip() {
    let fmt = FileFormat::from_file("fixtures/compression/sample.lz").unwrap();
    assert_eq!(fmt, FileFormat::Lzip);
}

#[test]
fn test_lzop() {
    let fmt = FileFormat::from_file("fixtures/compression/sample.lzo").unwrap();
    assert_eq!(fmt, FileFormat::Lzop);
}

#[test]
fn test_snappy() {
    let fmt = FileFormat::from_file("fixtures/compression/sample.sz").unwrap();
    assert_eq!(fmt, FileFormat::Snappy);
}

#[test]
fn test_unix_compress() {
    let fmt = FileFormat::from_file("fixtures/compression/sample.Z").unwrap();
    assert_eq!(fmt, FileFormat::UnixCompress);
}

#[test]
fn test_xz() {
    let fmt = FileFormat::from_file("fixtures/compression/sample.xz").unwrap();
    assert_eq!(fmt, FileFormat::Xz);
}

#[test]
fn test_zstandard() {
    let fmt = FileFormat::from_file("fixtures/compression/sample.zst").unwrap();
    assert_eq!(fmt, FileFormat::Zstandard);
}
