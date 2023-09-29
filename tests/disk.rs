use file_format::FileFormat;

#[test]
fn test_amiga_disk_file() {
    let fmt = FileFormat::from_file("fixtures/disk/sample.adf").unwrap();
    assert_eq!(fmt, FileFormat::AmigaDiskFile);
}

#[test]
fn test_apple_disk_image() {
    let fmt = FileFormat::from_file("fixtures/disk/sample.dmg").unwrap();
    assert_eq!(fmt, FileFormat::AppleDiskImage);
}

#[test]
fn test_iso9660() {
    let fmt = FileFormat::from_file("fixtures/disk/sample.iso").unwrap();
    assert_eq!(fmt, FileFormat::Iso9660);
}

#[test]
fn test_microsoft_virtual_hard_disk() {
    let fmt = FileFormat::from_file("fixtures/disk/sample.vhd").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftVirtualHardDisk);
}

#[test]
fn test_microsoft_virtual_hard_disk2() {
    let fmt = FileFormat::from_file("fixtures/disk/sample.vhdx").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftVirtualHardDisk2);
}

#[test]
fn test_qemu_copy_on_write() {
    let fmt = FileFormat::from_file("fixtures/disk/sample.qcow").unwrap();
    assert_eq!(fmt, FileFormat::QemuCopyOnWrite);
}

#[test]
fn test_virtual_machine_disk() {
    let fmt = FileFormat::from_file("fixtures/disk/sample.vmdk").unwrap();
    assert_eq!(fmt, FileFormat::VirtualMachineDisk);
}

#[test]
fn test_virtualbox_virtual_disk_image() {
    let fmt = FileFormat::from_file("fixtures/disk/sample.vdi").unwrap();
    assert_eq!(fmt, FileFormat::VirtualboxVirtualDiskImage);
}
