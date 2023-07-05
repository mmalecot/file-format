use file_format::FileFormat;

#[test]
fn test_atom_1() {
    let fmt = FileFormat::from_file("fixtures/syndication/sample1.atom").unwrap();
    assert_eq!(fmt, FileFormat::Atom);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_atom_2() {
    let fmt = FileFormat::from_file("fixtures/syndication/sample2.atom").unwrap();
    assert_eq!(fmt, FileFormat::Atom);
}

#[test]
fn test_really_simple_syndication_1() {
    let fmt = FileFormat::from_file("fixtures/syndication/sample1.rss").unwrap();
    assert_eq!(fmt, FileFormat::ReallySimpleSyndication);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_really_simple_syndication_2() {
    let fmt = FileFormat::from_file("fixtures/syndication/sample2.rss").unwrap();
    assert_eq!(fmt, FileFormat::ReallySimpleSyndication);
}
