use file_format::FileFormat;

#[test]
fn test_microsoft_access2007_database() {
    let fmt = FileFormat::from_file("fixtures/database/sample.accdb").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftAccess2007Database);
}

#[test]
fn test_microsoft_access_database() {
    let fmt = FileFormat::from_file("fixtures/database/sample.mdb").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftAccessDatabase);
}

#[test]
fn test_microsoft_works_database_1() {
    let fmt = FileFormat::from_file("fixtures/database/sample1.wdb").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftWorksDatabase);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_works_database_2() {
    let fmt = FileFormat::from_file("fixtures/database/sample2.wdb").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftWorksDatabase);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_opendocument_database() {
    let fmt = FileFormat::from_file("fixtures/database/sample.odb").unwrap();
    assert_eq!(fmt, FileFormat::OpendocumentDatabase);
}

#[test]
fn test_sqlite3() {
    let fmt = FileFormat::from_file("fixtures/database/sample.sqlite").unwrap();
    assert_eq!(fmt, FileFormat::Sqlite3);
}
