use file_format::FileFormat;

#[test]
#[cfg(feature = "from-media-type")]
fn test_microsoft_access2007_database() {
    let fmt = FileFormat::from_media_type("application/x-msaccess");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftAccess2007Database)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftAccess2007Database);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_microsoft_access_database() {
    let fmt = FileFormat::from_media_type("application/x-msaccess");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftAccessDatabase)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftAccessDatabase);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_microsoft_works_database(){
    let fmt = FileFormat::from_media_type("application/vnd.ms-works-db");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftWorksDatabase)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftWorksDatabase);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_opendocument_database() {
    let fmt = FileFormat::from_media_type("application/vnd.oasis.opendocument.database");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OpendocumentDatabase)), "{:?} does not contain {}", fmt, FileFormat::OpendocumentDatabase);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_sqlite3(){
    let fmt = FileFormat::from_media_type("application/vnd.sqlite3");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Sqlite3)), "{:?} does not contain {}", fmt, FileFormat::Sqlite3);
}

