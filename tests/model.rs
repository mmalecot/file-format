use file_format::FileFormat;

#[test]
fn test_glb() {
    let format = FileFormat::from_file("fixtures/model/sample.glb").unwrap();
    assert_eq!(format, FileFormat::Gltf);
    assert_eq!(format.media_type(), "model/gltf-binary");
    assert_eq!(format.preferred_extension(), "glb");
}
