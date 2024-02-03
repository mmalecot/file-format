use file_format::FileFormat;

#[test]
fn test_abiword_1() {
    let fmt = FileFormat::from_file("fixtures/document/sample1.abw").unwrap();
    assert_eq!(fmt, FileFormat::Abiword);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_abiword_2() {
    let fmt = FileFormat::from_file("fixtures/document/sample2.abw").unwrap();
    assert_eq!(fmt, FileFormat::Abiword);
}

#[test]
fn test_abiword_template_1() {
    let fmt = FileFormat::from_file("fixtures/document/sample1.awt").unwrap();
    assert_eq!(fmt, FileFormat::AbiwordTemplate);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_abiword_template_2() {
    let fmt = FileFormat::from_file("fixtures/document/sample2.awt").unwrap();
    assert_eq!(fmt, FileFormat::AbiwordTemplate);
}

#[test]
fn test_adobe_indesign_document() {
    let fmt = FileFormat::from_file("fixtures/document/sample.indd").unwrap();
    assert_eq!(fmt, FileFormat::AdobeIndesignDocument);
}

#[test]
fn test_djvu() {
    let fmt = FileFormat::from_file("fixtures/document/sample.djvu").unwrap();
    assert_eq!(fmt, FileFormat::Djvu);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_indesign_markup_language() {
    let fmt = FileFormat::from_file("fixtures/document/sample.idml").unwrap();
    assert_eq!(fmt, FileFormat::IndesignMarkupLanguage);
}

#[test]
fn test_latex() {
    let fmt = FileFormat::from_file("fixtures/document/sample.tex").unwrap();
    assert_eq!(fmt, FileFormat::Latex);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_publisher_document() {
    let fmt = FileFormat::from_file("fixtures/document/sample.pub").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftPublisherDocument);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_word_document() {
    let fmt = FileFormat::from_file("fixtures/document/sample.doc").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftWordDocument);
}

#[test]
fn test_microsoft_works_word_processor_1() {
    let fmt = FileFormat::from_file("fixtures/document/sample1.wps").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftWorksWordProcessor);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_works_word_processor_2() {
    let fmt = FileFormat::from_file("fixtures/document/sample2.wps").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftWorksWordProcessor);
}

#[test]
fn test_microsoft_write() {
    let fmt = FileFormat::from_file("fixtures/document/sample.wri").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftWrite);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_office_open_xml_document() {
    let fmt = FileFormat::from_file("fixtures/document/sample.docx").unwrap();
    assert_eq!(fmt, FileFormat::OfficeOpenXmlDocument);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_text() {
    let fmt = FileFormat::from_file("fixtures/document/sample.odt").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentText);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_text_master() {
    let fmt = FileFormat::from_file("fixtures/document/sample.odm").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentTextMaster);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_text_master_template() {
    let fmt = FileFormat::from_file("fixtures/document/sample.otm").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentTextMasterTemplate);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_text_template() {
    let fmt = FileFormat::from_file("fixtures/document/sample.ott").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentTextTemplate);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_openxps() {
    let fmt = FileFormat::from_file("fixtures/document/sample.xps").unwrap();
    assert_eq!(fmt, FileFormat::Openxps);
}

#[test]
fn test_portable_document_format() {
    let fmt = FileFormat::from_file("fixtures/document/sample.pdf").unwrap();
    assert_eq!(fmt, FileFormat::PortableDocumentFormat);
}

#[test]
fn test_postscript() {
    let fmt = FileFormat::from_file("fixtures/document/sample.ps").unwrap();
    assert_eq!(fmt, FileFormat::Postscript);
}

#[test]
fn test_rich_text_format() {
    let fmt = FileFormat::from_file("fixtures/document/sample.rtf").unwrap();
    assert_eq!(fmt, FileFormat::RichTextFormat);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_starwriter() {
    let fmt = FileFormat::from_file("fixtures/document/sample.sdw").unwrap();
    assert_eq!(fmt, FileFormat::Starwriter);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_sun_xml_writer() {
    let fmt = FileFormat::from_file("fixtures/document/sample.sxw").unwrap();
    assert_eq!(fmt, FileFormat::SunXmlWriter);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_sun_xml_writer_global() {
    let fmt = FileFormat::from_file("fixtures/document/sample.sgw").unwrap();
    assert_eq!(fmt, FileFormat::SunXmlWriterGlobal);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_sun_xml_writer_template() {
    let fmt = FileFormat::from_file("fixtures/document/sample.stw").unwrap();
    assert_eq!(fmt, FileFormat::SunXmlWriterTemplate);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_uniform_office_format_text() {
    let fmt = FileFormat::from_file("fixtures/document/sample.uot").unwrap();
    assert_eq!(fmt, FileFormat::UniformOfficeFormatText);
}

#[test]
fn test_wordperfect_document_1() {
    let fmt = FileFormat::from_file("fixtures/document/sample1.wpd").unwrap();
    assert_eq!(fmt, FileFormat::WordperfectDocument);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_wordperfect_document_2() {
    let fmt = FileFormat::from_file("fixtures/document/sample2.wpd").unwrap();
    assert_eq!(fmt, FileFormat::WordperfectDocument);
}
