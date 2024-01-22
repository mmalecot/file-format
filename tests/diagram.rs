use file_format::FileFormat;

#[cfg(feature = "reader-zip")]
#[test]
fn test_circuit_diagram_document() {
    let fmt = FileFormat::from_file("fixtures/diagram/sample.cddx").unwrap();
    assert_eq!(fmt, FileFormat::CircuitDiagramDocument);
}

#[test]
fn test_drawio_1() {
    let fmt = FileFormat::from_file("fixtures/diagram/sample1.drawio").unwrap();
    assert_eq!(fmt, FileFormat::Drawio);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_drawio_2() {
    let fmt = FileFormat::from_file("fixtures/diagram/sample2.drawio").unwrap();
    assert_eq!(fmt, FileFormat::Drawio);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_visio_drawing() {
    let fmt = FileFormat::from_file("fixtures/diagram/sample.vsd").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftVisioDrawing);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_office_open_xml_drawing() {
    let fmt = FileFormat::from_file("fixtures/diagram/sample.vsdx").unwrap();
    assert_eq!(fmt, FileFormat::OfficeOpenXmlDrawing);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_starchart() {
    let fmt = FileFormat::from_file("fixtures/diagram/sample.sds").unwrap();
    assert_eq!(fmt, FileFormat::Starchart);
}
