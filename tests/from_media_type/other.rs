use file_format::FileFormat;

#[test]
#[cfg(feature = "from-media-type")]
fn test_activemime(){
    let fmt = FileFormat::from_media_type("application/x-mso");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Activemime)), "{:?} does not contain {}", fmt, FileFormat::Activemime);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_advanced_systems_format() {
    let fmt = FileFormat::from_media_type("application/vnd.ms-asf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AdvancedSystemsFormat)), "{:?} does not contain {}", fmt, FileFormat::AdvancedSystemsFormat);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_age_encryption() {
    let fmt = FileFormat::from_media_type("application/x-age-encryption");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AgeEncryption)), "{:?} does not contain {}", fmt, FileFormat::AgeEncryption);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_android_resource_storage_container() {
    let fmt = FileFormat::from_media_type("application/vnd.android.arsc");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AndroidResourceStorageContainer)), "{:?} does not contain {}", fmt, FileFormat::AndroidResourceStorageContainer);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_apache_arrow_columnar() {
    let fmt = FileFormat::from_media_type("application/vnd.apache.arrow.file");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ApacheArrowColumnar)), "{:?} does not contain {}", fmt, FileFormat::ApacheArrowColumnar);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_apache_avro() {
    let fmt = FileFormat::from_media_type("application/vnd.apache.avro");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ApacheAvro)), "{:?} does not contain {}", fmt, FileFormat::ApacheAvro);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_apache_parquet() {
    let fmt = FileFormat::from_media_type("application/vnd.apache.parquet");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ApacheParquet)), "{:?} does not contain {}", fmt, FileFormat::ApacheParquet);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_arbitrary_binary_data() {
    let fmt = FileFormat::from_media_type("application/octet-stream");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ArbitraryBinaryData)), "{:?} does not contain {}", fmt, FileFormat::ArbitraryBinaryData);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_atom(){
    let fmt = FileFormat::from_media_type("application/atom+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Atom)), "{:?} does not contain {}", fmt, FileFormat::Atom);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_clojure_script() {
    let fmt = FileFormat::from_media_type("text/x-clojure");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ClojureScript)), "{:?} does not contain {}", fmt, FileFormat::ClojureScript);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_compound_file_binary() {
    let fmt = FileFormat::from_media_type("application/x-cfb");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::CompoundFileBinary)), "{:?} does not contain {}", fmt, FileFormat::CompoundFileBinary);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_der_certificate() {
    let fmt = FileFormat::from_media_type("application/x-x509-ca-cert");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::DerCertificate)), "{:?} does not contain {}", fmt, FileFormat::DerCertificate);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_digital_imaging_and_communications_in_medicine() {
    let fmt = FileFormat::from_media_type("application/dicom");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::DigitalImagingAndCommunicationsInMedicine)), "{:?} does not contain {}", fmt, FileFormat::DigitalImagingAndCommunicationsInMedicine);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_empty(){
    let fmt = FileFormat::from_media_type("application/x-empty");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Empty)), "{:?} does not contain {}", fmt, FileFormat::Empty);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_extensible_binary_meta_language() {
    let fmt = FileFormat::from_media_type("application/x-ebml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ExtensibleBinaryMetaLanguage)), "{:?} does not contain {}", fmt, FileFormat::ExtensibleBinaryMetaLanguage);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_extensible_markup_language() {
    let fmt = FileFormat::from_media_type("text/xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ExtensibleMarkupLanguage)), "{:?} does not contain {}", fmt, FileFormat::ExtensibleMarkupLanguage);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_extensible_stylesheet_language_transformations(){
    let fmt = FileFormat::from_media_type("application/xslt+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ExtensibleStylesheetLanguageTransformations)), "{:?} does not contain {}", fmt, FileFormat::ExtensibleStylesheetLanguageTransformations);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_flash_cs5_project() {
    let fmt = FileFormat::from_media_type("application/vnd.adobe.fla");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::FlashCs5Project)), "{:?} does not contain {}", fmt, FileFormat::FlashCs5Project);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_flash_project() {
    let fmt = FileFormat::from_media_type("application/vnd.adobe.fla");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::FlashProject)), "{:?} does not contain {}", fmt, FileFormat::FlashProject);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_flexible_image_transport_system() {
    let fmt = FileFormat::from_media_type("application/fits");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::FlexibleImageTransportSystem)), "{:?} does not contain {}", fmt, FileFormat::FlexibleImageTransportSystem);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_gettext_machine_object() {
    let fmt = FileFormat::from_media_type("application/x-gettext-translation");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::GettextMachineObject)), "{:?} does not contain {}", fmt, FileFormat::GettextMachineObject);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_hypertext_markup_language() {
    let fmt = FileFormat::from_media_type("text/html");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::HypertextMarkupLanguage)), "{:?} does not contain {}", fmt, FileFormat::HypertextMarkupLanguage);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_icalendar(){
    let fmt = FileFormat::from_media_type("text/calendar");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Icalendar)), "{:?} does not contain {}", fmt, FileFormat::Icalendar);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_icc_profile() {
    let fmt = FileFormat::from_media_type("application/vnd.iccprofile");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::IccProfile)), "{:?} does not contain {}", fmt, FileFormat::IccProfile);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_java_keystore() {
    let fmt = FileFormat::from_media_type("application/x-java-keystore");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::JavaKeystore)), "{:?} does not contain {}", fmt, FileFormat::JavaKeystore);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_json_feed() {
    let fmt = FileFormat::from_media_type("application/feed+json");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::JsonFeed)), "{:?} does not contain {}", fmt, FileFormat::JsonFeed);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_lua_script() {
    let fmt = FileFormat::from_media_type("text/x-lua");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::LuaScript)), "{:?} does not contain {}", fmt, FileFormat::LuaScript);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_microsoft_compiled_html_help() {
    let fmt = FileFormat::from_media_type("application/vnd.ms-htmlhelp");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftCompiledHtmlHelp)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftCompiledHtmlHelp);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_microsoft_project_plan() {
    let fmt = FileFormat::from_media_type("application/vnd.ms-project");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftProjectPlan)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftProjectPlan);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_microsoft_visual_studio_solution() {
    let fmt = FileFormat::from_media_type("application/vnd.ms-developer");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftVisualStudioSolution)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftVisualStudioSolution);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_mpeg4_part14() {
    let fmt = FileFormat::from_media_type("application/mp4");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Mpeg4Part14)), "{:?} does not contain {}", fmt, FileFormat::Mpeg4Part14);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_ms_dos_batch() {
    let fmt = FileFormat::from_media_type("text/x-msdos-batch");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MsDosBatch)), "{:?} does not contain {}", fmt, FileFormat::MsDosBatch);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_musicxml(){
    let fmt = FileFormat::from_media_type("application/vnd.recordare.musicxml+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Musicxml)), "{:?} does not contain {}", fmt, FileFormat::Musicxml);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_musicxml_zip() {
    let fmt = FileFormat::from_media_type("application/vnd.recordare.musicxml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MusicxmlZip)), "{:?} does not contain {}", fmt, FileFormat::MusicxmlZip);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_ogg_multiplexed_media() {
    let fmt = FileFormat::from_media_type("application/ogg");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OggMultiplexedMedia)), "{:?} does not contain {}", fmt, FileFormat::OggMultiplexedMedia);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_pcap_dump() {
    let fmt = FileFormat::from_media_type("application/vnd.tcpdump.pcap");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PcapDump)), "{:?} does not contain {}", fmt, FileFormat::PcapDump);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_pcap_next_generation_dump() {
    let fmt = FileFormat::from_media_type("application/x-pcapng");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PcapNextGenerationDump)), "{:?} does not contain {}", fmt, FileFormat::PcapNextGenerationDump);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_pem_certificate() {
    let fmt = FileFormat::from_media_type("application/x-pem-file");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PemCertificate)), "{:?} does not contain {}", fmt, FileFormat::PemCertificate);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_pem_certificate_signing_request() {
    let fmt = FileFormat::from_media_type("application/x-pem-file");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PemCertificateSigningRequest)), "{:?} does not contain {}", fmt, FileFormat::PemCertificateSigningRequest);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_pem_private_key() {
    let fmt = FileFormat::from_media_type("application/x-pem-file");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PemPrivateKey)), "{:?} does not contain {}", fmt, FileFormat::PemPrivateKey);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_pem_public_key() {
    let fmt = FileFormat::from_media_type("application/x-pem-file");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PemPublicKey)), "{:?} does not contain {}", fmt, FileFormat::PemPublicKey);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_perl_script() {
    let fmt = FileFormat::from_media_type("text/x-perl");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PerlScript)), "{:?} does not contain {}", fmt, FileFormat::PerlScript);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_personal_storage_table() {
    let fmt = FileFormat::from_media_type("application/vnd.ms-outlook");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PersonalStorageTable)), "{:?} does not contain {}", fmt, FileFormat::PersonalStorageTable);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_pgp_message() {
    let fmt = FileFormat::from_media_type("application/pgp");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PgpMessage)), "{:?} does not contain {}", fmt, FileFormat::PgpMessage);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_pgp_private_key_block() {
    let fmt = FileFormat::from_media_type("application/pgp-keys");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PgpPrivateKeyBlock)), "{:?} does not contain {}", fmt, FileFormat::PgpPrivateKeyBlock);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_pgp_public_key_block() {
    let fmt = FileFormat::from_media_type("application/pgp-keys");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PgpPublicKeyBlock)), "{:?} does not contain {}", fmt, FileFormat::PgpPublicKeyBlock);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_pgp_signature() {
    let fmt = FileFormat::from_media_type("application/pgp-signature");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PgpSignature)), "{:?} does not contain {}", fmt, FileFormat::PgpSignature);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_pgp_signed_message() {
    let fmt = FileFormat::from_media_type("application/pgp");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PgpSignedMessage)), "{:?} does not contain {}", fmt, FileFormat::PgpSignedMessage);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_plain_text() {
    let fmt = FileFormat::from_media_type("text/plain");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PlainText)), "{:?} does not contain {}", fmt, FileFormat::PlainText);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_python_script() {
    let fmt = FileFormat::from_media_type("text/x-script.python");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PythonScript)), "{:?} does not contain {}", fmt, FileFormat::PythonScript);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_really_simple_syndication(){
    let fmt = FileFormat::from_media_type("application/rss+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ReallySimpleSyndication)), "{:?} does not contain {}", fmt, FileFormat::ReallySimpleSyndication);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_realmedia(){
    let fmt = FileFormat::from_media_type("application/vnd.rn-realmedia");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Realmedia)), "{:?} does not contain {}", fmt, FileFormat::Realmedia);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_ruby_script() {
    let fmt = FileFormat::from_media_type("text/x-ruby");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::RubyScript)), "{:?} does not contain {}", fmt, FileFormat::RubyScript);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_shell_script() {
    let fmt = FileFormat::from_media_type("text/x-shellscript");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ShellScript)), "{:?} does not contain {}", fmt, FileFormat::ShellScript);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_simple_object_access_protocol(){
    let fmt = FileFormat::from_media_type("application/soap+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SimpleObjectAccessProtocol)), "{:?} does not contain {}", fmt, FileFormat::SimpleObjectAccessProtocol);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_small_web_format() {
    let fmt = FileFormat::from_media_type("application/x-shockwave-flash");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SmallWebFormat)), "{:?} does not contain {}", fmt, FileFormat::SmallWebFormat);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_tiled_map_xml(){
    let fmt = FileFormat::from_media_type("application/x-tmx+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::TiledMapXml)), "{:?} does not contain {}", fmt, FileFormat::TiledMapXml);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_tiled_tileset_xml(){
    let fmt = FileFormat::from_media_type("application/x-tsx+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::TiledTilesetXml)), "{:?} does not contain {}", fmt, FileFormat::TiledTilesetXml);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_tool_command_language_script() {
    let fmt = FileFormat::from_media_type("text/x-tcl");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ToolCommandLanguageScript)), "{:?} does not contain {}", fmt, FileFormat::ToolCommandLanguageScript);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_vcalendar(){
    let fmt = FileFormat::from_media_type("text/calendar");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Vcalendar)), "{:?} does not contain {}", fmt, FileFormat::Vcalendar);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_vcard(){
    let fmt = FileFormat::from_media_type("text/vcard");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Vcard)), "{:?} does not contain {}", fmt, FileFormat::Vcard);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_webassembly_text() {
    let fmt = FileFormat::from_media_type("text/wasm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WebassemblyText)), "{:?} does not contain {}", fmt, FileFormat::WebassemblyText);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_wordperfect_macro() {
    let fmt = FileFormat::from_media_type("application/vnd.wordperfect");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WordperfectMacro)), "{:?} does not contain {}", fmt, FileFormat::WordperfectMacro);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_xml_localization_interchange_file_format(){
    let fmt = FileFormat::from_media_type("application/xliff+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::XmlLocalizationInterchangeFileFormat)), "{:?} does not contain {}", fmt, FileFormat::XmlLocalizationInterchangeFileFormat);
}

