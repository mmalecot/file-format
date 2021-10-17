use file_format::FileFormat;

#[test]
fn test_7z() {
    let format = FileFormat::from_file("fixtures/application/sample.7z").unwrap();
    assert_eq!(format.media_type(), "application/x-7z-compressed");
    assert_eq!(format.extension(), "7z");
}

#[test]
fn test_alias() {
    let format = FileFormat::from_file("fixtures/application/sample.alias").unwrap();
    assert_eq!(format.media_type(), "application/x-apple-alias");
    assert_eq!(format.extension(), "alias");
}

#[test]
fn test_alz() {
    let format = FileFormat::from_file("fixtures/application/sample.alz").unwrap();
    assert_eq!(format.media_type(), "application/x-alz-compressed");
    assert_eq!(format.extension(), "alz");
}

#[test]
fn test_ani() {
    let format = FileFormat::from_file("fixtures/application/sample.ani").unwrap();
    assert_eq!(format.media_type(), "application/x-navi-animation");
    assert_eq!(format.extension(), "ani");
}

#[test]
fn test_ar() {
    let format = FileFormat::from_file("fixtures/application/sample.ar").unwrap();
    assert_eq!(format.media_type(), "application/x-archive");
    assert_eq!(format.extension(), "ar");
}

#[test]
fn test_arj() {
    let format = FileFormat::from_file("fixtures/application/sample.arj").unwrap();
    assert_eq!(format.media_type(), "application/x-arj");
    assert_eq!(format.extension(), "arj");
}

#[test]
fn test_arrow() {
    let format = FileFormat::from_file("fixtures/application/sample.arrow").unwrap();
    assert_eq!(format.media_type(), "application/x-apache-arrow");
    assert_eq!(format.extension(), "arrow");
}

#[test]
fn test_bin() {
    let format = FileFormat::from_file("fixtures/application/sample.bin").unwrap();
    assert_eq!(format.media_type(), "application/octet-stream");
    assert_eq!(format.extension(), "bin");
}

#[test]
fn test_blend() {
    let format = FileFormat::from_file("fixtures/application/sample.blend").unwrap();
    assert_eq!(format.media_type(), "application/x-blender");
    assert_eq!(format.extension(), "blend");
}

#[test]
fn test_bz2() {
    let format = FileFormat::from_file("fixtures/application/sample.bz2").unwrap();
    assert_eq!(format.media_type(), "application/x-bzip2");
    assert_eq!(format.extension(), "bz2");
}

#[test]
fn test_cab() {
    let format = FileFormat::from_file("fixtures/application/sample.cab").unwrap();
    assert_eq!(format.media_type(), "application/vnd.ms-cab-compressed");
    assert_eq!(format.extension(), "cab");
}

#[test]
fn test_cfb() {
    let format = FileFormat::from_file("fixtures/application/sample.cfb").unwrap();
    assert_eq!(format.media_type(), "application/x-cfb");
    assert_eq!(format.extension(), "cfb");
}

#[test]
fn test_chm() {
    let format = FileFormat::from_file("fixtures/application/sample.chm").unwrap();
    assert_eq!(format.media_type(), "application/vnd.ms-htmlhelp");
    assert_eq!(format.extension(), "chm");
}

#[test]
fn test_class() {
    let format = FileFormat::from_file("fixtures/application/sample.class").unwrap();
    assert_eq!(format.media_type(), "application/java-vm");
    assert_eq!(format.extension(), "class");
}

#[test]
fn test_cpio() {
    let format = FileFormat::from_file("fixtures/application/sample.cpio").unwrap();
    assert_eq!(format.media_type(), "application/x-cpio");
    assert_eq!(format.extension(), "cpio");
}

#[test]
fn test_crx() {
    let format = FileFormat::from_file("fixtures/application/sample.crx").unwrap();
    assert_eq!(format.media_type(), "application/x-google-chrome-extension");
    assert_eq!(format.extension(), "crx");
}

#[test]
fn test_dcm() {
    let format = FileFormat::from_file("fixtures/application/sample.dcm").unwrap();
    assert_eq!(format.media_type(), "application/dicom");
    assert_eq!(format.extension(), "dcm");
}

#[test]
fn test_deb() {
    let format = FileFormat::from_file("fixtures/application/sample.deb").unwrap();
    assert_eq!(format.media_type(), "application/vnd.debian.binary-package");
    assert_eq!(format.extension(), "deb");
}

#[test]
fn test_dex() {
    let format = FileFormat::from_file("fixtures/application/sample.dex").unwrap();
    assert_eq!(format.media_type(), "application/vnd.android.dex");
    assert_eq!(format.extension(), "dex");
}

#[test]
fn test_dmg() {
    let format = FileFormat::from_file("fixtures/application/sample.dmg").unwrap();
    assert_eq!(format.media_type(), "application/x-apple-diskimage");
    assert_eq!(format.extension(), "dmg");
}

#[test]
fn test_elf() {
    let format = FileFormat::from_file("fixtures/application/sample.elf").unwrap();
    assert_eq!(format.media_type(), "application/x-executable");
    assert_eq!(format.extension(), "elf");
}

#[test]
fn test_eot() {
    let format = FileFormat::from_file("fixtures/application/sample.eot").unwrap();
    assert_eq!(format.media_type(), "application/vnd.ms-fontobject");
    assert_eq!(format.extension(), "eot");
}

#[test]
fn test_epub() {
    let format = FileFormat::from_file("fixtures/application/sample.epub").unwrap();
    assert_eq!(format.media_type(), "application/epub+zip");
    assert_eq!(format.extension(), "epub");
}

#[test]
fn test_exe() {
    let format = FileFormat::from_file("fixtures/application/sample.exe").unwrap();
    assert_eq!(format.media_type(), "application/x-msdownload");
    assert_eq!(format.extension(), "exe");
}

#[test]
fn test_gb() {
    let format = FileFormat::from_file("fixtures/application/sample.gb").unwrap();
    assert_eq!(format.media_type(), "application/x-gameboy-rom");
    assert_eq!(format.extension(), "gb");
}

#[test]
fn test_gba() {
    let format = FileFormat::from_file("fixtures/application/sample.gba").unwrap();
    assert_eq!(format.media_type(), "application/x-gba-rom");
    assert_eq!(format.extension(), "gba");
}

#[test]
fn test_gbc() {
    let format = FileFormat::from_file("fixtures/application/sample.gbc").unwrap();
    assert_eq!(format.media_type(), "application/x-gameboy-color-rom");
    assert_eq!(format.extension(), "gbc");
}

#[test]
fn test_gz() {
    let format = FileFormat::from_file("fixtures/application/sample.gz").unwrap();
    assert_eq!(format.media_type(), "application/gzip");
    assert_eq!(format.extension(), "gz");
}

#[test]
fn test_indd() {
    let format = FileFormat::from_file("fixtures/application/sample.indd").unwrap();
    assert_eq!(format.media_type(), "application/x-indesign");
    assert_eq!(format.extension(), "indd");
}

#[test]
fn test_iso() {
    let format = FileFormat::from_file("fixtures/application/sample.iso").unwrap();
    assert_eq!(format.media_type(), "application/x-iso9660-image");
    assert_eq!(format.extension(), "iso");
}

#[test]
fn test_lnk() {
    let format = FileFormat::from_file("fixtures/application/sample.lnk").unwrap();
    assert_eq!(format.media_type(), "application/x-ms-shortcut");
    assert_eq!(format.extension(), "lnk");
}

#[test]
fn test_lrz() {
    let format = FileFormat::from_file("fixtures/application/sample.lrz").unwrap();
    assert_eq!(format.media_type(), "application/x-lrzip");
    assert_eq!(format.extension(), "lrz");
}

#[test]
fn test_lz() {
    let format = FileFormat::from_file("fixtures/application/sample.lz").unwrap();
    assert_eq!(format.media_type(), "application/x-lzip");
    assert_eq!(format.extension(), "lz");
}

#[test]
fn test_lz4() {
    let format = FileFormat::from_file("fixtures/application/sample.lz4").unwrap();
    assert_eq!(format.media_type(), "application/x-lz4");
    assert_eq!(format.extension(), "lz4");
}

#[test]
fn test_lzfse() {
    let format = FileFormat::from_file("fixtures/application/sample.lzfse").unwrap();
    assert_eq!(format.media_type(), "application/x-lzfse");
    assert_eq!(format.extension(), "lzfse");
}

#[test]
fn test_lzh() {
    let format = FileFormat::from_file("fixtures/application/sample.lzh").unwrap();
    assert_eq!(format.media_type(), "application/x-lzh-compressed");
    assert_eq!(format.extension(), "lzh");
}

#[test]
fn test_lzo() {
    let format = FileFormat::from_file("fixtures/application/sample.lzo").unwrap();
    assert_eq!(format.media_type(), "application/x-lzop");
    assert_eq!(format.extension(), "lzo");
}

#[test]
fn test_mobi() {
    let format = FileFormat::from_file("fixtures/application/sample.mobi").unwrap();
    assert_eq!(format.media_type(), "application/x-mobipocket-ebook");
    assert_eq!(format.extension(), "mobi");
}

#[test]
fn test_mxf() {
    let format = FileFormat::from_file("fixtures/application/sample.mxf").unwrap();
    assert_eq!(format.media_type(), "application/mxf");
    assert_eq!(format.extension(), "mxf");
}

#[test]
fn test_nds() {
    let format = FileFormat::from_file("fixtures/application/sample.nds").unwrap();
    assert_eq!(format.media_type(), "application/x-nintendo-ds-rom");
    assert_eq!(format.extension(), "nds");
}

#[test]
fn test_nes() {
    let format = FileFormat::from_file("fixtures/application/sample.nes").unwrap();
    assert_eq!(format.media_type(), "application/x-nintendo-nes-rom");
    assert_eq!(format.extension(), "nes");
}

#[test]
fn test_odg() {
    let format = FileFormat::from_file("fixtures/application/sample.odg").unwrap();
    assert_eq!(
        format.media_type(),
        "application/vnd.oasis.opendocument.graphics"
    );
    assert_eq!(format.extension(), "odg");
}

#[test]
fn test_odp() {
    let format = FileFormat::from_file("fixtures/application/sample.odp").unwrap();
    assert_eq!(
        format.media_type(),
        "application/vnd.oasis.opendocument.presentation"
    );
    assert_eq!(format.extension(), "odp");
}

#[test]
fn test_ods() {
    let format = FileFormat::from_file("fixtures/application/sample.ods").unwrap();
    assert_eq!(
        format.media_type(),
        "application/vnd.oasis.opendocument.spreadsheet"
    );
    assert_eq!(format.extension(), "ods");
}

#[test]
fn test_odt() {
    let format = FileFormat::from_file("fixtures/application/sample.odt").unwrap();
    assert_eq!(
        format.media_type(),
        "application/vnd.oasis.opendocument.text"
    );
    assert_eq!(format.extension(), "odt");
}

#[test]
fn test_ogx() {
    let format = FileFormat::from_file("fixtures/application/sample.ogx").unwrap();
    assert_eq!(format.media_type(), "application/ogg");
    assert_eq!(format.extension(), "ogx");
}

#[test]
fn test_pcap() {
    let format = FileFormat::from_file("fixtures/application/sample.pcap").unwrap();
    assert_eq!(format.media_type(), "application/vnd.tcpdump.pcap");
    assert_eq!(format.extension(), "pcap");
}

#[test]
fn test_pcapng() {
    let format = FileFormat::from_file("fixtures/application/sample.pcapng").unwrap();
    assert_eq!(format.media_type(), "application/x-pcapng");
    assert_eq!(format.extension(), "pcapng");
}

#[test]
fn test_pdf() {
    let format = FileFormat::from_file("fixtures/application/sample.pdf").unwrap();
    assert_eq!(format.media_type(), "application/pdf");
    assert_eq!(format.extension(), "pdf");
}

#[test]
fn test_rar() {
    let format = FileFormat::from_file("fixtures/application/sample.rar").unwrap();
    assert_eq!(format.media_type(), "application/vnd.rar");
    assert_eq!(format.extension(), "rar");
}

#[test]
fn test_rpm() {
    let format = FileFormat::from_file("fixtures/application/sample.rpm").unwrap();
    assert_eq!(format.media_type(), "application/x-rpm");
    assert_eq!(format.extension(), "rpm");
}

#[test]
fn test_sbx() {
    let format = FileFormat::from_file("fixtures/application/sample.sbx").unwrap();
    assert_eq!(format.media_type(), "application/x-sbx");
    assert_eq!(format.extension(), "sbx");
}

#[test]
fn test_shp() {
    let format = FileFormat::from_file("fixtures/application/sample.shp").unwrap();
    assert_eq!(format.media_type(), "application/x-esri-shape");
    assert_eq!(format.extension(), "shp");
}

#[test]
fn test_skp() {
    let format = FileFormat::from_file("fixtures/application/sample.skp").unwrap();
    assert_eq!(format.media_type(), "application/vnd.sketchup.skp");
    assert_eq!(format.extension(), "skp");
}

#[test]
fn test_sqlite() {
    let format = FileFormat::from_file("fixtures/application/sample.sqlite").unwrap();
    assert_eq!(format.media_type(), "application/vnd.sqlite3");
    assert_eq!(format.extension(), "sqlite");
}

#[test]
fn test_swf() {
    let format = FileFormat::from_file("fixtures/application/sample.swf").unwrap();
    assert_eq!(format.media_type(), "application/x-shockwave-flash");
    assert_eq!(format.extension(), "swf");
}

#[test]
fn test_sz() {
    let format = FileFormat::from_file("fixtures/application/sample.sz").unwrap();
    assert_eq!(format.media_type(), "application/x-snappy-framed");
    assert_eq!(format.extension(), "sz");
}

#[test]
fn test_tar() {
    let format = FileFormat::from_file("fixtures/application/sample.tar").unwrap();
    assert_eq!(format.media_type(), "application/x-tar");
    assert_eq!(format.extension(), "tar");
}

#[test]
fn test_vdi() {
    let format = FileFormat::from_file("fixtures/application/sample.vdi").unwrap();
    assert_eq!(format.media_type(), "application/x-virtualbox-vdi");
    assert_eq!(format.extension(), "vdi");
}

#[test]
fn test_vhd() {
    let format = FileFormat::from_file("fixtures/application/sample.vhd").unwrap();
    assert_eq!(format.media_type(), "application/x-vhd");
    assert_eq!(format.extension(), "vhd");
}

#[test]
fn test_vhdx() {
    let format = FileFormat::from_file("fixtures/application/sample.vhdx").unwrap();
    assert_eq!(format.media_type(), "application/x-vhdx");
    assert_eq!(format.extension(), "vhdx");
}

#[test]
fn test_wasm() {
    let format = FileFormat::from_file("fixtures/application/sample.wasm").unwrap();
    assert_eq!(format.media_type(), "application/wasm");
    assert_eq!(format.extension(), "wasm");
}

#[test]
fn test_xar() {
    let format = FileFormat::from_file("fixtures/application/sample.xar").unwrap();
    assert_eq!(format.media_type(), "application/x-xar");
    assert_eq!(format.extension(), "xar");
}

#[test]
fn test_xz() {
    let format = FileFormat::from_file("fixtures/application/sample.xz").unwrap();
    assert_eq!(format.media_type(), "application/x-xz");
    assert_eq!(format.extension(), "xz");
}

#[test]
fn test_z() {
    let format = FileFormat::from_file("fixtures/application/sample.Z").unwrap();
    assert_eq!(format.media_type(), "application/x-compress");
    assert_eq!(format.extension(), "Z");
}

#[test]
fn test_z64() {
    let format = FileFormat::from_file("fixtures/application/sample.z64").unwrap();
    assert_eq!(format.media_type(), "application/x-n64-rom");
    assert_eq!(format.extension(), "z64");
}

#[test]
fn test_zip() {
    let format = FileFormat::from_file("fixtures/application/sample.zip").unwrap();
    assert_eq!(format.media_type(), "application/zip");
    assert_eq!(format.extension(), "zip");
}

#[test]
fn test_zoo() {
    let format = FileFormat::from_file("fixtures/application/sample.zoo").unwrap();
    assert_eq!(format.media_type(), "application/x-zoo");
    assert_eq!(format.extension(), "zoo");
}

#[test]
fn test_zst() {
    let format = FileFormat::from_file("fixtures/application/sample.zst").unwrap();
    assert_eq!(format.media_type(), "application/zstd");
    assert_eq!(format.extension(), "zst");
}
