use file_format::FileFormat;

#[test]
#[cfg(feature = "from-media-type")]
fn test_commodore64_program() {
    let fmt = FileFormat::from_media_type("application/x-commodore-64-program");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Commodore64Program)), "{:?} does not contain {}", fmt, FileFormat::Commodore64Program);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_common_object_file_format() {
    let fmt = FileFormat::from_media_type("application/x-coff");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::CommonObjectFileFormat)), "{:?} does not contain {}", fmt, FileFormat::CommonObjectFileFormat);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_dalvik_executable() {
    let fmt = FileFormat::from_media_type("application/vnd.android.dex");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::DalvikExecutable)), "{:?} does not contain {}", fmt, FileFormat::DalvikExecutable);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_dynamic_link_library() {
    let fmt = FileFormat::from_media_type("application/vnd.microsoft.portable-executable");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::DynamicLinkLibrary)), "{:?} does not contain {}", fmt, FileFormat::DynamicLinkLibrary);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_executable_and_linkable_format() {
    let fmt = FileFormat::from_media_type("application/x-executable");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ExecutableAndLinkableFormat)), "{:?} does not contain {}", fmt, FileFormat::ExecutableAndLinkableFormat);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_java_class() {
    let fmt = FileFormat::from_media_type("application/java-vm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::JavaClass)), "{:?} does not contain {}", fmt, FileFormat::JavaClass);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_linear_executable() {
    let fmt = FileFormat::from_media_type("application/x-dosexec");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::LinearExecutable)), "{:?} does not contain {}", fmt, FileFormat::LinearExecutable);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_llvm_bitcode() {
    let fmt = FileFormat::from_media_type("application/x-llvm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::LlvmBitcode)), "{:?} does not contain {}", fmt, FileFormat::LlvmBitcode);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_lua_bytecode() {
    let fmt = FileFormat::from_media_type("application/x-lua-bytecode");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::LuaBytecode)), "{:?} does not contain {}", fmt, FileFormat::LuaBytecode);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_mach_o() {
    let fmt = FileFormat::from_media_type("application/x-mach-binary");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MachO)), "{:?} does not contain {}", fmt, FileFormat::MachO);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_ms_dos_executable() {
    let fmt = FileFormat::from_media_type("application/x-dosexec");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MsDosExecutable)), "{:?} does not contain {}", fmt, FileFormat::MsDosExecutable);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_new_executable() {
    let fmt = FileFormat::from_media_type("application/x-ms-ne-executable");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::NewExecutable)), "{:?} does not contain {}", fmt, FileFormat::NewExecutable);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_nintendo_switch_executable() {
    let fmt = FileFormat::from_media_type("application/x-nintendo-switch-executable");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::NintendoSwitchExecutable)), "{:?} does not contain {}", fmt, FileFormat::NintendoSwitchExecutable);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_optimized_dalvik_executable() {
    let fmt = FileFormat::from_media_type("application/vnd.android.dey");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OptimizedDalvikExecutable)), "{:?} does not contain {}", fmt, FileFormat::OptimizedDalvikExecutable);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_portable_executable() {
    let fmt = FileFormat::from_media_type("application/vnd.microsoft.portable-executable");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PortableExecutable)), "{:?} does not contain {}", fmt, FileFormat::PortableExecutable);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_webassembly_binary() {
    let fmt = FileFormat::from_media_type("application/wasm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WebassemblyBinary)), "{:?} does not contain {}", fmt, FileFormat::WebassemblyBinary);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_xbox360_executable() {
    let fmt = FileFormat::from_media_type("application/x-xbox360-executable");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Xbox360Executable)), "{:?} does not contain {}", fmt, FileFormat::Xbox360Executable);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_xbox_executable() {
    let fmt = FileFormat::from_media_type("application/x-xbox-executable");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::XboxExecutable)), "{:?} does not contain {}", fmt, FileFormat::XboxExecutable);
}

