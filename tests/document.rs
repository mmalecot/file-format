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

#[cfg(feature = "reader-zip")]
#[test]
fn test_circuit_diagram_document() {
    let fmt = FileFormat::from_file("fixtures/document/sample.cddx").unwrap();
    assert_eq!(fmt, FileFormat::CircuitDiagramDocument);
}

#[test]
fn test_drawio_1() {
    let fmt = FileFormat::from_file("fixtures/document/sample1.drawio").unwrap();
    assert_eq!(fmt, FileFormat::Drawio);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_drawio_2() {
    let fmt = FileFormat::from_file("fixtures/document/sample2.drawio").unwrap();
    assert_eq!(fmt, FileFormat::Drawio);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_indesign_markup_language() {
    let fmt = FileFormat::from_file("fixtures/document/sample.idml").unwrap();
    assert_eq!(fmt, FileFormat::IndesignMarkupLanguage);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_excel_spreadsheet() {
    let fmt = FileFormat::from_file("fixtures/document/sample.xls").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftExcelSpreadsheet);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_powerpoint_presentation() {
    let fmt = FileFormat::from_file("fixtures/document/sample.ppt").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftPowerpointPresentation);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_project_plan() {
    let fmt = FileFormat::from_file("fixtures/document/sample.mpp").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftProjectPlan);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_publisher_document() {
    let fmt = FileFormat::from_file("fixtures/document/sample.pub").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftPublisherDocument);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_visio_drawing() {
    let fmt = FileFormat::from_file("fixtures/document/sample.vsd").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftVisioDrawing);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_word_document() {
    let fmt = FileFormat::from_file("fixtures/document/sample.doc").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftWordDocument);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_works6_spreadsheet() {
    let fmt = FileFormat::from_file("fixtures/document/sample.xlr").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftWorks6Spreadsheet);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_works_spreadsheet() {
    let fmt = FileFormat::from_file("fixtures/document/sample.wks").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftWorksSpreadsheet);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_works_word_processor() {
    let fmt = FileFormat::from_file("fixtures/document/sample.wps").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftWorksWordProcessor);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_office_open_xml_document() {
    let fmt = FileFormat::from_file("fixtures/document/sample.docx").unwrap();
    assert_eq!(fmt, FileFormat::OfficeOpenXmlDocument);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_office_open_xml_drawing() {
    let fmt = FileFormat::from_file("fixtures/document/sample.vsdx").unwrap();
    assert_eq!(fmt, FileFormat::OfficeOpenXmlDrawing);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_office_open_xml_presentation() {
    let fmt = FileFormat::from_file("fixtures/document/sample.pptx").unwrap();
    assert_eq!(fmt, FileFormat::OfficeOpenXmlPresentation);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_office_open_xml_spreadsheet() {
    let fmt = FileFormat::from_file("fixtures/document/sample.xlsx").unwrap();
    assert_eq!(fmt, FileFormat::OfficeOpenXmlSpreadsheet);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_formula() {
    let fmt = FileFormat::from_file("fixtures/document/sample.odf").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentFormula);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_formula_template() {
    let fmt = FileFormat::from_file("fixtures/document/sample.otf").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentFormulaTemplate);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_graphics() {
    let fmt = FileFormat::from_file("fixtures/document/sample.odg").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentGraphics);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_graphics_template() {
    let fmt = FileFormat::from_file("fixtures/document/sample.otg").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentGraphicsTemplate);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_presentation() {
    let fmt = FileFormat::from_file("fixtures/document/sample.odp").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentPresentation);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_presentation_template() {
    let fmt = FileFormat::from_file("fixtures/document/sample.otp").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentPresentationTemplate);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_spreadsheet() {
    let fmt = FileFormat::from_file("fixtures/document/sample.ods").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentSpreadsheet);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_spreadsheet_template() {
    let fmt = FileFormat::from_file("fixtures/document/sample.ots").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentSpreadsheetTemplate);
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

#[test]
fn test_portable_document_format() {
    let fmt = FileFormat::from_file("fixtures/document/sample.pdf").unwrap();
    assert_eq!(fmt, FileFormat::PortableDocumentFormat);
}

#[test]
fn test_rich_text_format() {
    let fmt = FileFormat::from_file("fixtures/document/sample.rtf").unwrap();
    assert_eq!(fmt, FileFormat::RichTextFormat);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_starcalc() {
    let fmt = FileFormat::from_file("fixtures/document/sample.sdc").unwrap();
    assert_eq!(fmt, FileFormat::Starcalc);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_starchart() {
    let fmt = FileFormat::from_file("fixtures/document/sample.sds").unwrap();
    assert_eq!(fmt, FileFormat::Starchart);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_stardraw() {
    let fmt = FileFormat::from_file("fixtures/document/sample.sda").unwrap();
    assert_eq!(fmt, FileFormat::Stardraw);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_starimpress() {
    let fmt = FileFormat::from_file("fixtures/document/sample.sdd").unwrap();
    assert_eq!(fmt, FileFormat::Starimpress);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_starmath() {
    let fmt = FileFormat::from_file("fixtures/document/sample.smf").unwrap();
    assert_eq!(fmt, FileFormat::Starmath);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_starwriter() {
    let fmt = FileFormat::from_file("fixtures/document/sample.sdw").unwrap();
    assert_eq!(fmt, FileFormat::Starwriter);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_sun_xml_calc() {
    let fmt = FileFormat::from_file("fixtures/document/sample.sxc").unwrap();
    assert_eq!(fmt, FileFormat::SunXmlCalc);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_sun_xml_calc_template() {
    let fmt = FileFormat::from_file("fixtures/document/sample.stc").unwrap();
    assert_eq!(fmt, FileFormat::SunXmlCalcTemplate);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_sun_xml_draw() {
    let fmt = FileFormat::from_file("fixtures/document/sample.sxd").unwrap();
    assert_eq!(fmt, FileFormat::SunXmlDraw);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_sun_xml_draw_template() {
    let fmt = FileFormat::from_file("fixtures/document/sample.std").unwrap();
    assert_eq!(fmt, FileFormat::SunXmlDrawTemplate);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_sun_xml_impress() {
    let fmt = FileFormat::from_file("fixtures/document/sample.sxi").unwrap();
    assert_eq!(fmt, FileFormat::SunXmlImpress);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_sun_xml_impress_template() {
    let fmt = FileFormat::from_file("fixtures/document/sample.sti").unwrap();
    assert_eq!(fmt, FileFormat::SunXmlImpressTemplate);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_sun_xml_math() {
    let fmt = FileFormat::from_file("fixtures/document/sample.sxm").unwrap();
    assert_eq!(fmt, FileFormat::SunXmlMath);
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

#[test]
fn test_wordperfect_graphics_1() {
    let fmt = FileFormat::from_file("fixtures/document/sample1.wpg").unwrap();
    assert_eq!(fmt, FileFormat::WordperfectGraphics);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_wordperfect_graphics_2() {
    let fmt = FileFormat::from_file("fixtures/document/sample2.wpg").unwrap();
    assert_eq!(fmt, FileFormat::WordperfectGraphics);
}

#[test]
fn test_wordperfect_macro() {
    let fmt = FileFormat::from_file("fixtures/document/sample.wpm").unwrap();
    assert_eq!(fmt, FileFormat::WordperfectMacro);
}
