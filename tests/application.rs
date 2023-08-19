use file_format::FileFormat;

#[test]
fn test_advanced_systems_format() {
    let fmt = FileFormat::from_file("fixtures/application/sample.asf").unwrap();
    assert_eq!(fmt, FileFormat::AdvancedSystemsFormat);
}

#[test]
fn test_android_binary_xml() {
    let fmt = FileFormat::from_file("fixtures/application/sample.xml").unwrap();
    assert_eq!(fmt, FileFormat::AndroidBinaryXml);
}

#[test]
fn test_android_compiled_resources() {
    let fmt = FileFormat::from_file("fixtures/application/sample.arsc").unwrap();
    assert_eq!(fmt, FileFormat::AndroidCompiledResources);
}

#[test]
fn test_apache_arrow_columnar() {
    let fmt = FileFormat::from_file("fixtures/application/sample.arrow").unwrap();
    assert_eq!(fmt, FileFormat::ApacheArrowColumnar);
}

#[test]
fn test_apache_avro_object_container() {
    let fmt = FileFormat::from_file("fixtures/application/sample.avro").unwrap();
    assert_eq!(fmt, FileFormat::ApacheAvroObjectContainer);
}

#[test]
fn test_apache_parquet() {
    let fmt = FileFormat::from_file("fixtures/application/sample.parquet").unwrap();
    assert_eq!(fmt, FileFormat::ApacheParquet);
}

#[test]
fn test_arbitrary_binary_data() {
    let fmt = FileFormat::from_file("fixtures/application/sample.bin").unwrap();
    assert_eq!(fmt, FileFormat::ArbitraryBinaryData);
}

#[test]
fn test_bittorrent_file() {
    let fmt = FileFormat::from_file("fixtures/application/sample.torrent").unwrap();
    assert_eq!(fmt, FileFormat::BittorrentFile);
}

#[test]
fn test_cd_audio() {
    let fmt = FileFormat::from_file("fixtures/application/sample.cda").unwrap();
    assert_eq!(fmt, FileFormat::CdAudio);
}

#[test]
fn test_compound_file_binary() {
    let fmt = FileFormat::from_file("fixtures/application/sample.cfb").unwrap();
    assert_eq!(fmt, FileFormat::CompoundFileBinary);
}

#[test]
fn test_digital_imaging_and_communications_in_medicine() {
    let fmt = FileFormat::from_file("fixtures/application/sample.dcm").unwrap();
    assert_eq!(fmt, FileFormat::DigitalImagingAndCommunicationsInMedicine);
}

#[test]
fn test_encapsulated_postscript() {
    let fmt = FileFormat::from_file("fixtures/application/sample.eps").unwrap();
    assert_eq!(fmt, FileFormat::EncapsulatedPostscript);
}

#[test]
fn test_extensible_binary_meta_language() {
    let fmt = FileFormat::from_file("fixtures/application/sample.ebml").unwrap();
    assert_eq!(fmt, FileFormat::ExtensibleBinaryMetaLanguage);
}

#[test]
fn test_extensible_stylesheet_language_transformations_1() {
    let fmt = FileFormat::from_file("fixtures/application/sample1.xsl").unwrap();
    assert_eq!(fmt, FileFormat::ExtensibleStylesheetLanguageTransformations);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_extensible_stylesheet_language_transformations_2() {
    let fmt = FileFormat::from_file("fixtures/application/sample2.xsl").unwrap();
    assert_eq!(fmt, FileFormat::ExtensibleStylesheetLanguageTransformations);
}

#[test]
fn test_flexible_image_transport_system() {
    let fmt = FileFormat::from_file("fixtures/application/sample.fits").unwrap();
    assert_eq!(fmt, FileFormat::FlexibleImageTransportSystem);
}

#[test]
fn test_gettext_machine_object() {
    let fmt = FileFormat::from_file("fixtures/application/sample.mo").unwrap();
    assert_eq!(fmt, FileFormat::GettextMachineObject);
}

#[test]
fn test_icc_profile() {
    let fmt = FileFormat::from_file("fixtures/application/sample.icc").unwrap();
    assert_eq!(fmt, FileFormat::IccProfile);
}

#[test]
fn test_java_keystore() {
    let fmt = FileFormat::from_file("fixtures/application/sample.jks").unwrap();
    assert_eq!(fmt, FileFormat::JavaKeystore);
}

#[test]
fn test_macos_alias() {
    let fmt = FileFormat::from_file("fixtures/application/sample.alias").unwrap();
    assert_eq!(fmt, FileFormat::MacosAlias);
}

#[test]
fn test_mathematical_markup_language_1() {
    let fmt = FileFormat::from_file("fixtures/application/sample1.mathml").unwrap();
    assert_eq!(fmt, FileFormat::MathematicalMarkupLanguage);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_mathematical_markup_language_2() {
    let fmt = FileFormat::from_file("fixtures/application/sample2.mathml").unwrap();
    assert_eq!(fmt, FileFormat::MathematicalMarkupLanguage);
}

#[test]
fn test_meta_information_encapsulation() {
    let fmt = FileFormat::from_file("fixtures/application/sample.mie").unwrap();
    assert_eq!(fmt, FileFormat::MetaInformationEncapsulation);
}

#[test]
fn test_microsoft_compiled_html_help() {
    let fmt = FileFormat::from_file("fixtures/application/sample.chm").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftCompiledHtmlHelp);
}

#[test]
fn test_microsoft_visual_studio_solution() {
    let fmt = FileFormat::from_file("fixtures/application/sample.sln").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftVisualStudioSolution);
}

#[test]
fn test_musicxml_1() {
    let fmt = FileFormat::from_file("fixtures/application/sample1.musicxml").unwrap();
    assert_eq!(fmt, FileFormat::Musicxml);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_musicxml_2() {
    let fmt = FileFormat::from_file("fixtures/application/sample2.musicxml").unwrap();
    assert_eq!(fmt, FileFormat::Musicxml);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_musicxml_zipped() {
    let fmt = FileFormat::from_file("fixtures/application/sample.mxl").unwrap();
    assert_eq!(fmt, FileFormat::MusicxmlZipped);
}

#[test]
fn test_ogg_multiplexed_media() {
    let fmt = FileFormat::from_file("fixtures/application/sample.ogx").unwrap();
    assert_eq!(fmt, FileFormat::OggMultiplexedMedia);
}

#[test]
fn test_pcap_dump() {
    let fmt = FileFormat::from_file("fixtures/application/sample.pcap").unwrap();
    assert_eq!(fmt, FileFormat::PcapDump);
}

#[test]
fn test_pcap_next_generation_dump() {
    let fmt = FileFormat::from_file("fixtures/application/sample.pcapng").unwrap();
    assert_eq!(fmt, FileFormat::PcapNextGenerationDump);
}

#[test]
fn test_pem_certificate_signing_request() {
    let fmt = FileFormat::from_file("fixtures/application/sample.csr").unwrap();
    assert_eq!(fmt, FileFormat::PemCertificateSigningRequest);
}

#[test]
fn test_pem_private_key() {
    let fmt = FileFormat::from_file("fixtures/application/sample.key").unwrap();
    assert_eq!(fmt, FileFormat::PemPrivateKey);
}

#[test]
fn test_pem_public_key() {
    let fmt = FileFormat::from_file("fixtures/application/sample.pub").unwrap();
    assert_eq!(fmt, FileFormat::PemPublicKey);
}

#[test]
fn test_personal_storage_table() {
    let fmt = FileFormat::from_file("fixtures/application/sample.pst").unwrap();
    assert_eq!(fmt, FileFormat::PersonalStorageTable);
}

#[test]
fn test_pgp_message() {
    let fmt = FileFormat::from_file("fixtures/application/sample1.asc").unwrap();
    assert_eq!(fmt, FileFormat::PgpMessage);
}

#[test]
fn test_pgp_private_key_block() {
    let fmt = FileFormat::from_file("fixtures/application/sample2.asc").unwrap();
    assert_eq!(fmt, FileFormat::PgpPrivateKeyBlock);
}

#[test]
fn test_pgp_public_key_block() {
    let fmt = FileFormat::from_file("fixtures/application/sample3.asc").unwrap();
    assert_eq!(fmt, FileFormat::PgpPublicKeyBlock);
}

#[test]
fn test_pgp_signature() {
    let fmt = FileFormat::from_file("fixtures/application/sample4.asc").unwrap();
    assert_eq!(fmt, FileFormat::PgpSignature);
}

#[test]
fn test_pgp_signed_message() {
    let fmt = FileFormat::from_file("fixtures/application/sample5.asc").unwrap();
    assert_eq!(fmt, FileFormat::PgpSignedMessage);
}

#[test]
fn test_postscript() {
    let fmt = FileFormat::from_file("fixtures/application/sample.ps").unwrap();
    assert_eq!(fmt, FileFormat::Postscript);
}

#[test]
fn test_realmedia() {
    let fmt = FileFormat::from_file("fixtures/application/sample.rm").unwrap();
    assert_eq!(fmt, FileFormat::Realmedia);
}

#[test]
fn test_simple_object_access_protocol_1() {
    let fmt = FileFormat::from_file("fixtures/application/sample1.soap").unwrap();
    assert_eq!(fmt, FileFormat::SimpleObjectAccessProtocol);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_simple_object_access_protocol_2() {
    let fmt = FileFormat::from_file("fixtures/application/sample2.soap").unwrap();
    assert_eq!(fmt, FileFormat::SimpleObjectAccessProtocol);
}

#[test]
fn test_small_web_format() {
    let fmt = FileFormat::from_file("fixtures/application/sample.swf").unwrap();
    assert_eq!(fmt, FileFormat::SmallWebFormat);
}

#[test]
fn test_tasty() {
    let fmt = FileFormat::from_file("fixtures/application/sample.tasty").unwrap();
    assert_eq!(fmt, FileFormat::Tasty);
}

#[test]
fn test_tiled_map_xml_1() {
    let fmt = FileFormat::from_file("fixtures/application/sample1.tmx").unwrap();
    assert_eq!(fmt, FileFormat::TiledMapXml);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_tiled_map_xml_2() {
    let fmt = FileFormat::from_file("fixtures/application/sample2.tmx").unwrap();
    assert_eq!(fmt, FileFormat::TiledMapXml);
}

#[test]
fn test_tiled_tileset_xml_1() {
    let fmt = FileFormat::from_file("fixtures/application/sample1.tsx").unwrap();
    assert_eq!(fmt, FileFormat::TiledTilesetXml);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_tiled_tileset_xml_2() {
    let fmt = FileFormat::from_file("fixtures/application/sample2.tsx").unwrap();
    assert_eq!(fmt, FileFormat::TiledTilesetXml);
}

#[test]
fn test_windows_shortcut() {
    let fmt = FileFormat::from_file("fixtures/application/sample.lnk").unwrap();
    assert_eq!(fmt, FileFormat::WindowsShortcut);
}

#[test]
fn test_xml_localization_interchange_file_format_1() {
    let fmt = FileFormat::from_file("fixtures/application/sample1.xlf").unwrap();
    assert_eq!(fmt, FileFormat::XmlLocalizationInterchangeFileFormat);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_xml_localization_interchange_file_format_2() {
    let fmt = FileFormat::from_file("fixtures/application/sample2.xlf").unwrap();
    assert_eq!(fmt, FileFormat::XmlLocalizationInterchangeFileFormat);
}
