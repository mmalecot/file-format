use file_format::FileFormat;

#[cfg(feature = "reader-xml")]
#[test]
fn test_abiword() {
    let format = FileFormat::from_file("fixtures/document/sample.abw").unwrap();
    assert_eq!(format, FileFormat::Abiword);
}

#[test]
fn test_abiword_no_xml() {
    let format = FileFormat::from_file("fixtures/document/sample-no-xml.abw").unwrap();
    assert_eq!(format, FileFormat::Abiword);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_abiword_template() {
    let format = FileFormat::from_file("fixtures/document/sample.awt").unwrap();
    assert_eq!(format, FileFormat::AbiwordTemplate);
}

#[test]
fn test_abiword_template_no_xml() {
    let format = FileFormat::from_file("fixtures/document/sample-no-xml.awt").unwrap();
    assert_eq!(format, FileFormat::AbiwordTemplate);
}

#[test]
fn test_adobe_indesign_document() {
    let format = FileFormat::from_file("fixtures/document/sample.indd").unwrap();
    assert_eq!(format, FileFormat::AdobeIndesignDocument);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_circuit_diagram_document() {
    let format = FileFormat::from_file("fixtures/document/sample.cddx").unwrap();
    assert_eq!(format, FileFormat::CircuitDiagramDocument);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_drawio() {
    let format = FileFormat::from_file("fixtures/document/sample.drawio").unwrap();
    assert_eq!(format, FileFormat::Drawio);
}

#[test]
fn test_drawio_no_xml() {
    let format = FileFormat::from_file("fixtures/document/sample-no-xml.drawio").unwrap();
    assert_eq!(format, FileFormat::Drawio);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_indesign_markup_language() {
    let format = FileFormat::from_file("fixtures/document/sample.idml").unwrap();
    assert_eq!(format, FileFormat::IndesignMarkupLanguage);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_excel_spreadsheet() {
    let format = FileFormat::from_file("fixtures/document/sample.xls").unwrap();
    assert_eq!(format, FileFormat::MicrosoftExcelSpreadsheet);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_powerpoint_presentation() {
    let format = FileFormat::from_file("fixtures/document/sample.ppt").unwrap();
    assert_eq!(format, FileFormat::MicrosoftPowerpointPresentation);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_project_plan() {
    let format = FileFormat::from_file("fixtures/document/sample.mpp").unwrap();
    assert_eq!(format, FileFormat::MicrosoftProjectPlan);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_publisher_document() {
    let format = FileFormat::from_file("fixtures/document/sample-publisher.pub").unwrap();
    assert_eq!(format, FileFormat::MicrosoftPublisherDocument);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_visio_drawing() {
    let format = FileFormat::from_file("fixtures/document/sample.vsd").unwrap();
    assert_eq!(format, FileFormat::MicrosoftVisioDrawing);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_word_document() {
    let format = FileFormat::from_file("fixtures/document/sample.doc").unwrap();
    assert_eq!(format, FileFormat::MicrosoftWordDocument);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_office_open_xml_document() {
    let format = FileFormat::from_file("fixtures/document/sample.docx").unwrap();
    assert_eq!(format, FileFormat::OfficeOpenXmlDocument);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_office_open_xml_drawing() {
    let format = FileFormat::from_file("fixtures/document/sample.vsdx").unwrap();
    assert_eq!(format, FileFormat::OfficeOpenXmlDrawing);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_office_open_xml_presentation() {
    let format = FileFormat::from_file("fixtures/document/sample.pptx").unwrap();
    assert_eq!(format, FileFormat::OfficeOpenXmlPresentation);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_office_open_xml_spreadsheet() {
    let format = FileFormat::from_file("fixtures/document/sample.xlsx").unwrap();
    assert_eq!(format, FileFormat::OfficeOpenXmlSpreadsheet);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_formula() {
    let format = FileFormat::from_file("fixtures/document/sample.odf").unwrap();
    assert_eq!(format, FileFormat::OpendocumentFormula);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_formula_template() {
    let format = FileFormat::from_file("fixtures/document/sample.otf").unwrap();
    assert_eq!(format, FileFormat::OpendocumentFormulaTemplate);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_graphics() {
    let format = FileFormat::from_file("fixtures/document/sample.odg").unwrap();
    assert_eq!(format, FileFormat::OpendocumentGraphics);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_graphics_template() {
    let format = FileFormat::from_file("fixtures/document/sample.otg").unwrap();
    assert_eq!(format, FileFormat::OpendocumentGraphicsTemplate);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_presentation() {
    let format = FileFormat::from_file("fixtures/document/sample.odp").unwrap();
    assert_eq!(format, FileFormat::OpendocumentPresentation);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_presentation_template() {
    let format = FileFormat::from_file("fixtures/document/sample.otp").unwrap();
    assert_eq!(format, FileFormat::OpendocumentPresentationTemplate);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_spreadsheet() {
    let format = FileFormat::from_file("fixtures/document/sample.ods").unwrap();
    assert_eq!(format, FileFormat::OpendocumentSpreadsheet);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_spreadsheet_template() {
    let format = FileFormat::from_file("fixtures/document/sample.ots").unwrap();
    assert_eq!(format, FileFormat::OpendocumentSpreadsheetTemplate);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_text() {
    let format = FileFormat::from_file("fixtures/document/sample.odt").unwrap();
    assert_eq!(format, FileFormat::OpendocumentText);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_text_master() {
    let format = FileFormat::from_file("fixtures/document/sample.odm").unwrap();
    assert_eq!(format, FileFormat::OpendocumentTextMaster);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_text_master_template() {
    let format = FileFormat::from_file("fixtures/document/sample.otm").unwrap();
    assert_eq!(format, FileFormat::OpendocumentTextMasterTemplate);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_text_template() {
    let format = FileFormat::from_file("fixtures/document/sample.ott").unwrap();
    assert_eq!(format, FileFormat::OpendocumentTextTemplate);
}

#[test]
fn test_portable_document_format() {
    let format = FileFormat::from_file("fixtures/document/sample.pdf").unwrap();
    assert_eq!(format, FileFormat::PortableDocumentFormat);
}

#[test]
fn test_rich_text_format() {
    let format = FileFormat::from_file("fixtures/document/sample.rtf").unwrap();
    assert_eq!(format, FileFormat::RichTextFormat);
}
