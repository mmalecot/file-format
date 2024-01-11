use file_format::FileFormat;

#[test]
fn test_mathematical_markup_language_1() {
    let fmt = FileFormat::from_file("fixtures/mathematical/sample1.mathml").unwrap();
    assert_eq!(fmt, FileFormat::MathematicalMarkupLanguage);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_mathematical_markup_language_2() {
    let fmt = FileFormat::from_file("fixtures/mathematical/sample2.mathml").unwrap();
    assert_eq!(fmt, FileFormat::MathematicalMarkupLanguage);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_formula() {
    let fmt = FileFormat::from_file("fixtures/mathematical/sample.odf").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentFormula);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_formula_template() {
    let fmt = FileFormat::from_file("fixtures/mathematical/sample.otf").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentFormulaTemplate);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_starmath() {
    let fmt = FileFormat::from_file("fixtures/mathematical/sample.smf").unwrap();
    assert_eq!(fmt, FileFormat::Starmath);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_sun_xml_math() {
    let fmt = FileFormat::from_file("fixtures/mathematical/sample.sxm").unwrap();
    assert_eq!(fmt, FileFormat::SunXmlMath);
}
