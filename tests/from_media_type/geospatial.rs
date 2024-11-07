use file_format::FileFormat;

#[test]
#[cfg(feature = "from-media-type")]
fn test_flexible_and_interoperable_data_transfer() {
    let fmt = FileFormat::from_media_type("application/x-fit");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::FlexibleAndInteroperableDataTransfer)), "{:?} does not contain {}", fmt, FileFormat::FlexibleAndInteroperableDataTransfer);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_geography_markup_language(){
    let fmt = FileFormat::from_media_type("application/gml+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::GeographyMarkupLanguage)), "{:?} does not contain {}", fmt, FileFormat::GeographyMarkupLanguage);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_gps_exchange_format(){
    let fmt = FileFormat::from_media_type("application/gpx+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::GpsExchangeFormat)), "{:?} does not contain {}", fmt, FileFormat::GpsExchangeFormat);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_keyhole_markup_language(){
    let fmt = FileFormat::from_media_type("application/vnd.google-earth.kml+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::KeyholeMarkupLanguage)), "{:?} does not contain {}", fmt, FileFormat::KeyholeMarkupLanguage);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_keyhole_markup_language_zip() {
    let fmt = FileFormat::from_media_type("application/vnd.google-earth.kmz");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::KeyholeMarkupLanguageZip)), "{:?} does not contain {}", fmt, FileFormat::KeyholeMarkupLanguageZip);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_shapefile(){
    let fmt = FileFormat::from_media_type("application/x-esri-shape");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Shapefile)), "{:?} does not contain {}", fmt, FileFormat::Shapefile);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_training_center_xml(){
    let fmt = FileFormat::from_media_type("application/vnd.garmin.tcx+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::TrainingCenterXml)), "{:?} does not contain {}", fmt, FileFormat::TrainingCenterXml);
}

