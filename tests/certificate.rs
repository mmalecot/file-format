use file_format::FileFormat;

#[test]
fn test_der_certificate() {
    let format = FileFormat::from_file("fixtures/certificate/sample.der").unwrap();
    assert_eq!(format, FileFormat::DerCertificate);
}

#[test]
fn test_pem_certificate() {
    let format = FileFormat::from_file("fixtures/certificate/sample.crt").unwrap();
    assert_eq!(format, FileFormat::PemCertificate);
}
