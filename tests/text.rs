use file_format::FileFormat;

#[test]
fn test_html() {
    let format = FileFormat::from_file("fixtures/text/sample.html").unwrap();
    assert_eq!(format, FileFormat::Html);
    assert_eq!(format.media_type(), "text/html");
    assert_eq!(format.preferred_extension(), "html");
}

#[test]
fn test_ics() {
    let format = FileFormat::from_file("fixtures/text/sample.ics").unwrap();
    assert_eq!(format, FileFormat::ICalendar);
    assert_eq!(format.media_type(), "text/calendar");
    assert_eq!(format.preferred_extension(), "ics");
}

#[test]
fn test_rtf() {
    let format = FileFormat::from_file("fixtures/text/sample.rtf").unwrap();
    assert_eq!(format, FileFormat::Rtf);
    assert_eq!(format.media_type(), "text/rtf");
    assert_eq!(format.preferred_extension(), "rtf");
}

#[test]
fn test_txt() {
    let format = FileFormat::from_file("fixtures/text/sample.txt").unwrap();
    assert_eq!(format, FileFormat::Text);
    assert_eq!(format.media_type(), "text/plain");
    assert_eq!(format.preferred_extension(), "txt");
}

#[test]
fn test_vcf() {
    let format = FileFormat::from_file("fixtures/text/sample.vcf").unwrap();
    assert_eq!(format, FileFormat::VCard);
    assert_eq!(format.media_type(), "text/vcard");
    assert_eq!(format.preferred_extension(), "vcf");
}

#[test]
fn test_xml() {
    let format = FileFormat::from_file("fixtures/text/sample.xml").unwrap();
    assert_eq!(format, FileFormat::Xml);
    assert_eq!(format.media_type(), "text/xml");
    assert_eq!(format.preferred_extension(), "xml");
}
