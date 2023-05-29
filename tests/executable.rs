use file_format::FileFormat;

#[test]
fn test_common_object_file_format() {
    let format = FileFormat::from_file("fixtures/executable/sample.coff").unwrap();
    assert_eq!(format, FileFormat::CommonObjectFileFormat);
}

#[test]
fn test_dalvik_executable() {
    let format = FileFormat::from_file("fixtures/executable/sample.dex").unwrap();
    assert_eq!(format, FileFormat::DalvikExecutable);
}

#[cfg(feature = "reader-exe")]
#[test]
fn test_dynamic_link_library() {
    let format = FileFormat::from_file("fixtures/executable/sample.dll").unwrap();
    assert_eq!(format, FileFormat::DynamicLinkLibrary);
}

#[test]
fn test_executable_and_linkable_format() {
    let format = FileFormat::from_file("fixtures/executable/sample.elf").unwrap();
    assert_eq!(format, FileFormat::ExecutableAndLinkableFormat);
}

#[test]
fn test_java_class() {
    let format = FileFormat::from_file("fixtures/executable/sample.class").unwrap();
    assert_eq!(format, FileFormat::JavaClass);
}
#[test]
fn test_llvm_bitcode() {
    let format = FileFormat::from_file("fixtures/executable/sample.bc").unwrap();
    assert_eq!(format, FileFormat::LlvmBitcode);
}

#[test]
fn test_lua_bytecode() {
    let format = FileFormat::from_file("fixtures/executable/sample.luac").unwrap();
    assert_eq!(format, FileFormat::LuaBytecode);
}

#[test]
fn test_mach_o() {
    let format = FileFormat::from_file("fixtures/executable/sample.mach").unwrap();
    assert_eq!(format, FileFormat::MachO);
}

#[test]
fn test_ms_dos_executable() {
    let format = FileFormat::from_file("fixtures/executable/sample.exe").unwrap();
    assert_eq!(format, FileFormat::MsDosExecutable);
}

#[test]
fn test_nintendo_switch_executable() {
    let format = FileFormat::from_file("fixtures/executable/sample.nso").unwrap();
    assert_eq!(format, FileFormat::NintendoSwitchExecutable);
}

#[test]
fn test_optimized_dalvik_executable() {
    let format = FileFormat::from_file("fixtures/executable/sample.dey").unwrap();
    assert_eq!(format, FileFormat::OptimizedDalvikExecutable);
}

#[cfg(feature = "reader-exe")]
#[test]
fn test_portable_executable() {
    let format = FileFormat::from_file("fixtures/executable/sample-pe.exe").unwrap();
    assert_eq!(format, FileFormat::PortableExecutable);
}

#[test]
fn test_webassembly_binary() {
    let format = FileFormat::from_file("fixtures/executable/sample.wasm").unwrap();
    assert_eq!(format, FileFormat::WebassemblyBinary);
}

#[test]
fn test_xbox_executable() {
    let format = FileFormat::from_file("fixtures/executable/sample.xbe").unwrap();
    assert_eq!(format, FileFormat::XboxExecutable);
}
