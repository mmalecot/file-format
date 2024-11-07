use file_format::FileFormat;

#[test]
#[cfg(feature = "from-extension")]
fn test_android_binary_xml() {
    let fmt = FileFormat::from_extension("xml");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AndroidBinaryXml)), "{:?} does not contain {}", fmt, FileFormat::AndroidBinaryXml);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_bittorrent(){
    let fmt = FileFormat::from_extension("torrent");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Bittorrent)), "{:?} does not contain {}", fmt, FileFormat::Bittorrent);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_cd_audio() {
    let fmt = FileFormat::from_extension("cda");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::CdAudio)), "{:?} does not contain {}", fmt, FileFormat::CdAudio);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_macos_alias() {
    let fmt = FileFormat::from_extension("alias");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MacosAlias)), "{:?} does not contain {}", fmt, FileFormat::MacosAlias);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_meta_information_encapsulation() {
    let fmt = FileFormat::from_extension("mie");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MetaInformationEncapsulation)), "{:?} does not contain {}", fmt, FileFormat::MetaInformationEncapsulation);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_tasty(){
    let fmt = FileFormat::from_extension("tasty");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Tasty)), "{:?} does not contain {}", fmt, FileFormat::Tasty);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_windows_shortcut() {
    let fmt = FileFormat::from_extension("lnk");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WindowsShortcut)), "{:?} does not contain {}", fmt, FileFormat::WindowsShortcut);
}

