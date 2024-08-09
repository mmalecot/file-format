use file_format::FileFormat;

#[test]
#[cfg(feature = "from-media-type")]
fn test_microsoft_excel_spreadsheet() {
    let fmt = FileFormat::from_media_type("application/vnd.ms-excel");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftExcelSpreadsheet)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftExcelSpreadsheet);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_microsoft_works6_spreadsheet() {
    let fmt = FileFormat::from_media_type("application/vnd.ms-works");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftWorks6Spreadsheet)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftWorks6Spreadsheet);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_microsoft_works_spreadsheet() {
    let fmt = FileFormat::from_media_type("application/vnd.ms-works");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftWorksSpreadsheet)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftWorksSpreadsheet);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_office_open_xml_spreadsheet() {
    let fmt = FileFormat::from_media_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OfficeOpenXmlSpreadsheet)), "{:?} does not contain {}", fmt, FileFormat::OfficeOpenXmlSpreadsheet);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_opendocument_spreadsheet() {
    let fmt = FileFormat::from_media_type("application/vnd.oasis.opendocument.spreadsheet");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OpendocumentSpreadsheet)), "{:?} does not contain {}", fmt, FileFormat::OpendocumentSpreadsheet);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_opendocument_spreadsheet_template() {
    let fmt = FileFormat::from_media_type("application/vnd.oasis.opendocument.spreadsheet-template");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OpendocumentSpreadsheetTemplate)), "{:?} does not contain {}", fmt, FileFormat::OpendocumentSpreadsheetTemplate);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_starcalc(){
    let fmt = FileFormat::from_media_type("application/vnd.stardivision.calc");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Starcalc)), "{:?} does not contain {}", fmt, FileFormat::Starcalc);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_sun_xml_calc() {
    let fmt = FileFormat::from_media_type("application/vnd.sun.xml.calc");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SunXmlCalc)), "{:?} does not contain {}", fmt, FileFormat::SunXmlCalc);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_sun_xml_calc_template() {
    let fmt = FileFormat::from_media_type("application/vnd.sun.xml.calc.template");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SunXmlCalcTemplate)), "{:?} does not contain {}", fmt, FileFormat::SunXmlCalcTemplate);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_uniform_office_format_spreadsheet() {
    let fmt = FileFormat::from_media_type("application/vnd.uof.spreadsheet");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::UniformOfficeFormatSpreadsheet)), "{:?} does not contain {}", fmt, FileFormat::UniformOfficeFormatSpreadsheet);
}

