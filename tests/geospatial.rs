use file_format::FileFormat;

#[test]
fn test_flexible_and_interoperable_data_transfer() {
    let fmt = FileFormat::from_file("fixtures/geospatial/sample.fit").unwrap();
    assert_eq!(fmt, FileFormat::FlexibleAndInteroperableDataTransfer);
}

#[test]
fn test_geography_markup_language_1() {
    let fmt = FileFormat::from_file("fixtures/geospatial/sample1.gml").unwrap();
    assert_eq!(fmt, FileFormat::GeographyMarkupLanguage);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_geography_markup_language_2() {
    let fmt = FileFormat::from_file("fixtures/geospatial/sample2.gml").unwrap();
    assert_eq!(fmt, FileFormat::GeographyMarkupLanguage);
}

#[test]
fn test_gps_exchange_format_1() {
    let fmt = FileFormat::from_file("fixtures/geospatial/sample1.gpx").unwrap();
    assert_eq!(fmt, FileFormat::GpsExchangeFormat);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_gps_exchange_format_2() {
    let fmt = FileFormat::from_file("fixtures/geospatial/sample2.gpx").unwrap();
    assert_eq!(fmt, FileFormat::GpsExchangeFormat);
}

#[test]
fn test_keyhole_markup_language_1() {
    let fmt = FileFormat::from_file("fixtures/geospatial/sample1.kml").unwrap();
    assert_eq!(fmt, FileFormat::KeyholeMarkupLanguage);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_keyhole_markup_language_2() {
    let fmt = FileFormat::from_file("fixtures/geospatial/sample2.kml").unwrap();
    assert_eq!(fmt, FileFormat::KeyholeMarkupLanguage);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_keyhole_markup_language_zip() {
    let fmt = FileFormat::from_file("fixtures/geospatial/sample.kmz").unwrap();
    assert_eq!(fmt, FileFormat::KeyholeMarkupLanguageZip);
}

#[test]
fn test_shapefile() {
    let fmt = FileFormat::from_file("fixtures/geospatial/sample.shp").unwrap();
    assert_eq!(fmt, FileFormat::Shapefile);
}

#[test]
fn test_training_center_xml_1() {
    let fmt = FileFormat::from_file("fixtures/geospatial/sample1.tcx").unwrap();
    assert_eq!(fmt, FileFormat::TrainingCenterXml);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_training_center_xml_2() {
    let fmt = FileFormat::from_file("fixtures/geospatial/sample2.tcx").unwrap();
    assert_eq!(fmt, FileFormat::TrainingCenterXml);
}
