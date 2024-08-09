use file_format::FileFormat;

#[test]
#[cfg(feature = "from-extension")]
fn test_microsoft_access2007_database() {
    let fmt = FileFormat::from_extension("accdb");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftAccess2007Database)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftAccess2007Database);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_microsoft_access_database() {
    let fmt = FileFormat::from_extension("mdb");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftAccessDatabase)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftAccessDatabase);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_microsoft_works_database(){
    let fmt = FileFormat::from_extension("wdb");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftWorksDatabase)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftWorksDatabase);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_opendocument_database() {
    let fmt = FileFormat::from_extension("odb");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OpendocumentDatabase)), "{:?} does not contain {}", fmt, FileFormat::OpendocumentDatabase);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_sqlite3(){
    let fmt = FileFormat::from_extension("sqlite");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Sqlite3)), "{:?} does not contain {}", fmt, FileFormat::Sqlite3);
}

