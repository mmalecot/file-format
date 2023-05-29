use file_format::FileFormat;

#[test]
fn test_apple_disk_image() {
    let format = FileFormat::from_file("fixtures/disk/sample.dmg").unwrap();
    assert_eq!(format, FileFormat::AppleDiskImage);
}

#[test]
fn test_iso9660() {
    let format = FileFormat::from_file("fixtures/disk/sample.iso").unwrap();
    assert_eq!(format, FileFormat::Iso9660);
}

#[test]
fn test_microsoft_virtual_hard_disk() {
    let format = FileFormat::from_file("fixtures/disk/sample.vhd").unwrap();
    assert_eq!(format, FileFormat::MicrosoftVirtualHardDisk);
}

#[test]
fn test_microsoft_virtual_hard_disk2() {
    let format = FileFormat::from_file("fixtures/disk/sample.vhdx").unwrap();
    assert_eq!(format, FileFormat::MicrosoftVirtualHardDisk2);
}

#[test]
fn test_virtualbox_virtual_disk_image() {
    let format = FileFormat::from_file("fixtures/disk/sample.vdi").unwrap();
    assert_eq!(format, FileFormat::VirtualboxVirtualDiskImage);
}
