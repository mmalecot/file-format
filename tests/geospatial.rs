use file_format::FileFormat;

#[test]
fn test_flexible_and_interoperable_data_transfer() {
    let format = FileFormat::from_file("fixtures/geospatial/sample.fit").unwrap();
    assert_eq!(format, FileFormat::FlexibleAndInteroperableDataTransfer);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_geography_markup_language() {
    let format = FileFormat::from_file("fixtures/geospatial/sample.gml").unwrap();
    assert_eq!(format, FileFormat::GeographyMarkupLanguage);
}

#[test]
fn test_geography_markup_language_no_xml() {
    let format = FileFormat::from_file("fixtures/geospatial/sample-no-xml.gml").unwrap();
    assert_eq!(format, FileFormat::GeographyMarkupLanguage);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_gps_exchange_format() {
    let format = FileFormat::from_file("fixtures/geospatial/sample.gpx").unwrap();
    assert_eq!(format, FileFormat::GpsExchangeFormat);
}

#[test]
fn test_gps_exchange_format_no_xml() {
    let format = FileFormat::from_file("fixtures/geospatial/sample-no-xml.gpx").unwrap();
    assert_eq!(format, FileFormat::GpsExchangeFormat);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_keyhole_markup_language() {
    let format = FileFormat::from_file("fixtures/geospatial/sample.kml").unwrap();
    assert_eq!(format, FileFormat::KeyholeMarkupLanguage);
}

#[test]
fn test_keyhole_markup_language_no_xml() {
    let format = FileFormat::from_file("fixtures/geospatial/sample-no-xml.kml").unwrap();
    assert_eq!(format, FileFormat::KeyholeMarkupLanguage);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_keyhole_markup_language_zipped() {
    let format = FileFormat::from_file("fixtures/geospatial/sample.kmz").unwrap();
    assert_eq!(format, FileFormat::KeyholeMarkupLanguageZipped);
}

#[test]
fn test_shapefile() {
    let format = FileFormat::from_file("fixtures/geospatial/sample.shp").unwrap();
    assert_eq!(format, FileFormat::Shapefile);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_training_center_xml() {
    let format = FileFormat::from_file("fixtures/geospatial/sample.tcx").unwrap();
    assert_eq!(format, FileFormat::TrainingCenterXml);
}

#[test]
fn test_training_center_xml_no_xml() {
    let format = FileFormat::from_file("fixtures/geospatial/sample-no-xml.tcx").unwrap();
    assert_eq!(format, FileFormat::TrainingCenterXml);
}
