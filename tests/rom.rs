use file_format::FileFormat;

#[test]
fn test_atari7800_rom() {
    let fmt = FileFormat::from_file("fixtures/rom/sample.a78").unwrap();
    assert_eq!(fmt, FileFormat::Atari7800Rom);
}

#[test]
fn test_game_boy_advance_rom() {
    let fmt = FileFormat::from_file("fixtures/rom/sample.gba").unwrap();
    assert_eq!(fmt, FileFormat::GameBoyAdvanceRom);
}

#[test]
fn test_game_boy_color_rom() {
    let fmt = FileFormat::from_file("fixtures/rom/sample.gbc").unwrap();
    assert_eq!(fmt, FileFormat::GameBoyColorRom);
}

#[test]
fn test_game_boy_rom() {
    let fmt = FileFormat::from_file("fixtures/rom/sample.gb").unwrap();
    assert_eq!(fmt, FileFormat::GameBoyRom);
}

#[test]
fn test_game_gear_rom() {
    let fmt = FileFormat::from_file("fixtures/rom/sample.gg").unwrap();
    assert_eq!(fmt, FileFormat::GameGearRom);
}

#[test]
fn test_mega_drive_rom() {
    let fmt = FileFormat::from_file("fixtures/rom/sample.md").unwrap();
    assert_eq!(fmt, FileFormat::MegaDriveRom);
}

#[test]
fn test_neo_geo_pocket_color_rom() {
    let fmt = FileFormat::from_file("fixtures/rom/sample.ngc").unwrap();
    assert_eq!(fmt, FileFormat::NeoGeoPocketColorRom);
}

#[test]
fn test_neo_geo_pocket_rom() {
    let fmt = FileFormat::from_file("fixtures/rom/sample.ngp").unwrap();
    assert_eq!(fmt, FileFormat::NeoGeoPocketRom);
}

#[test]
fn test_nintendo64_rom() {
    let fmt = FileFormat::from_file("fixtures/rom/sample.z64").unwrap();
    assert_eq!(fmt, FileFormat::Nintendo64Rom);
}

#[test]
fn test_nintendo_ds_rom() {
    let fmt = FileFormat::from_file("fixtures/rom/sample.nds").unwrap();
    assert_eq!(fmt, FileFormat::NintendoDsRom);
}

#[test]
fn test_nintendo_entertainment_system_rom() {
    let fmt = FileFormat::from_file("fixtures/rom/sample.nes").unwrap();
    assert_eq!(fmt, FileFormat::NintendoEntertainmentSystemRom);
}

#[test]
fn test_nintendo_switch_rom() {
    let fmt = FileFormat::from_file("fixtures/rom/sample.xci").unwrap();
    assert_eq!(fmt, FileFormat::NintendoSwitchRom);
}

#[test]
fn test_sega_master_system_rom() {
    let fmt = FileFormat::from_file("fixtures/rom/sample.sms").unwrap();
    assert_eq!(fmt, FileFormat::SegaMasterSystemRom);
}
