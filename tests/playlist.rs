use file_format::FileFormat;

#[cfg(feature = "reader-xml")]
#[test]
fn test_advanced_stream_redirector() {
    let format = FileFormat::from_file("fixtures/playlist/sample.asx").unwrap();
    assert_eq!(format, FileFormat::AdvancedStreamRedirector);
}

#[test]
fn test_advanced_stream_redirector_no_xml() {
    let format = FileFormat::from_file("fixtures/playlist/sample-no-xml.asx").unwrap();
    assert_eq!(format, FileFormat::AdvancedStreamRedirector);
}

#[test]
fn test_mp3_url() {
    let format = FileFormat::from_file("fixtures/playlist/sample.m3u").unwrap();
    assert_eq!(format, FileFormat::Mp3Url);
}

#[test]
fn test_shoutcast_playlist() {
    let format = FileFormat::from_file("fixtures/playlist/sample.pls").unwrap();
    assert_eq!(format, FileFormat::ShoutcastPlaylist);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_xml_shareable_playlist() {
    let format = FileFormat::from_file("fixtures/playlist/sample.xspf").unwrap();
    assert_eq!(format, FileFormat::XmlShareablePlaylistFormat);
}

#[test]
fn test_xml_shareable_playlist_no_xml() {
    let format = FileFormat::from_file("fixtures/playlist/sample-no-xml.xspf").unwrap();
    assert_eq!(format, FileFormat::XmlShareablePlaylistFormat);
}
