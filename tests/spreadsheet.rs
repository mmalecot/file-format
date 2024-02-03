use file_format::FileFormat;

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_excel_spreadsheet() {
    let fmt = FileFormat::from_file("fixtures/spreadsheet/sample.xls").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftExcelSpreadsheet);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_works6_spreadsheet() {
    let fmt = FileFormat::from_file("fixtures/spreadsheet/sample.xlr").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftWorks6Spreadsheet);
}

#[test]
fn test_microsoft_works_spreadsheet() {
    let fmt = FileFormat::from_file("fixtures/spreadsheet/sample.wks").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftWorksSpreadsheet);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_office_open_xml_spreadsheet() {
    let fmt = FileFormat::from_file("fixtures/spreadsheet/sample.xlsx").unwrap();
    assert_eq!(fmt, FileFormat::OfficeOpenXmlSpreadsheet);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_spreadsheet() {
    let fmt = FileFormat::from_file("fixtures/spreadsheet/sample.ods").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentSpreadsheet);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_spreadsheet_template() {
    let fmt = FileFormat::from_file("fixtures/spreadsheet/sample.ots").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentSpreadsheetTemplate);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_starcalc() {
    let fmt = FileFormat::from_file("fixtures/spreadsheet/sample.sdc").unwrap();
    assert_eq!(fmt, FileFormat::Starcalc);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_sun_xml_calc() {
    let fmt = FileFormat::from_file("fixtures/spreadsheet/sample.sxc").unwrap();
    assert_eq!(fmt, FileFormat::SunXmlCalc);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_sun_xml_calc_template() {
    let fmt = FileFormat::from_file("fixtures/spreadsheet/sample.stc").unwrap();
    assert_eq!(fmt, FileFormat::SunXmlCalcTemplate);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_uniform_office_format_spreadsheet() {
    let fmt = FileFormat::from_file("fixtures/spreadsheet/sample.uos").unwrap();
    assert_eq!(fmt, FileFormat::UniformOfficeFormatSpreadsheet);
}
