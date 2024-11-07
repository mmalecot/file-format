use file_format::FileFormat;

#[test]
#[cfg(feature = "from-media-type")]
fn test_circuit_diagram_document() {
    let fmt = FileFormat::from_media_type("application/vnd.circuitdiagram.document.main+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::CircuitDiagramDocument)), "{:?} does not contain {}", fmt, FileFormat::CircuitDiagramDocument);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_drawio(){
    let fmt = FileFormat::from_media_type("application/vnd.jgraph.mxfile");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Drawio)), "{:?} does not contain {}", fmt, FileFormat::Drawio);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_microsoft_visio_drawing() {
    let fmt = FileFormat::from_media_type("application/vnd.visio");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftVisioDrawing)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftVisioDrawing);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_office_open_xml_drawing() {
    let fmt = FileFormat::from_media_type("application/vnd.ms-visio.drawing.main+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OfficeOpenXmlDrawing)), "{:?} does not contain {}", fmt, FileFormat::OfficeOpenXmlDrawing);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_starchart(){
    let fmt = FileFormat::from_media_type("application/vnd.stardivision.chart");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Starchart)), "{:?} does not contain {}", fmt, FileFormat::Starchart);
}

