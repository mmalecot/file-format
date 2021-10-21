use file_format::FileFormat;

#[test]
fn test_3mf() {
    let format = FileFormat::from_file("fixtures/model/sample.3mf").unwrap();
    assert_eq!(format.media_type(), "model/3mf");
    assert_eq!(format.extension(), "3mf");
}

#[test]
fn test_glb() {
    let format = FileFormat::from_file("fixtures/model/sample.glb").unwrap();
    assert_eq!(format.media_type(), "model/gltf-binary");
    assert_eq!(format.extension(), "glb");
}
