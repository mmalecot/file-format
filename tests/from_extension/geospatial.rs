use file_format::FileFormat;

#[test]
#[cfg(feature = "from-extension")]
fn test_flexible_and_interoperable_data_transfer() {
    let fmt = FileFormat::from_extension("fit");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::FlexibleAndInteroperableDataTransfer)), "{:?} does not contain {}", fmt, FileFormat::FlexibleAndInteroperableDataTransfer);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_geography_markup_language(){
    let fmt = FileFormat::from_extension("gml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::GeographyMarkupLanguage)), "{:?} does not contain {}", fmt, FileFormat::GeographyMarkupLanguage);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_gps_exchange_format(){
    let fmt = FileFormat::from_extension("gpx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::GpsExchangeFormat)), "{:?} does not contain {}", fmt, FileFormat::GpsExchangeFormat);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_keyhole_markup_language(){
    let fmt = FileFormat::from_extension("kml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::KeyholeMarkupLanguage)), "{:?} does not contain {}", fmt, FileFormat::KeyholeMarkupLanguage);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_keyhole_markup_language_zip() {
    let fmt = FileFormat::from_extension("kmz");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::KeyholeMarkupLanguageZip)), "{:?} does not contain {}", fmt, FileFormat::KeyholeMarkupLanguageZip);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_shapefile(){
    let fmt = FileFormat::from_extension("shp");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Shapefile)), "{:?} does not contain {}", fmt, FileFormat::Shapefile);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_training_center_xml(){
    let fmt = FileFormat::from_extension("tcx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::TrainingCenterXml)), "{:?} does not contain {}", fmt, FileFormat::TrainingCenterXml);
}

