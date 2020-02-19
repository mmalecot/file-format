use file_format::{FileFormat, Kind};

#[test]
fn test_glb() {
    let format = FileFormat::from_file("fixtures/model/sample.glb").unwrap();
    assert_eq!(format.kind(), Kind::Model);
    assert_eq!(format.media_type(), "model/gltf-binary");
    assert_eq!(format.preferred_extension(), "glb");
}
