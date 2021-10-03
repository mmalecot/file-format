use file_format::FileFormat;

#[test]
fn test_7z() {
    let format = FileFormat::from_file("fixtures/application/sample.7z").unwrap();
    assert_eq!(format, FileFormat::SevenZip);
    assert_eq!(format.media_type(), "application/x-7z-compressed");
    assert_eq!(format.preferred_extension(), "7z");
}

#[test]
fn test_bin() {
    let format = FileFormat::from_file("fixtures/application/sample.bin").unwrap();
    assert_eq!(format, FileFormat::Binary);
    assert_eq!(format.media_type(), "application/octet-stream");
    assert_eq!(format.preferred_extension(), "bin");
}

#[test]
fn test_blend() {
    let format = FileFormat::from_file("fixtures/application/sample.blend").unwrap();
    assert_eq!(format, FileFormat::Blender);
    assert_eq!(format.media_type(), "application/x-blender");
    assert_eq!(format.preferred_extension(), "blend");
}

#[test]
fn test_bz2() {
    let format = FileFormat::from_file("fixtures/application/sample.bz2").unwrap();
    assert_eq!(format, FileFormat::Bzip2);
    assert_eq!(format.media_type(), "application/x-bzip2");
    assert_eq!(format.preferred_extension(), "bz2");
}

#[test]
fn test_cab() {
    let format = FileFormat::from_file("fixtures/application/sample.cab").unwrap();
    assert_eq!(format, FileFormat::Cab);
    assert_eq!(format.media_type(), "application/vnd.ms-cab-compressed");
    assert_eq!(format.preferred_extension(), "cab");
}

#[test]
fn test_class() {
    let format = FileFormat::from_file("fixtures/application/sample.class").unwrap();
    assert_eq!(format, FileFormat::JavaClass);
    assert_eq!(format.media_type(), "application/java-vm");
    assert_eq!(format.preferred_extension(), "class");
}

#[test]
fn test_crx() {
    let format = FileFormat::from_file("fixtures/application/sample.crx").unwrap();
    assert_eq!(format, FileFormat::GoogleChromeExtension);
    assert_eq!(format.media_type(), "application/x-google-chrome-extension");
    assert_eq!(format.preferred_extension(), "crx");
}

#[test]
fn test_dcm() {
    let format = FileFormat::from_file("fixtures/application/sample.dcm").unwrap();
    assert_eq!(format, FileFormat::Dicom);
    assert_eq!(format.media_type(), "application/dicom");
    assert_eq!(format.preferred_extension(), "dcm");
}

#[test]
fn test_deb() {
    let format = FileFormat::from_file("fixtures/application/sample.deb").unwrap();
    assert_eq!(format, FileFormat::Deb);
    assert_eq!(format.media_type(), "application/x-deb");
    assert_eq!(format.preferred_extension(), "deb");
}

#[test]
fn test_dex() {
    let format = FileFormat::from_file("fixtures/application/sample.dex").unwrap();
    assert_eq!(format, FileFormat::Dex);
    assert_eq!(format.media_type(), "application/vnd.android.dex");
    assert_eq!(format.preferred_extension(), "dex");
}

#[test]
fn test_dmg() {
    let format = FileFormat::from_file("fixtures/application/sample.dmg").unwrap();
    assert_eq!(format, FileFormat::AppleDiskImage);
    assert_eq!(format.media_type(), "application/x-apple-diskimage");
    assert_eq!(format.preferred_extension(), "dmg");
}

#[test]
fn test_elf() {
    let format = FileFormat::from_file("fixtures/application/sample.elf").unwrap();
    assert_eq!(format, FileFormat::Elf);
    assert_eq!(format.media_type(), "application/x-executable");
    assert_eq!(format.preferred_extension(), "elf");
}

#[test]
fn test_eot() {
    let format = FileFormat::from_file("fixtures/application/sample.eot").unwrap();
    assert_eq!(format, FileFormat::Eot);
    assert_eq!(format.media_type(), "application/vnd.ms-fontobject");
    assert_eq!(format.preferred_extension(), "eot");
}

#[test]
fn test_exe() {
    let format = FileFormat::from_file("fixtures/application/sample.exe").unwrap();
    assert_eq!(format, FileFormat::Exe);
    assert_eq!(format.media_type(), "application/x-msdownload");
    assert_eq!(format.preferred_extension(), "exe");
}

#[test]
fn test_gb() {
    let format = FileFormat::from_file("fixtures/application/sample.gb").unwrap();
    assert_eq!(format, FileFormat::GbRom);
    assert_eq!(format.media_type(), "application/x-gameboy-rom");
    assert_eq!(format.preferred_extension(), "gb");
}

#[test]
fn test_gba() {
    let format = FileFormat::from_file("fixtures/application/sample.gba").unwrap();
    assert_eq!(format, FileFormat::GbaRom);
    assert_eq!(format.media_type(), "application/x-gba-rom");
    assert_eq!(format.preferred_extension(), "gba");
}

#[test]
fn test_gz() {
    let format = FileFormat::from_file("fixtures/application/sample.gz").unwrap();
    assert_eq!(format, FileFormat::Gzip);
    assert_eq!(format.media_type(), "application/gzip");
    assert_eq!(format.preferred_extension(), "gz");
}

#[test]
fn test_indd() {
    let format = FileFormat::from_file("fixtures/application/sample.indd").unwrap();
    assert_eq!(format, FileFormat::InDesignDocument);
    assert_eq!(format.media_type(), "application/x-indesign");
    assert_eq!(format.preferred_extension(), "indd");
}

#[test]
fn test_iso() {
    let format = FileFormat::from_file("fixtures/application/sample.iso").unwrap();
    assert_eq!(format, FileFormat::Iso);
    assert_eq!(format.media_type(), "application/x-iso9660-image");
    assert_eq!(format.preferred_extension(), "iso");
}

#[test]
fn test_lrz() {
    let format = FileFormat::from_file("fixtures/application/sample.lrz").unwrap();
    assert_eq!(format, FileFormat::Lrzip);
    assert_eq!(format.media_type(), "application/x-lrzip");
    assert_eq!(format.preferred_extension(), "lrz");
}

#[test]
fn test_lz() {
    let format = FileFormat::from_file("fixtures/application/sample.lz").unwrap();
    assert_eq!(format, FileFormat::Lzip);
    assert_eq!(format.media_type(), "application/x-lzip");
    assert_eq!(format.preferred_extension(), "lz");
}

#[test]
fn test_lz4() {
    let format = FileFormat::from_file("fixtures/application/sample.lz4").unwrap();
    assert_eq!(format, FileFormat::Lz4);
    assert_eq!(format.media_type(), "application/x-lz4");
    assert_eq!(format.preferred_extension(), "lz4");
}

#[test]
fn test_lzo() {
    let format = FileFormat::from_file("fixtures/application/sample.lzo").unwrap();
    assert_eq!(format, FileFormat::Lzop);
    assert_eq!(format.media_type(), "application/x-lzop");
    assert_eq!(format.preferred_extension(), "lzo");
}

#[test]
fn test_msi() {
    let format = FileFormat::from_file("fixtures/application/sample.msi").unwrap();
    assert_eq!(format, FileFormat::WindowsInstaller);
    assert_eq!(format.media_type(), "application/x-ole-storage");
    assert_eq!(format.preferred_extension(), "msi");
}

#[test]
fn test_nds() {
    let format = FileFormat::from_file("fixtures/application/sample.nds").unwrap();
    assert_eq!(format, FileFormat::NdsRom);
    assert_eq!(format.media_type(), "application/x-nintendo-ds-rom");
    assert_eq!(format.preferred_extension(), "nds");
}

#[test]
fn test_nes() {
    let format = FileFormat::from_file("fixtures/application/sample.nes").unwrap();
    assert_eq!(format, FileFormat::NesRom);
    assert_eq!(format.media_type(), "application/x-nintendo-nes-rom");
    assert_eq!(format.preferred_extension(), "nes");
}

#[test]
fn test_pcap() {
    let format = FileFormat::from_file("fixtures/application/sample.pcap").unwrap();
    assert_eq!(format, FileFormat::Pcap);
    assert_eq!(format.media_type(), "application/vnd.tcpdump.pcap");
    assert_eq!(format.preferred_extension(), "pcap");
}

#[test]
fn test_pcapng() {
    let format = FileFormat::from_file("fixtures/application/sample.pcapng").unwrap();
    assert_eq!(format, FileFormat::Pcapng);
    assert_eq!(format.media_type(), "application/x-pcapng");
    assert_eq!(format.preferred_extension(), "pcapng");
}

#[test]
fn test_pdf() {
    let format = FileFormat::from_file("fixtures/application/sample.pdf").unwrap();
    assert_eq!(format, FileFormat::Pdf);
    assert_eq!(format.media_type(), "application/pdf");
    assert_eq!(format.preferred_extension(), "pdf");
}

#[test]
fn test_rar() {
    let format = FileFormat::from_file("fixtures/application/sample.rar").unwrap();
    assert_eq!(format, FileFormat::Rar);
    assert_eq!(format.media_type(), "application/vnd.rar");
    assert_eq!(format.preferred_extension(), "rar");
}

#[test]
fn test_rpm() {
    let format = FileFormat::from_file("fixtures/application/sample.rpm").unwrap();
    assert_eq!(format, FileFormat::Rpm);
    assert_eq!(format.media_type(), "application/x-rpm");
    assert_eq!(format.preferred_extension(), "rpm");
}

#[test]
fn test_sqlite() {
    let format = FileFormat::from_file("fixtures/application/sample.sqlite").unwrap();
    assert_eq!(format, FileFormat::Sqlite3);
    assert_eq!(format.media_type(), "application/vnd.sqlite3");
    assert_eq!(format.preferred_extension(), "sqlite");
}

#[test]
fn test_swf() {
    let format = FileFormat::from_file("fixtures/application/sample.swf").unwrap();
    assert_eq!(format, FileFormat::Swf);
    assert_eq!(format.media_type(), "application/x-shockwave-flash");
    assert_eq!(format.preferred_extension(), "swf");
}

#[test]
fn test_tar() {
    let format = FileFormat::from_file("fixtures/application/sample.tar").unwrap();
    assert_eq!(format, FileFormat::Tar);
    assert_eq!(format.media_type(), "application/x-tar");
    assert_eq!(format.preferred_extension(), "tar");
}

#[test]
fn test_vdi() {
    let format = FileFormat::from_file("fixtures/application/sample.vdi").unwrap();
    assert_eq!(format, FileFormat::Vdi);
    assert_eq!(format.media_type(), "application/x-virtualbox-vdi");
    assert_eq!(format.preferred_extension(), "vdi");
}

#[test]
fn test_wasm() {
    let format = FileFormat::from_file("fixtures/application/sample.wasm").unwrap();
    assert_eq!(format, FileFormat::WebAssembly);
    assert_eq!(format.media_type(), "application/wasm");
    assert_eq!(format.preferred_extension(), "wasm");
}

#[test]
fn test_xar() {
    let format = FileFormat::from_file("fixtures/application/sample.xar").unwrap();
    assert_eq!(format, FileFormat::Xar);
    assert_eq!(format.media_type(), "application/x-xar");
    assert_eq!(format.preferred_extension(), "xar");
}

#[test]
fn test_xz() {
    let format = FileFormat::from_file("fixtures/application/sample.xz").unwrap();
    assert_eq!(format, FileFormat::Xz);
    assert_eq!(format.media_type(), "application/x-xz");
    assert_eq!(format.preferred_extension(), "xz");
}

#[test]
fn test_z64() {
    let format = FileFormat::from_file("fixtures/application/sample.z64").unwrap();
    assert_eq!(format, FileFormat::N64Rom);
    assert_eq!(format.media_type(), "application/x-n64-rom");
    assert_eq!(format.preferred_extension(), "z64");
}

#[test]
fn test_z() {
    let format = FileFormat::from_file("fixtures/application/sample.Z").unwrap();
    assert_eq!(format, FileFormat::Compress);
    assert_eq!(format.media_type(), "application/x-compress");
    assert_eq!(format.preferred_extension(), "Z");
}

#[test]
fn test_zip() {
    let format = FileFormat::from_file("fixtures/application/sample.zip").unwrap();
    assert_eq!(format, FileFormat::Zip);
    assert_eq!(format.media_type(), "application/zip");
    assert_eq!(format.preferred_extension(), "zip");
}

#[test]
fn test_zst() {
    let format = FileFormat::from_file("fixtures/application/sample.zst").unwrap();
    assert_eq!(format, FileFormat::Zstandard);
    assert_eq!(format.media_type(), "application/zstd");
    assert_eq!(format.preferred_extension(), "zst");
}
