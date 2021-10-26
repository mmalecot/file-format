use file_format::FileFormat;

#[test]
fn test_7z() {
    let format = FileFormat::from_file("fixtures/application/sample.7z").unwrap();
    assert_eq!(format, FileFormat::SevenZip);
}

#[test]
fn test_alias() {
    let format = FileFormat::from_file("fixtures/application/sample.alias").unwrap();
    assert_eq!(format, FileFormat::MacOsAlias);
}

#[test]
fn test_alz() {
    let format = FileFormat::from_file("fixtures/application/sample.alz").unwrap();
    assert_eq!(format, FileFormat::Alz);
}

#[test]
fn test_ani() {
    let format = FileFormat::from_file("fixtures/application/sample.ani").unwrap();
    assert_eq!(format, FileFormat::Ani);
}

#[test]
fn test_ar() {
    let format = FileFormat::from_file("fixtures/application/sample.ar").unwrap();
    assert_eq!(format, FileFormat::UnixArchiver);
}

#[test]
fn test_arj() {
    let format = FileFormat::from_file("fixtures/application/sample.arj").unwrap();
    assert_eq!(format, FileFormat::ArchivedByRobertJung);
}

#[test]
fn test_arrow() {
    let format = FileFormat::from_file("fixtures/application/sample.arrow").unwrap();
    assert_eq!(format, FileFormat::ApacheArrowColumnar);
}

#[test]
fn test_bin() {
    let format = FileFormat::from_file("fixtures/application/sample.bin").unwrap();
    assert_eq!(format, FileFormat::ArbitraryBinaryData);
}

#[test]
fn test_blend() {
    let format = FileFormat::from_file("fixtures/application/sample.blend").unwrap();
    assert_eq!(format, FileFormat::Blender);
}

#[test]
fn test_bz2() {
    let format = FileFormat::from_file("fixtures/application/sample.bz2").unwrap();
    assert_eq!(format, FileFormat::Bzip2);
}

#[test]
fn test_cab() {
    let format = FileFormat::from_file("fixtures/application/sample.cab").unwrap();
    assert_eq!(format, FileFormat::Cabinet);
}

#[test]
fn test_cfb() {
    let format = FileFormat::from_file("fixtures/application/sample.cfb").unwrap();
    assert_eq!(format, FileFormat::CompoundFileBinary);
}

#[test]
fn test_chm() {
    let format = FileFormat::from_file("fixtures/application/sample.chm").unwrap();
    assert_eq!(format, FileFormat::MicrosoftCompiledHtmlHelp);
}

#[test]
fn test_class() {
    let format = FileFormat::from_file("fixtures/application/sample.class").unwrap();
    assert_eq!(format, FileFormat::JavaClass);
}

#[test]
fn test_cpio() {
    let format = FileFormat::from_file("fixtures/application/sample.cpio").unwrap();
    assert_eq!(format, FileFormat::Cpio);
}

#[test]
fn test_crx() {
    let format = FileFormat::from_file("fixtures/application/sample.crx").unwrap();
    assert_eq!(format, FileFormat::GoogleChromeExtension);
}

#[test]
fn test_dcm() {
    let format = FileFormat::from_file("fixtures/application/sample.dcm").unwrap();
    assert_eq!(
        format,
        FileFormat::DigitalImagingAndCommunicationsInMedicine
    );
}

#[test]
fn test_deb() {
    let format = FileFormat::from_file("fixtures/application/sample.deb").unwrap();
    assert_eq!(format, FileFormat::DebianBinaryPackage);
}

#[test]
fn test_dex() {
    let format = FileFormat::from_file("fixtures/application/sample.dex").unwrap();
    assert_eq!(format, FileFormat::DalvikExecutable);
}

#[test]
fn test_dmg() {
    let format = FileFormat::from_file("fixtures/application/sample.dmg").unwrap();
    assert_eq!(format, FileFormat::AppleDiskImage);
}

#[test]
fn test_docx() {
    let format = FileFormat::from_file("fixtures/application/sample.docx").unwrap();
    assert_eq!(format, FileFormat::MicrosoftWord2007);
}

#[test]
fn test_elf() {
    let format = FileFormat::from_file("fixtures/application/sample.elf").unwrap();
    assert_eq!(format, FileFormat::ExecutableAndLinkableFormat);
}

#[test]
fn test_eot() {
    let format = FileFormat::from_file("fixtures/application/sample.eot").unwrap();
    assert_eq!(format, FileFormat::EmbeddedOpenType);
}

#[test]
fn test_epub() {
    let format = FileFormat::from_file("fixtures/application/sample.epub").unwrap();
    assert_eq!(format, FileFormat::ElectronicPublication);
}

#[test]
fn test_exe() {
    let format = FileFormat::from_file("fixtures/application/sample.exe").unwrap();
    assert_eq!(format, FileFormat::WindowsExecutable);
}

#[test]
fn test_gb() {
    let format = FileFormat::from_file("fixtures/application/sample.gb").unwrap();
    assert_eq!(format, FileFormat::GameBoyRom);
}

#[test]
fn test_gba() {
    let format = FileFormat::from_file("fixtures/application/sample.gba").unwrap();
    assert_eq!(format, FileFormat::GameBoyAdvanceRom);
}

#[test]
fn test_gbc() {
    let format = FileFormat::from_file("fixtures/application/sample.gbc").unwrap();
    assert_eq!(format, FileFormat::GameBoyColorRom);
}

#[test]
fn test_gz() {
    let format = FileFormat::from_file("fixtures/application/sample.gz").unwrap();
    assert_eq!(format, FileFormat::Gzip);
}

#[test]
fn test_indd() {
    let format = FileFormat::from_file("fixtures/application/sample.indd").unwrap();
    assert_eq!(format, FileFormat::AdobeInDesignDocument);
}

#[test]
fn test_iso() {
    let format = FileFormat::from_file("fixtures/application/sample.iso").unwrap();
    assert_eq!(format, FileFormat::Iso9660);
}

#[test]
fn test_jar() {
    let format = FileFormat::from_file("fixtures/application/sample.jar").unwrap();
    assert_eq!(format, FileFormat::JavaArchive);
}

#[test]
fn test_jks() {
    let format = FileFormat::from_file("fixtures/application/sample.jks").unwrap();
    assert_eq!(format, FileFormat::JavaKeyStore);
}

#[test]
fn test_lnk() {
    let format = FileFormat::from_file("fixtures/application/sample.lnk").unwrap();
    assert_eq!(format, FileFormat::WindowsShortcut);
}

#[test]
fn test_lrz() {
    let format = FileFormat::from_file("fixtures/application/sample.lrz").unwrap();
    assert_eq!(format, FileFormat::LongRangeZip);
}

#[test]
fn test_lz() {
    let format = FileFormat::from_file("fixtures/application/sample.lz").unwrap();
    assert_eq!(format, FileFormat::Lzip);
}

#[test]
fn test_lz4() {
    let format = FileFormat::from_file("fixtures/application/sample.lz4").unwrap();
    assert_eq!(format, FileFormat::Lz4);
}

#[test]
fn test_lzfse() {
    let format = FileFormat::from_file("fixtures/application/sample.lzfse").unwrap();
    assert_eq!(format, FileFormat::LempelZivFiniteStateEntropy);
}

#[test]
fn test_lzh() {
    let format = FileFormat::from_file("fixtures/application/sample.lzh").unwrap();
    assert_eq!(format, FileFormat::Lha);
}

#[test]
fn test_lzo() {
    let format = FileFormat::from_file("fixtures/application/sample.lzo").unwrap();
    assert_eq!(format, FileFormat::Lzop);
}

#[test]
fn test_mobi() {
    let format = FileFormat::from_file("fixtures/application/sample.mobi").unwrap();
    assert_eq!(format, FileFormat::Mobipocket);
}

#[test]
fn test_mxf() {
    let format = FileFormat::from_file("fixtures/application/sample.mxf").unwrap();
    assert_eq!(format, FileFormat::MaterialExchangeFormat);
}

#[test]
fn test_nds() {
    let format = FileFormat::from_file("fixtures/application/sample.nds").unwrap();
    assert_eq!(format, FileFormat::NintendoDsRom);
}

#[test]
fn test_nes() {
    let format = FileFormat::from_file("fixtures/application/sample.nes").unwrap();
    assert_eq!(format, FileFormat::NintendoEntertainmentSystemRom);
}

#[test]
fn test_odg() {
    let format = FileFormat::from_file("fixtures/application/sample.odg").unwrap();
    assert_eq!(format, FileFormat::OpenDocumentGraphics);
}

#[test]
fn test_odp() {
    let format = FileFormat::from_file("fixtures/application/sample.odp").unwrap();
    assert_eq!(format, FileFormat::OpenDocumentPresentation);
}

#[test]
fn test_ods() {
    let format = FileFormat::from_file("fixtures/application/sample.ods").unwrap();
    assert_eq!(format, FileFormat::OpenDocumentSpreadSheet);
}

#[test]
fn test_odt() {
    let format = FileFormat::from_file("fixtures/application/sample.odt").unwrap();
    assert_eq!(format, FileFormat::OpenDocumentText);
}

#[test]
fn test_ogx() {
    let format = FileFormat::from_file("fixtures/application/sample.ogx").unwrap();
    assert_eq!(format, FileFormat::OggMultiplexedMedia);
}

#[test]
fn test_pcap() {
    let format = FileFormat::from_file("fixtures/application/sample.pcap").unwrap();
    assert_eq!(format, FileFormat::PcapDump);
}

#[test]
fn test_pcapng() {
    let format = FileFormat::from_file("fixtures/application/sample.pcapng").unwrap();
    assert_eq!(format, FileFormat::PcapNextGenerationDump);
}

#[test]
fn test_pdf() {
    let format = FileFormat::from_file("fixtures/application/sample.pdf").unwrap();
    assert_eq!(format, FileFormat::PortableDocumentFormat);
}

#[test]
fn test_pptx() {
    let format = FileFormat::from_file("fixtures/application/sample.pptx").unwrap();
    assert_eq!(format, FileFormat::MicrosoftPowerPoint2007);
}

#[test]
fn test_rar() {
    let format = FileFormat::from_file("fixtures/application/sample.rar").unwrap();
    assert_eq!(format, FileFormat::RoshalArchive);
}

#[test]
fn test_rpm() {
    let format = FileFormat::from_file("fixtures/application/sample.rpm").unwrap();
    assert_eq!(format, FileFormat::RedHatPackageManager);
}

#[test]
fn test_sbx() {
    let format = FileFormat::from_file("fixtures/application/sample.sbx").unwrap();
    assert_eq!(format, FileFormat::SeqBox);
}

#[test]
fn test_shp() {
    let format = FileFormat::from_file("fixtures/application/sample.shp").unwrap();
    assert_eq!(format, FileFormat::Shapefile);
}

#[test]
fn test_skp() {
    let format = FileFormat::from_file("fixtures/application/sample.skp").unwrap();
    assert_eq!(format, FileFormat::SketchUp);
}

#[test]
fn test_sqlite() {
    let format = FileFormat::from_file("fixtures/application/sample.sqlite").unwrap();
    assert_eq!(format, FileFormat::Sqlite3);
}

#[test]
fn test_swf() {
    let format = FileFormat::from_file("fixtures/application/sample.swf").unwrap();
    assert_eq!(format, FileFormat::SmallWebFormat);
}

#[test]
fn test_sz() {
    let format = FileFormat::from_file("fixtures/application/sample.sz").unwrap();
    assert_eq!(format, FileFormat::Snappy);
}

#[test]
fn test_tar() {
    let format = FileFormat::from_file("fixtures/application/sample.tar").unwrap();
    assert_eq!(format, FileFormat::TapeArchive);
}

#[test]
fn test_vdi() {
    let format = FileFormat::from_file("fixtures/application/sample.vdi").unwrap();
    assert_eq!(format, FileFormat::VirtualBoxVirtualDiskImage);
}

#[test]
fn test_vhd() {
    let format = FileFormat::from_file("fixtures/application/sample.vhd").unwrap();
    assert_eq!(format, FileFormat::MicrosoftVirtualHardDisk);
}

#[test]
fn test_vhdx() {
    let format = FileFormat::from_file("fixtures/application/sample.vhdx").unwrap();
    assert_eq!(format, FileFormat::MicrosoftVirtualHardDisk2);
}

#[test]
fn test_vsdx() {
    let format = FileFormat::from_file("fixtures/application/sample.vsdx").unwrap();
    assert_eq!(format, FileFormat::MicrosoftVisio2013);
}

#[test]
fn test_war() {
    let format = FileFormat::from_file("fixtures/application/sample.war").unwrap();
    assert_eq!(format, FileFormat::WebApplicationArchive);
}

#[test]
fn test_wasm() {
    let format = FileFormat::from_file("fixtures/application/sample.wasm").unwrap();
    assert_eq!(format, FileFormat::WebAssemblyBinary);
}

#[test]
fn test_xap() {
    let format = FileFormat::from_file("fixtures/application/sample.xap").unwrap();
    assert_eq!(format, FileFormat::Xap);
}

#[test]
fn test_xar() {
    let format = FileFormat::from_file("fixtures/application/sample.xar").unwrap();
    assert_eq!(format, FileFormat::ExtensibleArchive);
}

#[test]
fn test_xlsx() {
    let format = FileFormat::from_file("fixtures/application/sample.xlsx").unwrap();
    assert_eq!(format, FileFormat::MicrosoftExcel2007);
}

#[test]
fn test_xpi() {
    let format = FileFormat::from_file("fixtures/application/sample.xpi").unwrap();
    assert_eq!(format, FileFormat::Xpinstall);
}

#[test]
fn test_xz() {
    let format = FileFormat::from_file("fixtures/application/sample.xz").unwrap();
    assert_eq!(format, FileFormat::Xz);
}

#[test]
fn test_z() {
    let format = FileFormat::from_file("fixtures/application/sample.Z").unwrap();
    assert_eq!(format, FileFormat::UnixCompress);
}

#[test]
fn test_z64() {
    let format = FileFormat::from_file("fixtures/application/sample.z64").unwrap();
    assert_eq!(format, FileFormat::Nintendo64Rom);
}

#[test]
fn test_zip() {
    let format = FileFormat::from_file("fixtures/application/sample.zip").unwrap();
    assert_eq!(format, FileFormat::Zip);
}

#[test]
fn test_zoo() {
    let format = FileFormat::from_file("fixtures/application/sample.zoo").unwrap();
    assert_eq!(format, FileFormat::Zoo);
}

#[test]
fn test_zst() {
    let format = FileFormat::from_file("fixtures/application/sample.zst").unwrap();
    assert_eq!(format, FileFormat::Zstandard);
}
