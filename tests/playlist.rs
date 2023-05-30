use file_format::FileFormat;

#[test]
fn test_advanced_stream_redirector_1() {
    let format = FileFormat::from_file("fixtures/playlist/sample1.asx").unwrap();
    assert_eq!(format, FileFormat::AdvancedStreamRedirector);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_advanced_stream_redirector_2() {
    let format = FileFormat::from_file("fixtures/playlist/sample2.asx").unwrap();
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

#[test]
fn test_xml_shareable_playlist_1() {
    let format = FileFormat::from_file("fixtures/playlist/sample1.xspf").unwrap();
    assert_eq!(format, FileFormat::XmlShareablePlaylistFormat);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_xml_shareable_playlist_2() {
    let format = FileFormat::from_file("fixtures/playlist/sample2.xspf").unwrap();
    assert_eq!(format, FileFormat::XmlShareablePlaylistFormat);
}
