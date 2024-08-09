use file_format::FileFormat;

#[test]
#[cfg(feature = "from-extension")]
fn test_mathematical_markup_language(){
    let fmt = FileFormat::from_extension("mathml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MathematicalMarkupLanguage)), "{:?} does not contain {}", fmt, FileFormat::MathematicalMarkupLanguage);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_opendocument_formula() {
    let fmt = FileFormat::from_extension("odf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OpendocumentFormula)), "{:?} does not contain {}", fmt, FileFormat::OpendocumentFormula);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_opendocument_formula_template() {
    let fmt = FileFormat::from_extension("otf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OpendocumentFormulaTemplate)), "{:?} does not contain {}", fmt, FileFormat::OpendocumentFormulaTemplate);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_starmath(){
    let fmt = FileFormat::from_extension("smf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Starmath)), "{:?} does not contain {}", fmt, FileFormat::Starmath);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_sun_xml_math() {
    let fmt = FileFormat::from_extension("sxm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SunXmlMath)), "{:?} does not contain {}", fmt, FileFormat::SunXmlMath);
}

