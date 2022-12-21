use file_format::FileFormat;

#[test]
fn test_adobe_illustrator_artwork() {
    let format = FileFormat::from_file("fixtures/application/sample.ai").unwrap();
    assert_eq!(format, FileFormat::AdobeIllustratorArtwork);
}

#[test]
fn test_adobe_indesign_document() {
    let format = FileFormat::from_file("fixtures/application/sample.indd").unwrap();
    assert_eq!(format, FileFormat::AdobeIndesignDocument);
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

#[cfg(feature = "zip")]
#[test]
fn test_android_package() {
    let format = FileFormat::from_file("fixtures/application/sample.apk").unwrap();
    assert_eq!(format, FileFormat::AndroidPackage);
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

#[cfg(feature = "zip")]
#[test]
fn test_circuit_diagram_document() {
    let format = FileFormat::from_file("fixtures/application/sample.cddx").unwrap();
    assert_eq!(format, FileFormat::CircuitDiagramDocument);
}

#[test]
fn test_common_object_file_format() {
    let format = FileFormat::from_file("fixtures/application/sample.coff").unwrap();
    assert_eq!(format, FileFormat::CommonObjectFileFormat);
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
fn test_der_certificate() {
    let format = FileFormat::from_file("fixtures/application/sample.der").unwrap();
    assert_eq!(format, FileFormat::DerCertificate);
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
fn test_dynamic_link_library() {
    let format = FileFormat::from_file("fixtures/application/sample.dll").unwrap();
    assert_eq!(format, FileFormat::DynamicLinkLibrary);
}

#[cfg(feature = "zip")]
#[test]
fn test_electronic_publication() {
    let format = FileFormat::from_file("fixtures/application/sample.epub").unwrap();
    assert_eq!(format, FileFormat::ElectronicPublication);
}

#[test]
fn test_embedded_opentype() {
    let format = FileFormat::from_file("fixtures/application/sample.eot").unwrap();
    assert_eq!(format, FileFormat::EmbeddedOpentype);
}

#[test]
fn test_encapsulated_postscript() {
    let format = FileFormat::from_file("fixtures/application/sample.eps").unwrap();
    assert_eq!(format, FileFormat::EncapsulatedPostscript);
}

#[cfg(feature = "zip")]
#[test]
fn test_enterprise_application_archive() {
    let format = FileFormat::from_file("fixtures/application/sample.ear").unwrap();
    assert_eq!(format, FileFormat::EnterpriseApplicationArchive);
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
fn test_extensible_stylesheet_language_transformations() {
    let format = FileFormat::from_file("fixtures/application/sample.xsl").unwrap();
    assert_eq!(
        format,
        FileFormat::ExtensibleStylesheetLanguageTransformations
    );
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
fn test_geography_markup_language() {
    let format = FileFormat::from_file("fixtures/application/sample.gml").unwrap();
    assert_eq!(format, FileFormat::GeographyMarkupLanguage);
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
fn test_icc_profile() {
    let format = FileFormat::from_file("fixtures/application/sample.icc").unwrap();
    assert_eq!(format, FileFormat::IccProfile);
}

#[cfg(feature = "zip")]
#[test]
fn test_ios_app_store_package() {
    let format = FileFormat::from_file("fixtures/application/sample.ipa").unwrap();
    assert_eq!(format, FileFormat::IosAppStorePackage);
}

#[test]
fn test_iso9660() {
    let format = FileFormat::from_file("fixtures/application/sample.iso").unwrap();
    assert_eq!(format, FileFormat::Iso9660);
}

#[cfg(feature = "zip")]
#[test]
fn test_java_archive() {
    let format = FileFormat::from_file("fixtures/application/sample.jar").unwrap();
    assert_eq!(format, FileFormat::JavaArchive);
}

#[test]
fn test_java_class() {
    let format = FileFormat::from_file("fixtures/application/sample.class").unwrap();
    assert_eq!(format, FileFormat::JavaClass);
}

#[test]
fn test_java_keystore() {
    let format = FileFormat::from_file("fixtures/application/sample.jks").unwrap();
    assert_eq!(format, FileFormat::JavaKeystore);
}

#[test]
fn test_keyhole_markup_language() {
    let format = FileFormat::from_file("fixtures/application/sample.kml").unwrap();
    assert_eq!(format, FileFormat::KeyholeMarkupLanguage);
}

#[cfg(feature = "zip")]
#[test]
fn test_keyhole_markup_language_zipped() {
    let format = FileFormat::from_file("fixtures/application/sample.kmz").unwrap();
    assert_eq!(format, FileFormat::KeyholeMarkupLanguageZipped);
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
fn test_lua_bytecode() {
    let format = FileFormat::from_file("fixtures/application/sample.luac").unwrap();
    assert_eq!(format, FileFormat::LuaBytecode);
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
fn test_mach_o() {
    let format = FileFormat::from_file("fixtures/application/sample.mach").unwrap();
    assert_eq!(format, FileFormat::MachO);
}

#[test]
fn test_macos_alias() {
    let format = FileFormat::from_file("fixtures/application/sample.alias").unwrap();
    assert_eq!(format, FileFormat::MacosAlias);
}

#[test]
fn test_material_exchange_format() {
    let format = FileFormat::from_file("fixtures/application/sample.mxf").unwrap();
    assert_eq!(format, FileFormat::MaterialExchangeFormat);
}

#[test]
fn test_meta_information_encapsulation() {
    let format = FileFormat::from_file("fixtures/application/sample.mie").unwrap();
    assert_eq!(format, FileFormat::MetaInformationEncapsulation);
}

#[test]
fn test_microsoft_access2007_database() {
    let format = FileFormat::from_file("fixtures/application/sample.accdb").unwrap();
    assert_eq!(format, FileFormat::MicrosoftAccess2007Database);
}

#[test]
fn test_microsoft_access_database() {
    let format = FileFormat::from_file("fixtures/application/sample.mdb").unwrap();
    assert_eq!(format, FileFormat::MicrosoftAccessDatabase);
}

#[test]
fn test_microsoft_compiled_html_help() {
    let format = FileFormat::from_file("fixtures/application/sample.chm").unwrap();
    assert_eq!(format, FileFormat::MicrosoftCompiledHtmlHelp);
}

#[cfg(feature = "cfb")]
#[test]
fn test_microsoft_excel_spreadsheet() {
    let format = FileFormat::from_file("fixtures/application/sample.xls").unwrap();
    assert_eq!(format, FileFormat::MicrosoftExcelSpreadsheet);
}

#[cfg(feature = "cfb")]
#[test]
fn test_microsoft_powerpoint_presentation() {
    let format = FileFormat::from_file("fixtures/application/sample.ppt").unwrap();
    assert_eq!(format, FileFormat::MicrosoftPowerpointPresentation);
}

#[cfg(feature = "cfb")]
#[test]
fn test_microsoft_project_plan() {
    let format = FileFormat::from_file("fixtures/application/sample.mpp").unwrap();
    assert_eq!(format, FileFormat::MicrosoftProjectPlan);
}

#[cfg(feature = "cfb")]
#[test]
fn test_microsoft_publisher_document() {
    let format = FileFormat::from_file("fixtures/application/sample.pub").unwrap();
    assert_eq!(format, FileFormat::MicrosoftPublisherDocument);
}

#[cfg(feature = "cfb")]
#[test]
fn test_microsoft_software_installer() {
    let format = FileFormat::from_file("fixtures/application/sample.msi").unwrap();
    assert_eq!(format, FileFormat::MicrosoftSoftwareInstaller);
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

#[cfg(feature = "cfb")]
#[test]
fn test_microsoft_visio_drawing() {
    let format = FileFormat::from_file("fixtures/application/sample.vsd").unwrap();
    assert_eq!(format, FileFormat::MicrosoftVisioDrawing);
}

#[cfg(feature = "zip")]
#[test]
fn test_microsoft_visual_studio_extension() {
    let format = FileFormat::from_file("fixtures/application/sample.vsix").unwrap();
    assert_eq!(format, FileFormat::MicrosoftVisualStudioExtension);
}

#[cfg(feature = "cfb")]
#[test]
fn test_microsoft_word_document() {
    let format = FileFormat::from_file("fixtures/application/sample.doc").unwrap();
    assert_eq!(format, FileFormat::MicrosoftWordDocument);
}

#[test]
fn test_mobipocket() {
    let format = FileFormat::from_file("fixtures/application/sample.mobi").unwrap();
    assert_eq!(format, FileFormat::Mobipocket);
}

#[test]
fn test_ms_dos_executable() {
    let format = FileFormat::from_file("fixtures/application/sample1.exe").unwrap();
    assert_eq!(format, FileFormat::MsDosExecutable);
}

#[test]
fn test_musicxml() {
    let format = FileFormat::from_file("fixtures/application/sample.musicxml").unwrap();
    assert_eq!(format, FileFormat::Musicxml);
}

#[cfg(feature = "zip")]
#[test]
fn test_musicxml_zipped() {
    let format = FileFormat::from_file("fixtures/application/sample.mxl").unwrap();
    assert_eq!(format, FileFormat::MusicxmlZipped);
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

#[cfg(feature = "zip")]
#[test]
fn test_office_open_xml_document() {
    let format = FileFormat::from_file("fixtures/application/sample.docx").unwrap();
    assert_eq!(format, FileFormat::OfficeOpenXmlDocument);
}

#[cfg(feature = "zip")]
#[test]
fn test_office_open_xml_drawing() {
    let format = FileFormat::from_file("fixtures/application/sample.vsdx").unwrap();
    assert_eq!(format, FileFormat::OfficeOpenXmlDrawing);
}

#[cfg(feature = "zip")]
#[test]
fn test_office_open_xml_presentation() {
    let format = FileFormat::from_file("fixtures/application/sample.pptx").unwrap();
    assert_eq!(format, FileFormat::OfficeOpenXmlPresentation);
}

#[cfg(feature = "zip")]
#[test]
fn test_office_open_xml_spreadsheet() {
    let format = FileFormat::from_file("fixtures/application/sample.xlsx").unwrap();
    assert_eq!(format, FileFormat::OfficeOpenXmlSpreadsheet);
}

#[test]
fn test_ogg_multiplexed_media() {
    let format = FileFormat::from_file("fixtures/application/sample.ogx").unwrap();
    assert_eq!(format, FileFormat::OggMultiplexedMedia);
}

#[cfg(feature = "zip")]
#[test]
fn test_open_document_graphics() {
    let format = FileFormat::from_file("fixtures/application/sample.odg").unwrap();
    assert_eq!(format, FileFormat::OpenDocumentGraphics);
}

#[cfg(feature = "zip")]
#[test]
fn test_open_document_presentation() {
    let format = FileFormat::from_file("fixtures/application/sample.odp").unwrap();
    assert_eq!(format, FileFormat::OpenDocumentPresentation);
}

#[cfg(feature = "zip")]
#[test]
fn test_open_document_spreadsheet() {
    let format = FileFormat::from_file("fixtures/application/sample.ods").unwrap();
    assert_eq!(format, FileFormat::OpenDocumentSpreadsheet);
}

#[cfg(feature = "zip")]
#[test]
fn test_open_document_text() {
    let format = FileFormat::from_file("fixtures/application/sample.odt").unwrap();
    assert_eq!(format, FileFormat::OpenDocumentText);
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
fn test_pem_certificate() {
    let format = FileFormat::from_file("fixtures/application/sample.crt").unwrap();
    assert_eq!(format, FileFormat::PemCertificate);
}

#[test]
fn test_pem_certificate_signing_request() {
    let format = FileFormat::from_file("fixtures/application/sample.csr").unwrap();
    assert_eq!(format, FileFormat::PemCertificateSigningRequest);
}

#[test]
fn test_pem_private_key() {
    let format = FileFormat::from_file("fixtures/application/sample.key").unwrap();
    assert_eq!(format, FileFormat::PemPrivateKey);
}

#[test]
fn test_pgp_message() {
    let format = FileFormat::from_file("fixtures/application/sample1.asc").unwrap();
    assert_eq!(format, FileFormat::PgpMessage);
}

#[test]
fn test_pgp_private_key_block() {
    let format = FileFormat::from_file("fixtures/application/sample2.asc").unwrap();
    assert_eq!(format, FileFormat::PgpPrivateKeyBlock);
}

#[test]
fn test_pgp_public_key_block() {
    let format = FileFormat::from_file("fixtures/application/sample3.asc").unwrap();
    assert_eq!(format, FileFormat::PgpPublicKeyBlock);
}

#[test]
fn test_pgp_signature() {
    let format = FileFormat::from_file("fixtures/application/sample4.asc").unwrap();
    assert_eq!(format, FileFormat::PgpSignature);
}

#[test]
fn test_pgp_signed_message() {
    let format = FileFormat::from_file("fixtures/application/sample5.asc").unwrap();
    assert_eq!(format, FileFormat::PgpSignedMessage);
}

#[test]
fn test_portable_document_format() {
    let format = FileFormat::from_file("fixtures/application/sample.pdf").unwrap();
    assert_eq!(format, FileFormat::PortableDocumentFormat);
}

#[test]
fn test_portable_executable() {
    let format = FileFormat::from_file("fixtures/application/sample2.exe").unwrap();
    assert_eq!(format, FileFormat::PortableExecutable);
}

#[test]
fn test_postscript() {
    let format = FileFormat::from_file("fixtures/application/sample.ps").unwrap();
    assert_eq!(format, FileFormat::Postscript);
}

#[test]
fn test_really_simple_syndication() {
    let format = FileFormat::from_file("fixtures/application/sample.rss").unwrap();
    assert_eq!(format, FileFormat::ReallySimpleSyndication);
}

#[test]
fn test_red_hat_package_manager() {
    let format = FileFormat::from_file("fixtures/application/sample.rpm").unwrap();
    assert_eq!(format, FileFormat::RedHatPackageManager);
}

#[test]
fn test_rich_text_format() {
    let format = FileFormat::from_file("fixtures/application/sample.rtf").unwrap();
    assert_eq!(format, FileFormat::RichTextFormat);
}

#[test]
fn test_roshal_archive() {
    let format = FileFormat::from_file("fixtures/application/sample.rar").unwrap();
    assert_eq!(format, FileFormat::RoshalArchive);
}

#[test]
fn test_seqbox() {
    let format = FileFormat::from_file("fixtures/application/sample.sbx").unwrap();
    assert_eq!(format, FileFormat::Seqbox);
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
fn test_simple_object_access_protocol() {
    let format = FileFormat::from_file("fixtures/application/sample.soap").unwrap();
    assert_eq!(format, FileFormat::SimpleObjectAccessProtocol);
}

#[test]
fn test_sketchup() {
    let format = FileFormat::from_file("fixtures/application/sample.skp").unwrap();
    assert_eq!(format, FileFormat::Sketchup);
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
fn test_tasty() {
    let format = FileFormat::from_file("fixtures/application/sample.tasty").unwrap();
    assert_eq!(format, FileFormat::Tasty);
}

#[cfg(feature = "zip")]
#[test]
fn test_three_dimensional_manufacturing_format() {
    let format = FileFormat::from_file("fixtures/application/sample.3mf").unwrap();
    assert_eq!(format, FileFormat::ThreeDimensionalManufacturingFormat);
}

#[test]
fn test_unix_archiver() {
    let format = FileFormat::from_file("fixtures/application/sample.a").unwrap();
    assert_eq!(format, FileFormat::UnixArchiver);
}

#[test]
fn test_unix_compress() {
    let format = FileFormat::from_file("fixtures/application/sample.Z").unwrap();
    assert_eq!(format, FileFormat::UnixCompress);
}

#[test]
fn test_virtualbox_virtual_disk_image() {
    let format = FileFormat::from_file("fixtures/application/sample.vdi").unwrap();
    assert_eq!(format, FileFormat::VirtualboxVirtualDiskImage);
}

#[cfg(feature = "zip")]
#[test]
fn test_web_application_archive() {
    let format = FileFormat::from_file("fixtures/application/sample.war").unwrap();
    assert_eq!(format, FileFormat::WebApplicationArchive);
}

#[test]
fn test_webassembly_binary() {
    let format = FileFormat::from_file("fixtures/application/sample.wasm").unwrap();
    assert_eq!(format, FileFormat::WebassemblyBinary);
}

#[test]
fn test_windows_animated_cursor() {
    let format = FileFormat::from_file("fixtures/application/sample.ani").unwrap();
    assert_eq!(format, FileFormat::WindowsAnimatedCursor);
}

#[cfg(feature = "zip")]
#[test]
fn test_windows_app_package() {
    let format = FileFormat::from_file("fixtures/application/sample.appx").unwrap();
    assert_eq!(format, FileFormat::WindowsAppPackage);
}

#[test]
fn test_windows_shortcut() {
    let format = FileFormat::from_file("fixtures/application/sample.lnk").unwrap();
    assert_eq!(format, FileFormat::WindowsShortcut);
}

#[cfg(feature = "zip")]
#[test]
fn test_xap() {
    let format = FileFormat::from_file("fixtures/application/sample.xap").unwrap();
    assert_eq!(format, FileFormat::Xap);
}

#[cfg(feature = "zip")]
#[test]
fn test_xpinstall() {
    let format = FileFormat::from_file("fixtures/application/sample.xpi").unwrap();
    assert_eq!(format, FileFormat::Xpinstall);
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
