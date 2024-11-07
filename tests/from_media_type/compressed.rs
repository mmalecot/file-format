use file_format::FileFormat;

#[test]
#[cfg(feature = "from-media-type")]
fn test_bzip(){
    let fmt = FileFormat::from_media_type("application/x-bzip");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Bzip)), "{:?} does not contain {}", fmt, FileFormat::Bzip);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_bzip2(){
    let fmt = FileFormat::from_media_type("application/x-bzip2");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Bzip2)), "{:?} does not contain {}", fmt, FileFormat::Bzip2);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_bzip3(){
    let fmt = FileFormat::from_media_type("application/x-bzip3");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Bzip3)), "{:?} does not contain {}", fmt, FileFormat::Bzip3);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_gzip(){
    let fmt = FileFormat::from_media_type("application/gzip");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Gzip)), "{:?} does not contain {}", fmt, FileFormat::Gzip);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_lempel_ziv_finite_state_entropy() {
    let fmt = FileFormat::from_media_type("application/x-lzfse");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::LempelZivFiniteStateEntropy)), "{:?} does not contain {}", fmt, FileFormat::LempelZivFiniteStateEntropy);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_lempel_ziv_markov_chain_algorithm() {
    let fmt = FileFormat::from_media_type("application/x-lzma");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::LempelZivMarkovChainAlgorithm)), "{:?} does not contain {}", fmt, FileFormat::LempelZivMarkovChainAlgorithm);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_long_range_zip() {
    let fmt = FileFormat::from_media_type("application/x-lrzip");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::LongRangeZip)), "{:?} does not contain {}", fmt, FileFormat::LongRangeZip);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_lz4(){
    let fmt = FileFormat::from_media_type("application/x-lz4");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Lz4)), "{:?} does not contain {}", fmt, FileFormat::Lz4);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_lzip(){
    let fmt = FileFormat::from_media_type("application/x-lzip");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Lzip)), "{:?} does not contain {}", fmt, FileFormat::Lzip);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_lzop(){
    let fmt = FileFormat::from_media_type("application/x-lzop");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Lzop)), "{:?} does not contain {}", fmt, FileFormat::Lzop);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_rzip(){
    let fmt = FileFormat::from_media_type("application/x-rzip");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Rzip)), "{:?} does not contain {}", fmt, FileFormat::Rzip);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_snappy(){
    let fmt = FileFormat::from_media_type("application/x-snappy-framed");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Snappy)), "{:?} does not contain {}", fmt, FileFormat::Snappy);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_unix_compress() {
    let fmt = FileFormat::from_media_type("application/x-compress");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::UnixCompress)), "{:?} does not contain {}", fmt, FileFormat::UnixCompress);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_xz(){
    let fmt = FileFormat::from_media_type("application/x-xz");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Xz)), "{:?} does not contain {}", fmt, FileFormat::Xz);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_zstandard(){
    let fmt = FileFormat::from_media_type("application/zstd");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Zstandard)), "{:?} does not contain {}", fmt, FileFormat::Zstandard);
}

