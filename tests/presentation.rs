#[cfg(any(feature = "reader-cfb", feature = "reader-zip"))]
use file_format::FileFormat;

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_powerpoint_presentation() {
    let fmt = FileFormat::from_file("fixtures/presentation/sample.ppt").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftPowerpointPresentation);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_office_open_xml_presentation() {
    let fmt = FileFormat::from_file("fixtures/presentation/sample.pptx").unwrap();
    assert_eq!(fmt, FileFormat::OfficeOpenXmlPresentation);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_presentation() {
    let fmt = FileFormat::from_file("fixtures/presentation/sample.odp").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentPresentation);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_presentation_template() {
    let fmt = FileFormat::from_file("fixtures/presentation/sample.otp").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentPresentationTemplate);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_starimpress() {
    let fmt = FileFormat::from_file("fixtures/presentation/sample.sdd").unwrap();
    assert_eq!(fmt, FileFormat::Starimpress);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_sun_xml_impress() {
    let fmt = FileFormat::from_file("fixtures/presentation/sample.sxi").unwrap();
    assert_eq!(fmt, FileFormat::SunXmlImpress);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_sun_xml_impress_template() {
    let fmt = FileFormat::from_file("fixtures/presentation/sample.sti").unwrap();
    assert_eq!(fmt, FileFormat::SunXmlImpressTemplate);
}
