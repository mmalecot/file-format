use file_format::FileFormat;

#[test]
fn test_clojure_script() {
    let fmt = FileFormat::from_file("fixtures/text/sample.clj").unwrap();
    assert_eq!(fmt, FileFormat::ClojureScript);
}

#[test]
fn test_extensible_markup_language() {
    let fmt = FileFormat::from_file("fixtures/text/sample.xml").unwrap();
    assert_eq!(fmt, FileFormat::ExtensibleMarkupLanguage);
}

#[test]
fn test_hypertext_markup_language() {
    let fmt = FileFormat::from_file("fixtures/text/sample.html").unwrap();
    assert_eq!(fmt, FileFormat::HypertextMarkupLanguage);
}

#[test]
fn test_icalendar() {
    let fmt = FileFormat::from_file("fixtures/text/sample.ics").unwrap();
    assert_eq!(fmt, FileFormat::Icalendar);
}

#[test]
fn test_latex() {
    let fmt = FileFormat::from_file("fixtures/text/sample.tex").unwrap();
    assert_eq!(fmt, FileFormat::Latex);
}

#[test]
fn test_lua_script() {
    let fmt = FileFormat::from_file("fixtures/text/sample.lua").unwrap();
    assert_eq!(fmt, FileFormat::LuaScript);
}

#[test]
fn test_ms_dos_batch() {
    let fmt = FileFormat::from_file("fixtures/text/sample.bat").unwrap();
    assert_eq!(fmt, FileFormat::MsDosBatch);
}

#[test]
fn test_perl_script() {
    let fmt = FileFormat::from_file("fixtures/text/sample.pl").unwrap();
    assert_eq!(fmt, FileFormat::PerlScript);
}

#[cfg(feature = "reader-txt")]
#[test]
fn test_plain_text() {
    let fmt = FileFormat::from_file("fixtures/text/sample.txt").unwrap();
    assert_eq!(fmt, FileFormat::PlainText);
}

#[test]
fn test_python_script() {
    let fmt = FileFormat::from_file("fixtures/text/sample.py").unwrap();
    assert_eq!(fmt, FileFormat::PythonScript);
}

#[test]
fn test_ruby_script() {
    let fmt = FileFormat::from_file("fixtures/text/sample.rb").unwrap();
    assert_eq!(fmt, FileFormat::RubyScript);
}

#[test]
fn test_shell_script() {
    let fmt = FileFormat::from_file("fixtures/text/sample.sh").unwrap();
    assert_eq!(fmt, FileFormat::ShellScript);
}

#[test]
fn test_tool_command_language_script() {
    let fmt = FileFormat::from_file("fixtures/text/sample.tcl").unwrap();
    assert_eq!(fmt, FileFormat::ToolCommandLanguageScript);
}

#[test]
fn test_vcalendar() {
    let fmt = FileFormat::from_file("fixtures/text/sample.vcs").unwrap();
    assert_eq!(fmt, FileFormat::Vcalendar);
}

#[test]
fn test_vcard() {
    let fmt = FileFormat::from_file("fixtures/text/sample.vcf").unwrap();
    assert_eq!(fmt, FileFormat::Vcard);
}

#[test]
fn test_webassembly_text() {
    let fmt = FileFormat::from_file("fixtures/text/sample.wat").unwrap();
    assert_eq!(fmt, FileFormat::WebassemblyText);
}
