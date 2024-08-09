use file_format::FileFormat;

#[test]
#[cfg(feature = "from-extension")]
fn test_ace(){
    let fmt = FileFormat::from_extension("ace");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Ace)), "{:?} does not contain {}", fmt, FileFormat::Ace);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_alz(){
    let fmt = FileFormat::from_extension("alz");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Alz)), "{:?} does not contain {}", fmt, FileFormat::Alz);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_archived_by_robert_jung(){
    let fmt = FileFormat::from_extension("arj");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ArchivedByRobertJung)), "{:?} does not contain {}", fmt, FileFormat::ArchivedByRobertJung);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_cabinet(){
    let fmt = FileFormat::from_extension("cab");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Cabinet)), "{:?} does not contain {}", fmt, FileFormat::Cabinet);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_cpio(){
    let fmt = FileFormat::from_extension("cpio");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Cpio)), "{:?} does not contain {}", fmt, FileFormat::Cpio);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_extensible_archive() {
    let fmt = FileFormat::from_extension("xar");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ExtensibleArchive)), "{:?} does not contain {}", fmt, FileFormat::ExtensibleArchive);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_larc(){
    let fmt = FileFormat::from_extension("lzs");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Larc)), "{:?} does not contain {}", fmt, FileFormat::Larc);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_lha(){
    let fmt = FileFormat::from_extension("lzh");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Lha)), "{:?} does not contain {}", fmt, FileFormat::Lha);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_mozilla_archive() {
    let fmt = FileFormat::from_extension("mar");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MozillaArchive)), "{:?} does not contain {}", fmt, FileFormat::MozillaArchive);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_multi_layer_archive() {
    let fmt = FileFormat::from_extension("mla");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MultiLayerArchive)), "{:?} does not contain {}", fmt, FileFormat::MultiLayerArchive);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_pmarc(){
    let fmt = FileFormat::from_extension("pma");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Pmarc)), "{:?} does not contain {}", fmt, FileFormat::Pmarc);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_roshal_archive() {
    let fmt = FileFormat::from_extension("rar");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::RoshalArchive)), "{:?} does not contain {}", fmt, FileFormat::RoshalArchive);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_seqbox(){
    let fmt = FileFormat::from_extension("sbx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Seqbox)), "{:?} does not contain {}", fmt, FileFormat::Seqbox);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_seven_zip() {
    let fmt = FileFormat::from_extension("7z");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SevenZip)), "{:?} does not contain {}", fmt, FileFormat::SevenZip);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_squashfs(){
    let fmt = FileFormat::from_extension("sqsh");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Squashfs)), "{:?} does not contain {}", fmt, FileFormat::Squashfs);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_stuffit(){
    let fmt = FileFormat::from_extension("sit");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Stuffit)), "{:?} does not contain {}", fmt, FileFormat::Stuffit);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_stuffit_x() {
    let fmt = FileFormat::from_extension("sitx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::StuffitX)), "{:?} does not contain {}", fmt, FileFormat::StuffitX);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_tape_archive() {
    let fmt = FileFormat::from_extension("tar");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::TapeArchive)), "{:?} does not contain {}", fmt, FileFormat::TapeArchive);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_unix_archiver() {
    let fmt = FileFormat::from_extension("a");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::UnixArchiver)), "{:?} does not contain {}", fmt, FileFormat::UnixArchiver);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_windows_imaging_format() {
    let fmt = FileFormat::from_extension("wim");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WindowsImagingFormat)), "{:?} does not contain {}", fmt, FileFormat::WindowsImagingFormat);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_zip(){
    let fmt = FileFormat::from_extension("zip");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Zip)), "{:?} does not contain {}", fmt, FileFormat::Zip);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_zoo(){
    let fmt = FileFormat::from_extension("zoo");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Zoo)), "{:?} does not contain {}", fmt, FileFormat::Zoo);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_zpaq(){
    let fmt = FileFormat::from_extension("zpaq");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Zpaq)), "{:?} does not contain {}", fmt, FileFormat::Zpaq);
}

