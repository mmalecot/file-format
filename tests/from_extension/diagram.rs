use file_format::FileFormat;

#[test]
#[cfg(feature = "from-extension")]
fn test_circuit_diagram_document() {
    let fmt = FileFormat::from_extension("cddx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::CircuitDiagramDocument)), "{:?} does not contain {}", fmt, FileFormat::CircuitDiagramDocument);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_drawio(){
    let fmt = FileFormat::from_extension("drawio");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Drawio)), "{:?} does not contain {}", fmt, FileFormat::Drawio);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_microsoft_visio_drawing() {
    let fmt = FileFormat::from_extension("vsd");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftVisioDrawing)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftVisioDrawing);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_office_open_xml_drawing() {
    let fmt = FileFormat::from_extension("vsdx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OfficeOpenXmlDrawing)), "{:?} does not contain {}", fmt, FileFormat::OfficeOpenXmlDrawing);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_starchart(){
    let fmt = FileFormat::from_extension("sds");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Starchart)), "{:?} does not contain {}", fmt, FileFormat::Starchart);
}

