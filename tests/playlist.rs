use file_format::FileFormat;

#[test]
fn test_advanced_stream_redirector_1() {
    let fmt = FileFormat::from_file("fixtures/playlist/sample1.asx").unwrap();
    assert_eq!(fmt, FileFormat::AdvancedStreamRedirector);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_advanced_stream_redirector_2() {
    let fmt = FileFormat::from_file("fixtures/playlist/sample2.asx").unwrap();
    assert_eq!(fmt, FileFormat::AdvancedStreamRedirector);
}

#[test]
fn test_mpeg_dash_manifest_1() {
    let fmt = FileFormat::from_file("fixtures/playlist/sample1.mpd").unwrap();
    assert_eq!(fmt, FileFormat::MpegDashManifest);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_mpeg_dash_manifest_2() {
    let fmt = FileFormat::from_file("fixtures/playlist/sample2.mpd").unwrap();
    assert_eq!(fmt, FileFormat::MpegDashManifest);
}

#[test]
fn test_mp3_url() {
    let fmt = FileFormat::from_file("fixtures/playlist/sample.m3u").unwrap();
    assert_eq!(fmt, FileFormat::Mp3Url);
}

#[test]
fn test_shoutcast_playlist() {
    let fmt = FileFormat::from_file("fixtures/playlist/sample.pls").unwrap();
    assert_eq!(fmt, FileFormat::ShoutcastPlaylist);
}

#[test]
fn test_windows_media_playlist() {
    let fmt = FileFormat::from_file("fixtures/playlist/sample.wpl").unwrap();
    assert_eq!(fmt, FileFormat::WindowsMediaPlaylist);
}

#[test]
fn test_xml_shareable_playlist_1() {
    let fmt = FileFormat::from_file("fixtures/playlist/sample1.xspf").unwrap();
    assert_eq!(fmt, FileFormat::XmlShareablePlaylistFormat);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_xml_shareable_playlist_2() {
    let fmt = FileFormat::from_file("fixtures/playlist/sample2.xspf").unwrap();
    assert_eq!(fmt, FileFormat::XmlShareablePlaylistFormat);
}
