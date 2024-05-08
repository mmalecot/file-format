use file_format::FileFormat;

#[test]
fn test_activemime() {
    let fmt = FileFormat::from_file("fixtures/other/sample.mso").unwrap();
    assert_eq!(fmt, FileFormat::Activemime);
}

#[test]
fn test_advanced_systems_format() {
    let fmt = FileFormat::from_file("fixtures/other/sample.asf").unwrap();
    assert_eq!(fmt, FileFormat::AdvancedSystemsFormat);
}

#[test]
fn test_age_encryption() {
    let fmt = FileFormat::from_file("fixtures/other/sample.age").unwrap();
    assert_eq!(fmt, FileFormat::AgeEncryption);
}

#[test]
fn test_android_resource_storage_container() {
    let fmt = FileFormat::from_file("fixtures/other/sample.arsc").unwrap();
    assert_eq!(fmt, FileFormat::AndroidResourceStorageContainer);
}

#[test]
fn test_apache_arrow_columnar() {
    let fmt = FileFormat::from_file("fixtures/other/sample.arrow").unwrap();
    assert_eq!(fmt, FileFormat::ApacheArrowColumnar);
}

#[test]
fn test_apache_avro() {
    let fmt = FileFormat::from_file("fixtures/other/sample.avro").unwrap();
    assert_eq!(fmt, FileFormat::ApacheAvro);
}

#[test]
fn test_apache_parquet() {
    let fmt = FileFormat::from_file("fixtures/other/sample.parquet").unwrap();
    assert_eq!(fmt, FileFormat::ApacheParquet);
}

#[test]
fn test_arbitrary_binary_data() {
    let fmt = FileFormat::from_file("fixtures/other/sample.bin").unwrap();
    assert_eq!(fmt, FileFormat::ArbitraryBinaryData);
}

#[test]
fn test_atom_1() {
    let fmt = FileFormat::from_file("fixtures/other/sample1.atom").unwrap();
    assert_eq!(fmt, FileFormat::Atom);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_atom_2() {
    let fmt = FileFormat::from_file("fixtures/other/sample2.atom").unwrap();
    assert_eq!(fmt, FileFormat::Atom);
}

#[test]
fn test_clojure_script() {
    let fmt = FileFormat::from_file("fixtures/other/sample.clj").unwrap();
    assert_eq!(fmt, FileFormat::ClojureScript);
}

#[test]
fn test_compound_file_binary() {
    let fmt = FileFormat::from_file("fixtures/other/sample.cfb").unwrap();
    assert_eq!(fmt, FileFormat::CompoundFileBinary);
}

#[test]
fn test_der_certificate() {
    let fmt = FileFormat::from_file("fixtures/other/sample.der").unwrap();
    assert_eq!(fmt, FileFormat::DerCertificate);
}

#[test]
fn test_digital_imaging_and_communications_in_medicine() {
    let fmt = FileFormat::from_file("fixtures/other/sample.dcm").unwrap();
    assert_eq!(fmt, FileFormat::DigitalImagingAndCommunicationsInMedicine);
}

#[test]
fn test_empty() {
    let fmt = FileFormat::from_file("fixtures/other/sample.empty").unwrap();
    assert_eq!(fmt, FileFormat::Empty);
}

#[test]
fn test_extensible_binary_meta_language() {
    let fmt = FileFormat::from_file("fixtures/other/sample.ebml").unwrap();
    assert_eq!(fmt, FileFormat::ExtensibleBinaryMetaLanguage);
}

#[test]
fn test_extensible_markup_language() {
    let fmt = FileFormat::from_file("fixtures/other/sample.xml").unwrap();
    assert_eq!(fmt, FileFormat::ExtensibleMarkupLanguage);
}

#[test]
fn test_extensible_stylesheet_language_transformations_1() {
    let fmt = FileFormat::from_file("fixtures/other/sample1.xsl").unwrap();
    assert_eq!(fmt, FileFormat::ExtensibleStylesheetLanguageTransformations);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_extensible_stylesheet_language_transformations_2() {
    let fmt = FileFormat::from_file("fixtures/other/sample2.xsl").unwrap();
    assert_eq!(fmt, FileFormat::ExtensibleStylesheetLanguageTransformations);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_flash_cs5_project() {
    let fmt = FileFormat::from_file("fixtures/other/sample1.fla").unwrap();
    assert_eq!(fmt, FileFormat::FlashCs5Project);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_flash_project() {
    let fmt = FileFormat::from_file("fixtures/other/sample2.fla").unwrap();
    assert_eq!(fmt, FileFormat::FlashProject);
}

#[test]
fn test_flexible_image_transport_system() {
    let fmt = FileFormat::from_file("fixtures/other/sample.fits").unwrap();
    assert_eq!(fmt, FileFormat::FlexibleImageTransportSystem);
}

#[test]
fn test_gettext_machine_object() {
    let fmt = FileFormat::from_file("fixtures/other/sample.mo").unwrap();
    assert_eq!(fmt, FileFormat::GettextMachineObject);
}

#[test]
fn test_hypertext_markup_language() {
    let fmt = FileFormat::from_file("fixtures/other/sample.html").unwrap();
    assert_eq!(fmt, FileFormat::HypertextMarkupLanguage);
}

#[test]
fn test_icalendar() {
    let fmt = FileFormat::from_file("fixtures/other/sample.ics").unwrap();
    assert_eq!(fmt, FileFormat::Icalendar);
}

#[test]
fn test_icc_profile() {
    let fmt = FileFormat::from_file("fixtures/other/sample.icc").unwrap();
    assert_eq!(fmt, FileFormat::IccProfile);
}

#[test]
fn test_java_keystore() {
    let fmt = FileFormat::from_file("fixtures/other/sample.jks").unwrap();
    assert_eq!(fmt, FileFormat::JavaKeystore);
}

#[test]
fn test_json_feed() {
    let fmt = FileFormat::from_file("fixtures/other/sample.json").unwrap();
    assert_eq!(fmt, FileFormat::JsonFeed);
}

#[test]
fn test_lua_script() {
    let fmt = FileFormat::from_file("fixtures/other/sample.lua").unwrap();
    assert_eq!(fmt, FileFormat::LuaScript);
}

#[test]
fn test_microsoft_compiled_html_help() {
    let fmt = FileFormat::from_file("fixtures/other/sample.chm").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftCompiledHtmlHelp);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_project_plan() {
    let fmt = FileFormat::from_file("fixtures/other/sample.mpp").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftProjectPlan);
}

#[test]
fn test_microsoft_visual_studio_solution() {
    let fmt = FileFormat::from_file("fixtures/other/sample.sln").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftVisualStudioSolution);
}

#[test]
fn test_mpeg4_part14() {
    let fmt = FileFormat::from_file("fixtures/other/sample.mp4").unwrap();
    assert_eq!(fmt, FileFormat::Mpeg4Part14);
}

#[test]
fn test_ms_dos_batch() {
    let fmt = FileFormat::from_file("fixtures/other/sample.bat").unwrap();
    assert_eq!(fmt, FileFormat::MsDosBatch);
}

#[test]
fn test_musicxml_1() {
    let fmt = FileFormat::from_file("fixtures/other/sample1.musicxml").unwrap();
    assert_eq!(fmt, FileFormat::Musicxml);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_musicxml_2() {
    let fmt = FileFormat::from_file("fixtures/other/sample2.musicxml").unwrap();
    assert_eq!(fmt, FileFormat::Musicxml);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_musicxml_zip() {
    let fmt = FileFormat::from_file("fixtures/other/sample.mxl").unwrap();
    assert_eq!(fmt, FileFormat::MusicxmlZip);
}

#[test]
fn test_ogg_multiplexed_media() {
    let fmt = FileFormat::from_file("fixtures/other/sample.ogx").unwrap();
    assert_eq!(fmt, FileFormat::OggMultiplexedMedia);
}

#[test]
fn test_pcap_dump() {
    let fmt = FileFormat::from_file("fixtures/other/sample.pcap").unwrap();
    assert_eq!(fmt, FileFormat::PcapDump);
}

#[test]
fn test_pcap_next_generation_dump() {
    let fmt = FileFormat::from_file("fixtures/other/sample.pcapng").unwrap();
    assert_eq!(fmt, FileFormat::PcapNextGenerationDump);
}

#[test]
fn test_pem_certificate() {
    let fmt = FileFormat::from_file("fixtures/other/sample.crt").unwrap();
    assert_eq!(fmt, FileFormat::PemCertificate);
}

#[test]
fn test_pem_certificate_signing_request() {
    let fmt = FileFormat::from_file("fixtures/other/sample.csr").unwrap();
    assert_eq!(fmt, FileFormat::PemCertificateSigningRequest);
}

#[test]
fn test_pem_private_key() {
    let fmt = FileFormat::from_file("fixtures/other/sample.key").unwrap();
    assert_eq!(fmt, FileFormat::PemPrivateKey);
}

#[test]
fn test_pem_public_key() {
    let fmt = FileFormat::from_file("fixtures/other/sample.pub").unwrap();
    assert_eq!(fmt, FileFormat::PemPublicKey);
}

#[test]
fn test_perl_script() {
    let fmt = FileFormat::from_file("fixtures/other/sample.pl").unwrap();
    assert_eq!(fmt, FileFormat::PerlScript);
}

#[test]
fn test_personal_storage_table() {
    let fmt = FileFormat::from_file("fixtures/other/sample.pst").unwrap();
    assert_eq!(fmt, FileFormat::PersonalStorageTable);
}

#[test]
fn test_pgp_message() {
    let fmt = FileFormat::from_file("fixtures/other/sample1.asc").unwrap();
    assert_eq!(fmt, FileFormat::PgpMessage);
}

#[test]
fn test_pgp_private_key_block() {
    let fmt = FileFormat::from_file("fixtures/other/sample2.asc").unwrap();
    assert_eq!(fmt, FileFormat::PgpPrivateKeyBlock);
}

#[test]
fn test_pgp_public_key_block() {
    let fmt = FileFormat::from_file("fixtures/other/sample3.asc").unwrap();
    assert_eq!(fmt, FileFormat::PgpPublicKeyBlock);
}

#[test]
fn test_pgp_signature() {
    let fmt = FileFormat::from_file("fixtures/other/sample4.asc").unwrap();
    assert_eq!(fmt, FileFormat::PgpSignature);
}

#[test]
fn test_pgp_signed_message() {
    let fmt = FileFormat::from_file("fixtures/other/sample5.asc").unwrap();
    assert_eq!(fmt, FileFormat::PgpSignedMessage);
}

#[cfg(feature = "reader-txt")]
#[test]
fn test_plain_text() {
    let fmt = FileFormat::from_file("fixtures/other/sample.txt").unwrap();
    assert_eq!(fmt, FileFormat::PlainText);
}

#[test]
fn test_python_script() {
    let fmt = FileFormat::from_file("fixtures/other/sample.py").unwrap();
    assert_eq!(fmt, FileFormat::PythonScript);
}

#[test]
fn test_really_simple_syndication_1() {
    let fmt = FileFormat::from_file("fixtures/other/sample1.rss").unwrap();
    assert_eq!(fmt, FileFormat::ReallySimpleSyndication);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_really_simple_syndication_2() {
    let fmt = FileFormat::from_file("fixtures/other/sample2.rss").unwrap();
    assert_eq!(fmt, FileFormat::ReallySimpleSyndication);
}

#[test]
fn test_realmedia() {
    let fmt = FileFormat::from_file("fixtures/other/sample.rm").unwrap();
    assert_eq!(fmt, FileFormat::Realmedia);
}

#[test]
fn test_ruby_script() {
    let fmt = FileFormat::from_file("fixtures/other/sample.rb").unwrap();
    assert_eq!(fmt, FileFormat::RubyScript);
}

#[test]
fn test_shell_script() {
    let fmt = FileFormat::from_file("fixtures/other/sample.sh").unwrap();
    assert_eq!(fmt, FileFormat::ShellScript);
}

#[test]
fn test_simple_object_access_protocol_1() {
    let fmt = FileFormat::from_file("fixtures/other/sample1.soap").unwrap();
    assert_eq!(fmt, FileFormat::SimpleObjectAccessProtocol);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_simple_object_access_protocol_2() {
    let fmt = FileFormat::from_file("fixtures/other/sample2.soap").unwrap();
    assert_eq!(fmt, FileFormat::SimpleObjectAccessProtocol);
}

#[test]
fn test_small_web_format() {
    let fmt = FileFormat::from_file("fixtures/other/sample.swf").unwrap();
    assert_eq!(fmt, FileFormat::SmallWebFormat);
}

#[test]
fn test_tiled_map_xml_1() {
    let fmt = FileFormat::from_file("fixtures/other/sample1.tmx").unwrap();
    assert_eq!(fmt, FileFormat::TiledMapXml);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_tiled_map_xml_2() {
    let fmt = FileFormat::from_file("fixtures/other/sample2.tmx").unwrap();
    assert_eq!(fmt, FileFormat::TiledMapXml);
}

#[test]
fn test_tiled_tileset_xml_1() {
    let fmt = FileFormat::from_file("fixtures/other/sample1.tsx").unwrap();
    assert_eq!(fmt, FileFormat::TiledTilesetXml);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_tiled_tileset_xml_2() {
    let fmt = FileFormat::from_file("fixtures/other/sample2.tsx").unwrap();
    assert_eq!(fmt, FileFormat::TiledTilesetXml);
}

#[test]
fn test_tool_command_language_script() {
    let fmt = FileFormat::from_file("fixtures/other/sample.tcl").unwrap();
    assert_eq!(fmt, FileFormat::ToolCommandLanguageScript);
}

#[test]
fn test_vcalendar() {
    let fmt = FileFormat::from_file("fixtures/other/sample.vcs").unwrap();
    assert_eq!(fmt, FileFormat::Vcalendar);
}

#[test]
fn test_vcard() {
    let fmt = FileFormat::from_file("fixtures/other/sample.vcf").unwrap();
    assert_eq!(fmt, FileFormat::Vcard);
}

#[test]
fn test_webassembly_text() {
    let fmt = FileFormat::from_file("fixtures/other/sample.wat").unwrap();
    assert_eq!(fmt, FileFormat::WebassemblyText);
}

#[test]
fn test_wordperfect_macro() {
    let fmt = FileFormat::from_file("fixtures/other/sample.wpm").unwrap();
    assert_eq!(fmt, FileFormat::WordperfectMacro);
}

#[test]
fn test_xml_localization_interchange_file_format_1() {
    let fmt = FileFormat::from_file("fixtures/other/sample1.xlf").unwrap();
    assert_eq!(fmt, FileFormat::XmlLocalizationInterchangeFileFormat);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_xml_localization_interchange_file_format_2() {
    let fmt = FileFormat::from_file("fixtures/other/sample2.xlf").unwrap();
    assert_eq!(fmt, FileFormat::XmlLocalizationInterchangeFileFormat);
}
