use file_format::FileFormat;

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
fn test_apache_arrow_columnar() {
    let format = FileFormat::from_file("fixtures/application/sample.arrow").unwrap();
    assert_eq!(format, FileFormat::ApacheArrowColumnar);
}

#[test]
fn test_apache_avro_object_container() {
    let format = FileFormat::from_file("fixtures/application/sample.avro").unwrap();
    assert_eq!(format, FileFormat::ApacheAvroObjectContainer);
}

#[test]
fn test_apache_parquet() {
    let format = FileFormat::from_file("fixtures/application/sample.parquet").unwrap();
    assert_eq!(format, FileFormat::ApacheParquet);
}

#[test]
fn test_arbitrary_binary_data() {
    let format = FileFormat::from_file("fixtures/application/sample.bin").unwrap();
    assert_eq!(format, FileFormat::ArbitraryBinaryData);
}

#[test]
fn test_bittorrent_file() {
    let format = FileFormat::from_file("fixtures/application/sample.torrent").unwrap();
    assert_eq!(format, FileFormat::BittorrentFile);
}

#[test]
fn test_compound_file_binary() {
    let format = FileFormat::from_file("fixtures/application/sample.cfb").unwrap();
    assert_eq!(format, FileFormat::CompoundFileBinary);
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
fn test_encapsulated_postscript() {
    let format = FileFormat::from_file("fixtures/application/sample.eps").unwrap();
    assert_eq!(format, FileFormat::EncapsulatedPostscript);
}

#[test]
fn test_extensible_stylesheet_language_transformations_1() {
    let format = FileFormat::from_file("fixtures/application/sample1.xsl").unwrap();
    assert_eq!(
        format,
        FileFormat::ExtensibleStylesheetLanguageTransformations
    );
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_extensible_stylesheet_language_transformations_2() {
    let format = FileFormat::from_file("fixtures/application/sample2.xsl").unwrap();
    assert_eq!(
        format,
        FileFormat::ExtensibleStylesheetLanguageTransformations
    );
}

#[test]
fn test_flexible_image_transport_system() {
    let format = FileFormat::from_file("fixtures/application/sample.fits").unwrap();
    assert_eq!(format, FileFormat::FlexibleImageTransportSystem);
}

#[test]
fn test_gettext_machine_object() {
    let format = FileFormat::from_file("fixtures/application/sample.mo").unwrap();
    assert_eq!(format, FileFormat::GettextMachineObject);
}

#[test]
fn test_icc_profile() {
    let format = FileFormat::from_file("fixtures/application/sample.icc").unwrap();
    assert_eq!(format, FileFormat::IccProfile);
}

#[test]
fn test_java_keystore() {
    let format = FileFormat::from_file("fixtures/application/sample.jks").unwrap();
    assert_eq!(format, FileFormat::JavaKeystore);
}

#[test]
fn test_macos_alias() {
    let format = FileFormat::from_file("fixtures/application/sample.alias").unwrap();
    assert_eq!(format, FileFormat::MacosAlias);
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

#[test]
fn test_microsoft_visual_studio_solution() {
    let format = FileFormat::from_file("fixtures/application/sample.sln").unwrap();
    assert_eq!(format, FileFormat::MicrosoftVisualStudioSolution);
}

#[test]
fn test_musicxml_1() {
    let format = FileFormat::from_file("fixtures/application/sample1.musicxml").unwrap();
    assert_eq!(format, FileFormat::Musicxml);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_musicxml_2() {
    let format = FileFormat::from_file("fixtures/application/sample2.musicxml").unwrap();
    assert_eq!(format, FileFormat::Musicxml);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_musicxml_zipped() {
    let format = FileFormat::from_file("fixtures/application/sample.mxl").unwrap();
    assert_eq!(format, FileFormat::MusicxmlZipped);
}

#[test]
fn test_ogg_multiplexed_media() {
    let format = FileFormat::from_file("fixtures/application/sample.ogx").unwrap();
    assert_eq!(format, FileFormat::OggMultiplexedMedia);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_database() {
    let format = FileFormat::from_file("fixtures/application/sample.odb").unwrap();
    assert_eq!(format, FileFormat::OpendocumentDatabase);
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
fn test_pem_public_key() {
    let format = FileFormat::from_file("fixtures/application/sample.pub").unwrap();
    assert_eq!(format, FileFormat::PemPublicKey);
}

#[test]
fn test_personal_storage_table() {
    let format = FileFormat::from_file("fixtures/application/sample.pst").unwrap();
    assert_eq!(format, FileFormat::PersonalStorageTable);
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
fn test_postscript() {
    let format = FileFormat::from_file("fixtures/application/sample.ps").unwrap();
    assert_eq!(format, FileFormat::Postscript);
}

#[test]
fn test_really_simple_syndication_1() {
    let format = FileFormat::from_file("fixtures/application/sample1.rss").unwrap();
    assert_eq!(format, FileFormat::ReallySimpleSyndication);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_really_simple_syndication_2() {
    let format = FileFormat::from_file("fixtures/application/sample2.rss").unwrap();
    assert_eq!(format, FileFormat::ReallySimpleSyndication);
}

#[test]
fn test_simple_object_access_protocol_1() {
    let format = FileFormat::from_file("fixtures/application/sample1.soap").unwrap();
    assert_eq!(format, FileFormat::SimpleObjectAccessProtocol);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_simple_object_access_protocol_2() {
    let format = FileFormat::from_file("fixtures/application/sample2.soap").unwrap();
    assert_eq!(format, FileFormat::SimpleObjectAccessProtocol);
}

#[test]
fn test_small_web_format() {
    let format = FileFormat::from_file("fixtures/application/sample.swf").unwrap();
    assert_eq!(format, FileFormat::SmallWebFormat);
}

#[test]
fn test_sqlite3() {
    let format = FileFormat::from_file("fixtures/application/sample.sqlite").unwrap();
    assert_eq!(format, FileFormat::Sqlite3);
}

#[test]
fn test_subrip_text() {
    let format = FileFormat::from_file("fixtures/application/sample.srt").unwrap();
    assert_eq!(format, FileFormat::SubripText);
}

#[test]
fn test_tasty() {
    let format = FileFormat::from_file("fixtures/application/sample.tasty").unwrap();
    assert_eq!(format, FileFormat::Tasty);
}

#[test]
fn test_windows_shortcut() {
    let format = FileFormat::from_file("fixtures/application/sample.lnk").unwrap();
    assert_eq!(format, FileFormat::WindowsShortcut);
}

#[test]
fn test_xml_localization_interchange_file_format_1() {
    let format = FileFormat::from_file("fixtures/application/sample1.xlf").unwrap();
    assert_eq!(format, FileFormat::XmlLocalizationInterchangeFileFormat);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_xml_localization_interchange_file_format_2() {
    let format = FileFormat::from_file("fixtures/application/sample2.xlf").unwrap();
    assert_eq!(format, FileFormat::XmlLocalizationInterchangeFileFormat);
}
