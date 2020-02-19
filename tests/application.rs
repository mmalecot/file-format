use file_format::{FileFormat, Kind};

#[test]
fn test_7z() {
    let format = FileFormat::from_file("fixtures/application/sample.7z").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-7z-compressed");
    assert_eq!(format.preferred_extension(), "7z");
}

#[test]
fn test_bin() {
    let format = FileFormat::from_file("fixtures/application/sample.bin").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/octet-stream");
    assert_eq!(format.preferred_extension(), "bin");
}

#[test]
fn test_bz2() {
    let format = FileFormat::from_file("fixtures/application/sample.bz2").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-bzip2");
    assert_eq!(format.preferred_extension(), "bz2");
}

#[test]
fn test_deb() {
    let format = FileFormat::from_file("fixtures/application/sample.deb").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-deb");
    assert_eq!(format.preferred_extension(), "deb");
}

#[test]
fn test_elf() {
    let format = FileFormat::from_file("fixtures/application/sample.elf").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-executable");
    assert_eq!(format.preferred_extension(), "elf");
}

#[test]
fn test_exe() {
    let format = FileFormat::from_file("fixtures/application/sample.exe").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-msdownload");
    assert_eq!(format.preferred_extension(), "exe");
}

#[test]
fn test_gz() {
    let format = FileFormat::from_file("fixtures/application/sample.gz").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/gzip");
    assert_eq!(format.preferred_extension(), "gz");
}

#[test]
fn test_lrz() {
    let format = FileFormat::from_file("fixtures/application/sample.lrz").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-lrzip");
    assert_eq!(format.preferred_extension(), "lrz");
}

#[test]
fn test_lz() {
    let format = FileFormat::from_file("fixtures/application/sample.lz").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-lzip");
    assert_eq!(format.preferred_extension(), "lz");
}

#[test]
fn test_lz4() {
    let format = FileFormat::from_file("fixtures/application/sample.lz4").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-lz4");
    assert_eq!(format.preferred_extension(), "lz4");
}

#[test]
fn test_lzo() {
    let format = FileFormat::from_file("fixtures/application/sample.lzo").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-lzop");
    assert_eq!(format.preferred_extension(), "lzo");
}

#[test]
fn test_pcap() {
    let format = FileFormat::from_file("fixtures/application/sample.pcap").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/vnd.tcpdump.pcap");
    assert_eq!(format.preferred_extension(), "pcap");
}

#[test]
fn test_pcapng() {
    let format = FileFormat::from_file("fixtures/application/sample.pcapng").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-pcapng");
    assert_eq!(format.preferred_extension(), "pcapng");
}

#[test]
fn test_pdf() {
    let format = FileFormat::from_file("fixtures/application/sample.pdf").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/pdf");
    assert_eq!(format.preferred_extension(), "pdf");
}

#[test]
fn test_rar() {
    let format = FileFormat::from_file("fixtures/application/sample.rar").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/vnd.rar");
    assert_eq!(format.preferred_extension(), "rar");
}

#[test]
fn test_rpm() {
    let format = FileFormat::from_file("fixtures/application/sample.rpm").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-rpm");
    assert_eq!(format.preferred_extension(), "rpm");
}

#[test]
fn test_sqlite() {
    let format = FileFormat::from_file("fixtures/application/sample.sqlite").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/vnd.sqlite3");
    assert_eq!(format.preferred_extension(), "sqlite");
}

#[test]
fn test_tar() {
    let format = FileFormat::from_file("fixtures/application/sample.tar").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-tar");
    assert_eq!(format.preferred_extension(), "tar");
}

#[test]
fn test_zip() {
    let format = FileFormat::from_file("fixtures/application/sample.zip").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/zip");
    assert_eq!(format.preferred_extension(), "zip");
}

#[test]
fn test_xz() {
    let format = FileFormat::from_file("fixtures/application/sample.xz").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-xz");
    assert_eq!(format.preferred_extension(), "xz");
}

#[test]
fn test_z() {
    let format = FileFormat::from_file("fixtures/application/sample.Z").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/x-compress");
    assert_eq!(format.preferred_extension(), "Z");
}

#[test]
fn test_zst() {
    let format = FileFormat::from_file("fixtures/application/sample.zst").unwrap();
    assert_eq!(format.kind(), Kind::Application);
    assert_eq!(format.media_type(), "application/zstd");
    assert_eq!(format.preferred_extension(), "zst");
}
