use file_format::FileFormat;

#[test]
#[cfg(feature = "from-media-type")]
fn test_atari7800_rom() {
    let fmt = FileFormat::from_media_type("application/x-atari-7800-rom");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Atari7800Rom)), "{:?} does not contain {}", fmt, FileFormat::Atari7800Rom);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_commodore64_cartridge() {
    let fmt = FileFormat::from_media_type("application/x-commodore-64-cartridge");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Commodore64Cartridge)), "{:?} does not contain {}", fmt, FileFormat::Commodore64Cartridge);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_game_boy_advance_rom() {
    let fmt = FileFormat::from_media_type("application/x-gba-rom");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::GameBoyAdvanceRom)), "{:?} does not contain {}", fmt, FileFormat::GameBoyAdvanceRom);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_game_boy_color_rom() {
    let fmt = FileFormat::from_media_type("application/x-gameboy-color-rom");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::GameBoyColorRom)), "{:?} does not contain {}", fmt, FileFormat::GameBoyColorRom);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_game_boy_rom() {
    let fmt = FileFormat::from_media_type("application/x-gameboy-rom");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::GameBoyRom)), "{:?} does not contain {}", fmt, FileFormat::GameBoyRom);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_game_gear_rom() {
    let fmt = FileFormat::from_media_type("application/x-gamegear-rom");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::GameGearRom)), "{:?} does not contain {}", fmt, FileFormat::GameGearRom);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_mega_drive_rom() {
    let fmt = FileFormat::from_media_type("application/x-genesis-rom");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MegaDriveRom)), "{:?} does not contain {}", fmt, FileFormat::MegaDriveRom);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_neo_geo_pocket_color_rom() {
    let fmt = FileFormat::from_media_type("application/x-neo-geo-pocket-rom");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::NeoGeoPocketColorRom)), "{:?} does not contain {}", fmt, FileFormat::NeoGeoPocketColorRom);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_neo_geo_pocket_rom() {
    let fmt = FileFormat::from_media_type("application/x-neo-geo-pocket-rom");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::NeoGeoPocketRom)), "{:?} does not contain {}", fmt, FileFormat::NeoGeoPocketRom);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_nintendo64_rom() {
    let fmt = FileFormat::from_media_type("application/x-n64-rom");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Nintendo64Rom)), "{:?} does not contain {}", fmt, FileFormat::Nintendo64Rom);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_nintendo_ds_rom() {
    let fmt = FileFormat::from_media_type("application/x-nintendo-ds-rom");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::NintendoDsRom)), "{:?} does not contain {}", fmt, FileFormat::NintendoDsRom);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_nintendo_entertainment_system_rom() {
    let fmt = FileFormat::from_media_type("application/x-nintendo-nes-rom");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::NintendoEntertainmentSystemRom)), "{:?} does not contain {}", fmt, FileFormat::NintendoEntertainmentSystemRom);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_nintendo_switch_rom() {
    let fmt = FileFormat::from_media_type("application/x-nintendo-switch-rom");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::NintendoSwitchRom)), "{:?} does not contain {}", fmt, FileFormat::NintendoSwitchRom);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_sega_master_system_rom() {
    let fmt = FileFormat::from_media_type("application/x-sms-rom");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SegaMasterSystemRom)), "{:?} does not contain {}", fmt, FileFormat::SegaMasterSystemRom);
}

