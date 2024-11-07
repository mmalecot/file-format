use file_format::FileFormat;

#[test]
#[cfg(feature = "from-extension")]
fn test_abiword(){
    let fmt = FileFormat::from_extension("abw");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Abiword)), "{:?} does not contain {}", fmt, FileFormat::Abiword);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_abiword_template(){
    let fmt = FileFormat::from_extension("awt");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AbiwordTemplate)), "{:?} does not contain {}", fmt, FileFormat::AbiwordTemplate);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_adobe_indesign_document() {
    let fmt = FileFormat::from_extension("indd");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AdobeIndesignDocument)), "{:?} does not contain {}", fmt, FileFormat::AdobeIndesignDocument);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_djvu(){
    let fmt = FileFormat::from_extension("djvu");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Djvu)), "{:?} does not contain {}", fmt, FileFormat::Djvu);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_indesign_markup_language() {
    let fmt = FileFormat::from_extension("idml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::IndesignMarkupLanguage)), "{:?} does not contain {}", fmt, FileFormat::IndesignMarkupLanguage);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_latex(){
    let fmt = FileFormat::from_extension("tex");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Latex)), "{:?} does not contain {}", fmt, FileFormat::Latex);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_microsoft_publisher_document() {
    let fmt = FileFormat::from_extension("pub");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftPublisherDocument)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftPublisherDocument);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_microsoft_word_document() {
    let fmt = FileFormat::from_extension("doc");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftWordDocument)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftWordDocument);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_microsoft_works_word_processor(){
    let fmt = FileFormat::from_extension("wps");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftWorksWordProcessor)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftWorksWordProcessor);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_microsoft_write() {
    let fmt = FileFormat::from_extension("wri");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftWrite)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftWrite);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_office_open_xml_document() {
    let fmt = FileFormat::from_extension("docx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OfficeOpenXmlDocument)), "{:?} does not contain {}", fmt, FileFormat::OfficeOpenXmlDocument);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_opendocument_text() {
    let fmt = FileFormat::from_extension("odt");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OpendocumentText)), "{:?} does not contain {}", fmt, FileFormat::OpendocumentText);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_opendocument_text_master() {
    let fmt = FileFormat::from_extension("odm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OpendocumentTextMaster)), "{:?} does not contain {}", fmt, FileFormat::OpendocumentTextMaster);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_opendocument_text_master_template() {
    let fmt = FileFormat::from_extension("otm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OpendocumentTextMasterTemplate)), "{:?} does not contain {}", fmt, FileFormat::OpendocumentTextMasterTemplate);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_opendocument_text_template() {
    let fmt = FileFormat::from_extension("ott");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OpendocumentTextTemplate)), "{:?} does not contain {}", fmt, FileFormat::OpendocumentTextTemplate);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_openxps(){
    let fmt = FileFormat::from_extension("xps");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Openxps)), "{:?} does not contain {}", fmt, FileFormat::Openxps);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_portable_document_format() {
    let fmt = FileFormat::from_extension("pdf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PortableDocumentFormat)), "{:?} does not contain {}", fmt, FileFormat::PortableDocumentFormat);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_postscript(){
    let fmt = FileFormat::from_extension("ps");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Postscript)), "{:?} does not contain {}", fmt, FileFormat::Postscript);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_rich_text_format() {
    let fmt = FileFormat::from_extension("rtf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::RichTextFormat)), "{:?} does not contain {}", fmt, FileFormat::RichTextFormat);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_starwriter(){
    let fmt = FileFormat::from_extension("sdw");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Starwriter)), "{:?} does not contain {}", fmt, FileFormat::Starwriter);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_sun_xml_writer() {
    let fmt = FileFormat::from_extension("sxw");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SunXmlWriter)), "{:?} does not contain {}", fmt, FileFormat::SunXmlWriter);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_sun_xml_writer_global() {
    let fmt = FileFormat::from_extension("sgw");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SunXmlWriterGlobal)), "{:?} does not contain {}", fmt, FileFormat::SunXmlWriterGlobal);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_sun_xml_writer_template() {
    let fmt = FileFormat::from_extension("stw");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SunXmlWriterTemplate)), "{:?} does not contain {}", fmt, FileFormat::SunXmlWriterTemplate);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_uniform_office_format_text() {
    let fmt = FileFormat::from_extension("uot");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::UniformOfficeFormatText)), "{:?} does not contain {}", fmt, FileFormat::UniformOfficeFormatText);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_wordperfect_document(){
    let fmt = FileFormat::from_extension("wpd");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WordperfectDocument)), "{:?} does not contain {}", fmt, FileFormat::WordperfectDocument);
}

