use file_format::FileFormat;

#[test]
fn test_bzip() {
    let fmt = FileFormat::from_file("fixtures/compressed/sample.bz").unwrap();
    assert_eq!(fmt, FileFormat::Bzip);
}

#[test]
fn test_bzip2() {
    let fmt = FileFormat::from_file("fixtures/compressed/sample.bz2").unwrap();
    assert_eq!(fmt, FileFormat::Bzip2);
}

#[test]
fn test_bzip3() {
    let fmt = FileFormat::from_file("fixtures/compressed/sample.bz3").unwrap();
    assert_eq!(fmt, FileFormat::Bzip3);
}

#[test]
fn test_gzip() {
    let fmt = FileFormat::from_file("fixtures/compressed/sample.gz").unwrap();
    assert_eq!(fmt, FileFormat::Gzip);
}

#[test]
fn test_lempel_ziv_finite_state_entropy() {
    let fmt = FileFormat::from_file("fixtures/compressed/sample.lzfse").unwrap();
    assert_eq!(fmt, FileFormat::LempelZivFiniteStateEntropy);
}

#[test]
fn test_lempel_ziv_markov_chain_algorithm() {
    let fmt = FileFormat::from_file("fixtures/compressed/sample.lzma").unwrap();
    assert_eq!(fmt, FileFormat::LempelZivMarkovChainAlgorithm);
}

#[test]
fn test_long_range_zip() {
    let fmt = FileFormat::from_file("fixtures/compressed/sample.lrz").unwrap();
    assert_eq!(fmt, FileFormat::LongRangeZip);
}

#[test]
fn test_lz4() {
    let fmt = FileFormat::from_file("fixtures/compressed/sample.lz4").unwrap();
    assert_eq!(fmt, FileFormat::Lz4);
}

#[test]
fn test_lzip() {
    let fmt = FileFormat::from_file("fixtures/compressed/sample.lz").unwrap();
    assert_eq!(fmt, FileFormat::Lzip);
}

#[test]
fn test_lzop() {
    let fmt = FileFormat::from_file("fixtures/compressed/sample.lzo").unwrap();
    assert_eq!(fmt, FileFormat::Lzop);
}

#[test]
fn test_rzip() {
    let fmt = FileFormat::from_file("fixtures/compressed/sample.rz").unwrap();
    assert_eq!(fmt, FileFormat::Rzip);
}

#[test]
fn test_snappy() {
    let fmt = FileFormat::from_file("fixtures/compressed/sample.sz").unwrap();
    assert_eq!(fmt, FileFormat::Snappy);
}

#[test]
fn test_unix_compress() {
    let fmt = FileFormat::from_file("fixtures/compressed/sample.Z").unwrap();
    assert_eq!(fmt, FileFormat::UnixCompress);
}

#[test]
fn test_xz() {
    let fmt = FileFormat::from_file("fixtures/compressed/sample.xz").unwrap();
    assert_eq!(fmt, FileFormat::Xz);
}

#[test]
fn test_zstandard() {
    let fmt = FileFormat::from_file("fixtures/compressed/sample.zst").unwrap();
    assert_eq!(fmt, FileFormat::Zstandard);
}
