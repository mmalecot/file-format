use file_format::Kind;

#[cfg(feature = "map-media-to-kind")]
#[test]
fn test_media_to_kind() {
	let media = "application/zip";
	let kind = Kind::Archive;
	let candidates = Kind::from_media_type(media).unwrap();
	let found = candidates.iter().find(|&&k| k == kind).unwrap();
	assert_eq!(*found, kind);
}
