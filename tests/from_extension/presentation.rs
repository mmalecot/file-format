use file_format::FileFormat;

#[test]
#[cfg(feature = "from-extension")]
fn test_corel_presentations() {
    let fmt = FileFormat::from_extension("shw");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::CorelPresentations)), "{:?} does not contain {}", fmt, FileFormat::CorelPresentations);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_corel_presentations7() {
    let fmt = FileFormat::from_extension("shw");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::CorelPresentations7)), "{:?} does not contain {}", fmt, FileFormat::CorelPresentations7);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_microsoft_powerpoint_presentation() {
    let fmt = FileFormat::from_extension("ppt");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftPowerpointPresentation)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftPowerpointPresentation);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_office_open_xml_presentation() {
    let fmt = FileFormat::from_extension("pptx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OfficeOpenXmlPresentation)), "{:?} does not contain {}", fmt, FileFormat::OfficeOpenXmlPresentation);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_opendocument_presentation() {
    let fmt = FileFormat::from_extension("odp");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OpendocumentPresentation)), "{:?} does not contain {}", fmt, FileFormat::OpendocumentPresentation);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_opendocument_presentation_template() {
    let fmt = FileFormat::from_extension("otp");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OpendocumentPresentationTemplate)), "{:?} does not contain {}", fmt, FileFormat::OpendocumentPresentationTemplate);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_starimpress(){
    let fmt = FileFormat::from_extension("sdd");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Starimpress)), "{:?} does not contain {}", fmt, FileFormat::Starimpress);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_sun_xml_impress() {
    let fmt = FileFormat::from_extension("sxi");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SunXmlImpress)), "{:?} does not contain {}", fmt, FileFormat::SunXmlImpress);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_sun_xml_impress_template() {
    let fmt = FileFormat::from_extension("sti");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SunXmlImpressTemplate)), "{:?} does not contain {}", fmt, FileFormat::SunXmlImpressTemplate);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_uniform_office_format_presentation() {
    let fmt = FileFormat::from_extension("uop");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::UniformOfficeFormatPresentation)), "{:?} does not contain {}", fmt, FileFormat::UniformOfficeFormatPresentation);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_wordperfect_presentations() {
    let fmt = FileFormat::from_extension("shw");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WordperfectPresentations)), "{:?} does not contain {}", fmt, FileFormat::WordperfectPresentations);
}

