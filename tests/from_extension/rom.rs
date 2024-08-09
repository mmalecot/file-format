use file_format::FileFormat;

#[test]
#[cfg(feature = "from-extension")]
fn test_atari7800_rom() {
    let fmt = FileFormat::from_extension("a78");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Atari7800Rom)), "{:?} does not contain {}", fmt, FileFormat::Atari7800Rom);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_commodore64_cartridge() {
    let fmt = FileFormat::from_extension("crt");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Commodore64Cartridge)), "{:?} does not contain {}", fmt, FileFormat::Commodore64Cartridge);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_game_boy_advance_rom() {
    let fmt = FileFormat::from_extension("gba");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::GameBoyAdvanceRom)), "{:?} does not contain {}", fmt, FileFormat::GameBoyAdvanceRom);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_game_boy_color_rom() {
    let fmt = FileFormat::from_extension("gbc");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::GameBoyColorRom)), "{:?} does not contain {}", fmt, FileFormat::GameBoyColorRom);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_game_boy_rom() {
    let fmt = FileFormat::from_extension("gb");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::GameBoyRom)), "{:?} does not contain {}", fmt, FileFormat::GameBoyRom);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_game_gear_rom() {
    let fmt = FileFormat::from_extension("gg");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::GameGearRom)), "{:?} does not contain {}", fmt, FileFormat::GameGearRom);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_mega_drive_rom() {
    let fmt = FileFormat::from_extension("md");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MegaDriveRom)), "{:?} does not contain {}", fmt, FileFormat::MegaDriveRom);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_neo_geo_pocket_color_rom() {
    let fmt = FileFormat::from_extension("ngc");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::NeoGeoPocketColorRom)), "{:?} does not contain {}", fmt, FileFormat::NeoGeoPocketColorRom);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_neo_geo_pocket_rom() {
    let fmt = FileFormat::from_extension("ngp");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::NeoGeoPocketRom)), "{:?} does not contain {}", fmt, FileFormat::NeoGeoPocketRom);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_nintendo64_rom() {
    let fmt = FileFormat::from_extension("z64");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Nintendo64Rom)), "{:?} does not contain {}", fmt, FileFormat::Nintendo64Rom);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_nintendo_ds_rom() {
    let fmt = FileFormat::from_extension("nds");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::NintendoDsRom)), "{:?} does not contain {}", fmt, FileFormat::NintendoDsRom);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_nintendo_entertainment_system_rom() {
    let fmt = FileFormat::from_extension("nes");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::NintendoEntertainmentSystemRom)), "{:?} does not contain {}", fmt, FileFormat::NintendoEntertainmentSystemRom);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_nintendo_switch_rom() {
    let fmt = FileFormat::from_extension("xci");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::NintendoSwitchRom)), "{:?} does not contain {}", fmt, FileFormat::NintendoSwitchRom);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_sega_master_system_rom() {
    let fmt = FileFormat::from_extension("sms");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SegaMasterSystemRom)), "{:?} does not contain {}", fmt, FileFormat::SegaMasterSystemRom);
}

