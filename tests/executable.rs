use file_format::FileFormat;

#[test]
fn test_commodore64_program() {
    let fmt = FileFormat::from_file("fixtures/rom/sample.prg").unwrap();
    assert_eq!(fmt, FileFormat::Commodore64Program);
}

#[test]
fn test_common_object_file_format() {
    let fmt = FileFormat::from_file("fixtures/executable/sample.coff").unwrap();
    assert_eq!(fmt, FileFormat::CommonObjectFileFormat);
}

#[test]
fn test_dalvik_executable() {
    let fmt = FileFormat::from_file("fixtures/executable/sample.dex").unwrap();
    assert_eq!(fmt, FileFormat::DalvikExecutable);
}

#[cfg(feature = "reader-exe")]
#[test]
fn test_dynamic_link_library() {
    let fmt = FileFormat::from_file("fixtures/executable/sample.dll").unwrap();
    assert_eq!(fmt, FileFormat::DynamicLinkLibrary);
}

#[test]
fn test_executable_and_linkable_format() {
    let fmt = FileFormat::from_file("fixtures/executable/sample.elf").unwrap();
    assert_eq!(fmt, FileFormat::ExecutableAndLinkableFormat);
}

#[test]
fn test_java_class() {
    let fmt = FileFormat::from_file("fixtures/executable/sample.class").unwrap();
    assert_eq!(fmt, FileFormat::JavaClass);
}

#[cfg(feature = "reader-exe")]
#[test]
fn test_linear_executable() {
    let fmt = FileFormat::from_file("fixtures/executable/sample1.exe").unwrap();
    assert_eq!(fmt, FileFormat::LinearExecutable);
}

#[test]
fn test_llvm_bitcode() {
    let fmt = FileFormat::from_file("fixtures/executable/sample.bc").unwrap();
    assert_eq!(fmt, FileFormat::LlvmBitcode);
}

#[test]
fn test_lua_bytecode() {
    let fmt = FileFormat::from_file("fixtures/executable/sample.luac").unwrap();
    assert_eq!(fmt, FileFormat::LuaBytecode);
}

#[test]
fn test_mach_o() {
    let fmt = FileFormat::from_file("fixtures/executable/sample.mach").unwrap();
    assert_eq!(fmt, FileFormat::MachO);
}

#[test]
fn test_ms_dos_executable() {
    let fmt = FileFormat::from_file("fixtures/executable/sample2.exe").unwrap();
    assert_eq!(fmt, FileFormat::MsDosExecutable);
}

#[cfg(feature = "reader-exe")]
#[test]
fn test_new_executable() {
    let fmt = FileFormat::from_file("fixtures/executable/sample3.exe").unwrap();
    assert_eq!(fmt, FileFormat::NewExecutable);
}

#[test]
fn test_nintendo_switch_executable() {
    let fmt = FileFormat::from_file("fixtures/executable/sample.nso").unwrap();
    assert_eq!(fmt, FileFormat::NintendoSwitchExecutable);
}

#[test]
fn test_optimized_dalvik_executable() {
    let fmt = FileFormat::from_file("fixtures/executable/sample.dey").unwrap();
    assert_eq!(fmt, FileFormat::OptimizedDalvikExecutable);
}

#[cfg(feature = "reader-exe")]
#[test]
fn test_portable_executable() {
    let fmt = FileFormat::from_file("fixtures/executable/sample4.exe").unwrap();
    assert_eq!(fmt, FileFormat::PortableExecutable);
}

#[test]
fn test_webassembly_binary() {
    let fmt = FileFormat::from_file("fixtures/executable/sample.wasm").unwrap();
    assert_eq!(fmt, FileFormat::WebassemblyBinary);
}

#[test]
fn test_xbox360_executable() {
    let fmt = FileFormat::from_file("fixtures/executable/sample.xex").unwrap();
    assert_eq!(fmt, FileFormat::Xbox360Executable);
}

#[test]
fn test_xbox_executable() {
    let fmt = FileFormat::from_file("fixtures/executable/sample.xbe").unwrap();
    assert_eq!(fmt, FileFormat::XboxExecutable);
}
