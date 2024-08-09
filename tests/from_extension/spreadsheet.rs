use file_format::FileFormat;

#[test]
#[cfg(feature = "from-extension")]
fn test_microsoft_excel_spreadsheet() {
    let fmt = FileFormat::from_extension("xls");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftExcelSpreadsheet)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftExcelSpreadsheet);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_microsoft_works6_spreadsheet() {
    let fmt = FileFormat::from_extension("xlr");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftWorks6Spreadsheet)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftWorks6Spreadsheet);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_microsoft_works_spreadsheet() {
    let fmt = FileFormat::from_extension("wks");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftWorksSpreadsheet)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftWorksSpreadsheet);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_office_open_xml_spreadsheet() {
    let fmt = FileFormat::from_extension("xlsx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OfficeOpenXmlSpreadsheet)), "{:?} does not contain {}", fmt, FileFormat::OfficeOpenXmlSpreadsheet);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_opendocument_spreadsheet() {
    let fmt = FileFormat::from_extension("ods");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OpendocumentSpreadsheet)), "{:?} does not contain {}", fmt, FileFormat::OpendocumentSpreadsheet);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_opendocument_spreadsheet_template() {
    let fmt = FileFormat::from_extension("ots");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OpendocumentSpreadsheetTemplate)), "{:?} does not contain {}", fmt, FileFormat::OpendocumentSpreadsheetTemplate);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_starcalc(){
    let fmt = FileFormat::from_extension("sdc");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Starcalc)), "{:?} does not contain {}", fmt, FileFormat::Starcalc);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_sun_xml_calc() {
    let fmt = FileFormat::from_extension("sxc");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SunXmlCalc)), "{:?} does not contain {}", fmt, FileFormat::SunXmlCalc);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_sun_xml_calc_template() {
    let fmt = FileFormat::from_extension("stc");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SunXmlCalcTemplate)), "{:?} does not contain {}", fmt, FileFormat::SunXmlCalcTemplate);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_uniform_office_format_spreadsheet() {
    let fmt = FileFormat::from_extension("uos");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::UniformOfficeFormatSpreadsheet)), "{:?} does not contain {}", fmt, FileFormat::UniformOfficeFormatSpreadsheet);
}

