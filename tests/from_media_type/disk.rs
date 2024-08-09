use file_format::FileFormat;

#[test]
#[cfg(feature = "from-media-type")]
fn test_amiga_disk_file() {
    let fmt = FileFormat::from_media_type("application/x-amiga-disk-format");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AmigaDiskFile)), "{:?} does not contain {}", fmt, FileFormat::AmigaDiskFile);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_apple_disk_image() {
    let fmt = FileFormat::from_media_type("application/x-apple-diskimage");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AppleDiskImage)), "{:?} does not contain {}", fmt, FileFormat::AppleDiskImage);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_iso9660(){
    let fmt = FileFormat::from_media_type("application/x-iso9660-image");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Iso9660)), "{:?} does not contain {}", fmt, FileFormat::Iso9660);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_microsoft_virtual_hard_disk() {
    let fmt = FileFormat::from_media_type("application/x-vhd");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftVirtualHardDisk)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftVirtualHardDisk);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_microsoft_virtual_hard_disk2() {
    let fmt = FileFormat::from_media_type("application/x-vhdx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftVirtualHardDisk2)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftVirtualHardDisk2);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_qemu_copy_on_write() {
    let fmt = FileFormat::from_media_type("application/x-qemu-disk");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::QemuCopyOnWrite)), "{:?} does not contain {}", fmt, FileFormat::QemuCopyOnWrite);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_virtual_machine_disk() {
    let fmt = FileFormat::from_media_type("application/x-vmdk");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::VirtualMachineDisk)), "{:?} does not contain {}", fmt, FileFormat::VirtualMachineDisk);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_virtualbox_virtual_disk_image() {
    let fmt = FileFormat::from_media_type("application/x-virtualbox-vdi");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::VirtualboxVirtualDiskImage)), "{:?} does not contain {}", fmt, FileFormat::VirtualboxVirtualDiskImage);
}

