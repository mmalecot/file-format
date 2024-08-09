use file_format::FileFormat;

#[test]
#[cfg(feature = "from-extension")]
fn test_commodore64_program() {
    let fmt = FileFormat::from_extension("prg");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Commodore64Program)), "{:?} does not contain {}", fmt, FileFormat::Commodore64Program);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_common_object_file_format() {
    let fmt = FileFormat::from_extension("coff");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::CommonObjectFileFormat)), "{:?} does not contain {}", fmt, FileFormat::CommonObjectFileFormat);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_dalvik_executable() {
    let fmt = FileFormat::from_extension("dex");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::DalvikExecutable)), "{:?} does not contain {}", fmt, FileFormat::DalvikExecutable);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_dynamic_link_library() {
    let fmt = FileFormat::from_extension("dll");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::DynamicLinkLibrary)), "{:?} does not contain {}", fmt, FileFormat::DynamicLinkLibrary);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_executable_and_linkable_format() {
    let fmt = FileFormat::from_extension("elf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ExecutableAndLinkableFormat)), "{:?} does not contain {}", fmt, FileFormat::ExecutableAndLinkableFormat);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_java_class() {
    let fmt = FileFormat::from_extension("class");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::JavaClass)), "{:?} does not contain {}", fmt, FileFormat::JavaClass);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_linear_executable() {
    let fmt = FileFormat::from_extension("exe");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::LinearExecutable)), "{:?} does not contain {}", fmt, FileFormat::LinearExecutable);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_llvm_bitcode() {
    let fmt = FileFormat::from_extension("bc");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::LlvmBitcode)), "{:?} does not contain {}", fmt, FileFormat::LlvmBitcode);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_lua_bytecode() {
    let fmt = FileFormat::from_extension("luac");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::LuaBytecode)), "{:?} does not contain {}", fmt, FileFormat::LuaBytecode);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_mach_o() {
    let fmt = FileFormat::from_extension("mach");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MachO)), "{:?} does not contain {}", fmt, FileFormat::MachO);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_ms_dos_executable() {
    let fmt = FileFormat::from_extension("exe");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MsDosExecutable)), "{:?} does not contain {}", fmt, FileFormat::MsDosExecutable);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_new_executable() {
    let fmt = FileFormat::from_extension("exe");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::NewExecutable)), "{:?} does not contain {}", fmt, FileFormat::NewExecutable);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_nintendo_switch_executable() {
    let fmt = FileFormat::from_extension("nso");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::NintendoSwitchExecutable)), "{:?} does not contain {}", fmt, FileFormat::NintendoSwitchExecutable);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_optimized_dalvik_executable() {
    let fmt = FileFormat::from_extension("dey");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OptimizedDalvikExecutable)), "{:?} does not contain {}", fmt, FileFormat::OptimizedDalvikExecutable);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_portable_executable() {
    let fmt = FileFormat::from_extension("exe");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::PortableExecutable)), "{:?} does not contain {}", fmt, FileFormat::PortableExecutable);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_webassembly_binary() {
    let fmt = FileFormat::from_extension("wasm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WebassemblyBinary)), "{:?} does not contain {}", fmt, FileFormat::WebassemblyBinary);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_xbox360_executable() {
    let fmt = FileFormat::from_extension("xex");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Xbox360Executable)), "{:?} does not contain {}", fmt, FileFormat::Xbox360Executable);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_xbox_executable() {
    let fmt = FileFormat::from_extension("xbe");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::XboxExecutable)), "{:?} does not contain {}", fmt, FileFormat::XboxExecutable);
}

