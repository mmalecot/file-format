use file_format::FileFormat;

#[test]
fn test_der_certificate() {
    let fmt = FileFormat::from_file("fixtures/certificate/sample.der").unwrap();
    assert_eq!(fmt, FileFormat::DerCertificate);
}

#[test]
fn test_pem_certificate() {
    let fmt = FileFormat::from_file("fixtures/certificate/sample.crt").unwrap();
    assert_eq!(fmt, FileFormat::PemCertificate);
}
