use file_format::FileFormat;

#[test]
#[cfg(feature = "from-media-type")]
fn test_advanced_stream_redirector(){
    let fmt = FileFormat::from_media_type("video/x-ms-asx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AdvancedStreamRedirector)), "{:?} does not contain {}", fmt, FileFormat::AdvancedStreamRedirector);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_mp3_url(){
    let fmt = FileFormat::from_media_type("audio/x-mpegurl");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Mp3Url)), "{:?} does not contain {}", fmt, FileFormat::Mp3Url);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_mpeg_dash_mpd(){
    let fmt = FileFormat::from_media_type("application/dash+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MpegDashMpd)), "{:?} does not contain {}", fmt, FileFormat::MpegDashMpd);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_shoutcast_playlist() {
    let fmt = FileFormat::from_media_type("audio/x-scpls");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ShoutcastPlaylist)), "{:?} does not contain {}", fmt, FileFormat::ShoutcastPlaylist);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_windows_media_playlist() {
    let fmt = FileFormat::from_media_type("application/vnd.ms-wpl");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WindowsMediaPlaylist)), "{:?} does not contain {}", fmt, FileFormat::WindowsMediaPlaylist);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_xml_shareable_playlist_format(){
    let fmt = FileFormat::from_media_type("application/xspf+xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::XmlShareablePlaylistFormat)), "{:?} does not contain {}", fmt, FileFormat::XmlShareablePlaylistFormat);
}

