use file_format::FileFormat;

#[test]
fn test_clojure_script() {
    let format = FileFormat::from_file("fixtures/text/sample.clj").unwrap();
    assert_eq!(format, FileFormat::ClojureScript);
}

#[test]
fn test_extensible_markup_language() {
    let format = FileFormat::from_file("fixtures/text/sample.xml").unwrap();
    assert_eq!(format, FileFormat::ExtensibleMarkupLanguage);
}

#[test]
fn test_hypertext_markup_language() {
    let format = FileFormat::from_file("fixtures/text/sample.html").unwrap();
    assert_eq!(format, FileFormat::HypertextMarkupLanguage);
}

#[test]
fn test_icalendar() {
    let format = FileFormat::from_file("fixtures/text/sample.ics").unwrap();
    assert_eq!(format, FileFormat::Icalendar);
}

#[test]
fn test_latex() {
    let format = FileFormat::from_file("fixtures/text/sample.tex").unwrap();
    assert_eq!(format, FileFormat::Latex);
}

#[test]
fn test_lua_script() {
    let format = FileFormat::from_file("fixtures/text/sample.lua").unwrap();
    assert_eq!(format, FileFormat::LuaScript);
}

#[test]
fn test_perl_script() {
    let format = FileFormat::from_file("fixtures/text/sample.pl").unwrap();
    assert_eq!(format, FileFormat::PerlScript);
}

#[cfg(feature = "reader-txt")]
#[test]
fn test_plain_text() {
    let format = FileFormat::from_file("fixtures/text/sample.txt").unwrap();
    assert_eq!(format, FileFormat::PlainText);
}

#[test]
fn test_python_script() {
    let format = FileFormat::from_file("fixtures/text/sample.py").unwrap();
    assert_eq!(format, FileFormat::PythonScript);
}

#[test]
fn test_ruby_script() {
    let format = FileFormat::from_file("fixtures/text/sample.rb").unwrap();
    assert_eq!(format, FileFormat::RubyScript);
}

#[test]
fn test_shell_script() {
    let format = FileFormat::from_file("fixtures/text/sample.sh").unwrap();
    assert_eq!(format, FileFormat::ShellScript);
}

#[test]
fn test_tool_command_language_script() {
    let format = FileFormat::from_file("fixtures/text/sample.tcl").unwrap();
    assert_eq!(format, FileFormat::ToolCommandLanguageScript);
}

#[test]
fn test_vcalendar() {
    let format = FileFormat::from_file("fixtures/text/sample.vcs").unwrap();
    assert_eq!(format, FileFormat::Vcalendar);
}

#[test]
fn test_vcard() {
    let format = FileFormat::from_file("fixtures/text/sample.vcf").unwrap();
    assert_eq!(format, FileFormat::Vcard);
}
