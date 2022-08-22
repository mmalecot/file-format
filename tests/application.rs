use file_format::FileFormat;

#[test]
fn test_adobe_in_design_document() {
    let format = FileFormat::from_file("fixtures/application/sample.indd").unwrap();
    assert_eq!(format, FileFormat::AdobeInDesignDocument);
}

#[test]
fn test_alz() {
    let format = FileFormat::from_file("fixtures/application/sample.alz").unwrap();
    assert_eq!(format, FileFormat::Alz);
}

#[test]
fn test_android_binary_xml() {
    let format = FileFormat::from_file("fixtures/application/sample.xml").unwrap();
    assert_eq!(format, FileFormat::AndroidBinaryXml);
}

#[test]
fn test_android_compiled_resources() {
    let format = FileFormat::from_file("fixtures/application/sample.arsc").unwrap();
    assert_eq!(format, FileFormat::AndroidCompiledResources);
}

#[test]
fn test_ani() {
    let format = FileFormat::from_file("fixtures/application/sample.ani").unwrap();
    assert_eq!(format, FileFormat::Ani);
}

#[test]
fn test_apache_arrow_columnar() {
    let format = FileFormat::from_file("fixtures/application/sample.arrow").unwrap();
    assert_eq!(format, FileFormat::ApacheArrowColumnar);
}

#[test]
fn test_apple_disk_image() {
    let format = FileFormat::from_file("fixtures/application/sample.dmg").unwrap();
    assert_eq!(format, FileFormat::AppleDiskImage);
}

#[test]
fn test_arbitrary_binary_data() {
    let format = FileFormat::from_file("fixtures/application/sample.bin").unwrap();
    assert_eq!(format, FileFormat::ArbitraryBinaryData);
}

#[test]
fn test_archived_by_robert_jung() {
    let format = FileFormat::from_file("fixtures/application/sample.arj").unwrap();
    assert_eq!(format, FileFormat::ArchivedByRobertJung);
}

#[test]
fn test_blender() {
    let format = FileFormat::from_file("fixtures/application/sample.blend").unwrap();
    assert_eq!(format, FileFormat::Blender);
}

#[test]
fn test_bzip2() {
    let format = FileFormat::from_file("fixtures/application/sample.bz2").unwrap();
    assert_eq!(format, FileFormat::Bzip2);
}

#[test]
fn test_cabinet() {
    let format = FileFormat::from_file("fixtures/application/sample.cab").unwrap();
    assert_eq!(format, FileFormat::Cabinet);
}

#[test]
fn test_compound_file_binary() {
    let format = FileFormat::from_file("fixtures/application/sample.cfb").unwrap();
    assert_eq!(format, FileFormat::CompoundFileBinary);
}

#[test]
fn test_cpio() {
    let format = FileFormat::from_file("fixtures/application/sample.cpio").unwrap();
    assert_eq!(format, FileFormat::Cpio);
}

#[test]
fn test_dalvik_executable() {
    let format = FileFormat::from_file("fixtures/application/sample.dex").unwrap();
    assert_eq!(format, FileFormat::DalvikExecutable);
}

#[test]
fn test_debian_binary_package() {
    let format = FileFormat::from_file("fixtures/application/sample.deb").unwrap();
    assert_eq!(format, FileFormat::DebianBinaryPackage);
}

#[test]
fn test_digital_imaging_and_communications_in_medicine() {
    let format = FileFormat::from_file("fixtures/application/sample.dcm").unwrap();
    assert_eq!(
        format,
        FileFormat::DigitalImagingAndCommunicationsInMedicine
    );
}

#[test]
fn test_embedded_open_type() {
    let format = FileFormat::from_file("fixtures/application/sample.eot").unwrap();
    assert_eq!(format, FileFormat::EmbeddedOpenType);
}

#[test]
fn test_executable_and_linkable_format() {
    let format = FileFormat::from_file("fixtures/application/sample.elf").unwrap();
    assert_eq!(format, FileFormat::ExecutableAndLinkableFormat);
}

#[test]
fn test_extensible_archive() {
    let format = FileFormat::from_file("fixtures/application/sample.xar").unwrap();
    assert_eq!(format, FileFormat::ExtensibleArchive);
}

#[test]
fn test_game_boy_advance_rom() {
    let format = FileFormat::from_file("fixtures/application/sample.gba").unwrap();
    assert_eq!(format, FileFormat::GameBoyAdvanceRom);
}

#[test]
fn test_game_boy_color_rom() {
    let format = FileFormat::from_file("fixtures/application/sample.gbc").unwrap();
    assert_eq!(format, FileFormat::GameBoyColorRom);
}

#[test]
fn test_game_boy_rom() {
    let format = FileFormat::from_file("fixtures/application/sample.gb").unwrap();
    assert_eq!(format, FileFormat::GameBoyRom);
}

#[test]
fn test_google_chrome_extension() {
    let format = FileFormat::from_file("fixtures/application/sample.crx").unwrap();
    assert_eq!(format, FileFormat::GoogleChromeExtension);
}

#[test]
fn test_gzip() {
    let format = FileFormat::from_file("fixtures/application/sample.gz").unwrap();
    assert_eq!(format, FileFormat::Gzip);
}

#[test]
fn test_iso9660() {
    let format = FileFormat::from_file("fixtures/application/sample.iso").unwrap();
    assert_eq!(format, FileFormat::Iso9660);
}

#[test]
fn test_java_class() {
    let format = FileFormat::from_file("fixtures/application/sample.class").unwrap();
    assert_eq!(format, FileFormat::JavaClass);
}

#[test]
fn test_java_key_store() {
    let format = FileFormat::from_file("fixtures/application/sample.jks").unwrap();
    assert_eq!(format, FileFormat::JavaKeyStore);
}

#[test]
fn test_lempel_ziv_finite_state_entropy() {
    let format = FileFormat::from_file("fixtures/application/sample.lzfse").unwrap();
    assert_eq!(format, FileFormat::LempelZivFiniteStateEntropy);
}

#[test]
fn test_lha() {
    let format = FileFormat::from_file("fixtures/application/sample.lzh").unwrap();
    assert_eq!(format, FileFormat::Lha);
}

#[test]
fn test_long_range_zip() {
    let format = FileFormat::from_file("fixtures/application/sample.lrz").unwrap();
    assert_eq!(format, FileFormat::LongRangeZip);
}

#[test]
fn test_lz4() {
    let format = FileFormat::from_file("fixtures/application/sample.lz4").unwrap();
    assert_eq!(format, FileFormat::Lz4);
}

#[test]
fn test_lzip() {
    let format = FileFormat::from_file("fixtures/application/sample.lz").unwrap();
    assert_eq!(format, FileFormat::Lzip);
}

#[test]
fn test_lzop() {
    let format = FileFormat::from_file("fixtures/application/sample.lzo").unwrap();
    assert_eq!(format, FileFormat::Lzop);
}

#[test]
fn test_mac_os_alias() {
    let format = FileFormat::from_file("fixtures/application/sample.alias").unwrap();
    assert_eq!(format, FileFormat::MacOsAlias);
}

#[test]
fn test_material_exchange_format() {
    let format = FileFormat::from_file("fixtures/application/sample.mxf").unwrap();
    assert_eq!(format, FileFormat::MaterialExchangeFormat);
}

#[test]
fn test_microsoft_compiled_html_help() {
    let format = FileFormat::from_file("fixtures/application/sample.chm").unwrap();
    assert_eq!(format, FileFormat::MicrosoftCompiledHtmlHelp);
}

#[test]
fn test_microsoft_virtual_hard_disk() {
    let format = FileFormat::from_file("fixtures/application/sample.vhd").unwrap();
    assert_eq!(format, FileFormat::MicrosoftVirtualHardDisk);
}

#[test]
fn test_microsoft_virtual_hard_disk2() {
    let format = FileFormat::from_file("fixtures/application/sample.vhdx").unwrap();
    assert_eq!(format, FileFormat::MicrosoftVirtualHardDisk2);
}

#[test]
fn test_mobipocket() {
    let format = FileFormat::from_file("fixtures/application/sample.mobi").unwrap();
    assert_eq!(format, FileFormat::Mobipocket);
}

#[test]
fn test_nintendo64_rom() {
    let format = FileFormat::from_file("fixtures/application/sample.z64").unwrap();
    assert_eq!(format, FileFormat::Nintendo64Rom);
}

#[test]
fn test_nintendo_ds_rom() {
    let format = FileFormat::from_file("fixtures/application/sample.nds").unwrap();
    assert_eq!(format, FileFormat::NintendoDsRom);
}

#[test]
fn test_nintendo_entertainment_system_rom() {
    let format = FileFormat::from_file("fixtures/application/sample.nes").unwrap();
    assert_eq!(format, FileFormat::NintendoEntertainmentSystemRom);
}

#[test]
fn test_ogg_multiplexed_media() {
    let format = FileFormat::from_file("fixtures/application/sample.ogx").unwrap();
    assert_eq!(format, FileFormat::OggMultiplexedMedia);
}

#[test]
fn test_optimized_dalvik_executable() {
    let format = FileFormat::from_file("fixtures/application/sample.dey").unwrap();
    assert_eq!(format, FileFormat::OptimizedDalvikExecutable);
}

#[test]
fn test_pcap_dump() {
    let format = FileFormat::from_file("fixtures/application/sample.pcap").unwrap();
    assert_eq!(format, FileFormat::PcapDump);
}

#[test]
fn test_pcap_next_generation_dump() {
    let format = FileFormat::from_file("fixtures/application/sample.pcapng").unwrap();
    assert_eq!(format, FileFormat::PcapNextGenerationDump);
}

#[test]
fn test_portable_document_format() {
    let format = FileFormat::from_file("fixtures/application/sample.pdf").unwrap();
    assert_eq!(format, FileFormat::PortableDocumentFormat);
}

#[test]
fn test_red_hat_package_manager() {
    let format = FileFormat::from_file("fixtures/application/sample.rpm").unwrap();
    assert_eq!(format, FileFormat::RedHatPackageManager);
}

#[test]
fn test_roshal_archive() {
    let format = FileFormat::from_file("fixtures/application/sample.rar").unwrap();
    assert_eq!(format, FileFormat::RoshalArchive);
}

#[test]
fn test_seq_box() {
    let format = FileFormat::from_file("fixtures/application/sample.sbx").unwrap();
    assert_eq!(format, FileFormat::SeqBox);
}

#[test]
fn test_seven_zip() {
    let format = FileFormat::from_file("fixtures/application/sample.7z").unwrap();
    assert_eq!(format, FileFormat::SevenZip);
}

#[test]
fn test_shapefile() {
    let format = FileFormat::from_file("fixtures/application/sample.shp").unwrap();
    assert_eq!(format, FileFormat::Shapefile);
}

#[test]
fn test_sketch_up() {
    let format = FileFormat::from_file("fixtures/application/sample.skp").unwrap();
    assert_eq!(format, FileFormat::SketchUp);
}

#[test]
fn test_small_web_format() {
    let format = FileFormat::from_file("fixtures/application/sample.swf").unwrap();
    assert_eq!(format, FileFormat::SmallWebFormat);
}

#[test]
fn test_snappy() {
    let format = FileFormat::from_file("fixtures/application/sample.sz").unwrap();
    assert_eq!(format, FileFormat::Snappy);
}

#[test]
fn test_sqlite3() {
    let format = FileFormat::from_file("fixtures/application/sample.sqlite").unwrap();
    assert_eq!(format, FileFormat::Sqlite3);
}

#[test]
fn test_tape_archive() {
    let format = FileFormat::from_file("fixtures/application/sample.tar").unwrap();
    assert_eq!(format, FileFormat::TapeArchive);
}

#[test]
fn test_unix_archiver() {
    let format = FileFormat::from_file("fixtures/application/sample.ar").unwrap();
    assert_eq!(format, FileFormat::UnixArchiver);
}

#[test]
fn test_unix_compress() {
    let format = FileFormat::from_file("fixtures/application/sample.Z").unwrap();
    assert_eq!(format, FileFormat::UnixCompress);
}

#[test]
fn test_virtual_box_virtual_disk_image() {
    let format = FileFormat::from_file("fixtures/application/sample.vdi").unwrap();
    assert_eq!(format, FileFormat::VirtualBoxVirtualDiskImage);
}

#[test]
fn test_web_assembly_binary() {
    let format = FileFormat::from_file("fixtures/application/sample.wasm").unwrap();
    assert_eq!(format, FileFormat::WebAssemblyBinary);
}

#[test]
fn test_windows_executable() {
    let format = FileFormat::from_file("fixtures/application/sample.exe").unwrap();
    assert_eq!(format, FileFormat::WindowsExecutable);
}

#[test]
fn test_windows_shortcut() {
    let format = FileFormat::from_file("fixtures/application/sample.lnk").unwrap();
    assert_eq!(format, FileFormat::WindowsShortcut);
}

#[test]
fn test_xz() {
    let format = FileFormat::from_file("fixtures/application/sample.xz").unwrap();
    assert_eq!(format, FileFormat::Xz);
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
fn test_zstandard() {
    let format = FileFormat::from_file("fixtures/application/sample.zst").unwrap();
    assert_eq!(format, FileFormat::Zstandard);
}
