use file_format::FileFormat;

#[test]
#[cfg(feature = "from-media-type")]
fn test_corel_presentations() {
    let fmt = FileFormat::from_media_type("application/x-corelpresentations");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::CorelPresentations)), "{:?} does not contain {}", fmt, FileFormat::CorelPresentations);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_corel_presentations7() {
    let fmt = FileFormat::from_media_type("application/x-corelpresentations");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::CorelPresentations7)), "{:?} does not contain {}", fmt, FileFormat::CorelPresentations7);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_microsoft_powerpoint_presentation() {
    let fmt = FileFormat::from_media_type("application/vnd.ms-powerpoint");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftPowerpointPresentation)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftPowerpointPresentation);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_office_open_xml_presentation() {
    let fmt = FileFormat::from_media_type("application/vnd.openxmlformats-officedocument.presentationml.presentation");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OfficeOpenXmlPresentation)), "{:?} does not contain {}", fmt, FileFormat::OfficeOpenXmlPresentation);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_opendocument_presentation() {
    let fmt = FileFormat::from_media_type("application/vnd.oasis.opendocument.presentation");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OpendocumentPresentation)), "{:?} does not contain {}", fmt, FileFormat::OpendocumentPresentation);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_opendocument_presentation_template() {
    let fmt = FileFormat::from_media_type("application/vnd.oasis.opendocument.presentation-template");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OpendocumentPresentationTemplate)), "{:?} does not contain {}", fmt, FileFormat::OpendocumentPresentationTemplate);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_starimpress(){
    let fmt = FileFormat::from_media_type("application/vnd.stardivision.impress");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Starimpress)), "{:?} does not contain {}", fmt, FileFormat::Starimpress);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_sun_xml_impress() {
    let fmt = FileFormat::from_media_type("application/vnd.sun.xml.impress");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SunXmlImpress)), "{:?} does not contain {}", fmt, FileFormat::SunXmlImpress);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_sun_xml_impress_template() {
    let fmt = FileFormat::from_media_type("application/vnd.sun.xml.impress.template");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SunXmlImpressTemplate)), "{:?} does not contain {}", fmt, FileFormat::SunXmlImpressTemplate);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_uniform_office_format_presentation() {
    let fmt = FileFormat::from_media_type("application/vnd.uof.presentation");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::UniformOfficeFormatPresentation)), "{:?} does not contain {}", fmt, FileFormat::UniformOfficeFormatPresentation);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_wordperfect_presentations() {
    let fmt = FileFormat::from_media_type("application/vnd.wordperfect");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WordperfectPresentations)), "{:?} does not contain {}", fmt, FileFormat::WordperfectPresentations);
}

