use file_format::FileFormat;

#[cfg(feature = "reader-zip")]
#[test]
fn test_android_package() {
    let fmt = FileFormat::from_file("fixtures/package/sample.apk").unwrap();
    assert_eq!(fmt, FileFormat::AndroidPackage);
}

#[test]
fn test_debian_binary_package() {
    let fmt = FileFormat::from_file("fixtures/package/sample.deb").unwrap();
    assert_eq!(fmt, FileFormat::DebianBinaryPackage);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_enterprise_application_archive() {
    let fmt = FileFormat::from_file("fixtures/package/sample.ear").unwrap();
    assert_eq!(fmt, FileFormat::EnterpriseApplicationArchive);
}

#[test]
fn test_google_chrome_extension() {
    let fmt = FileFormat::from_file("fixtures/package/sample.crx").unwrap();
    assert_eq!(fmt, FileFormat::GoogleChromeExtension);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_ios_app_store_package() {
    let fmt = FileFormat::from_file("fixtures/package/sample.ipa").unwrap();
    assert_eq!(fmt, FileFormat::IosAppStorePackage);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_java_archive() {
    let fmt = FileFormat::from_file("fixtures/package/sample.jar").unwrap();
    assert_eq!(fmt, FileFormat::JavaArchive);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_software_installer() {
    let fmt = FileFormat::from_file("fixtures/package/sample.msi").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftSoftwareInstaller);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_microsoft_visual_studio_extension() {
    let fmt = FileFormat::from_file("fixtures/package/sample.vsix").unwrap();
    assert_eq!(fmt, FileFormat::MicrosoftVisualStudioExtension);
}

#[test]
fn test_nintendo_switch_package() {
    let fmt = FileFormat::from_file("fixtures/package/sample.nsp").unwrap();
    assert_eq!(fmt, FileFormat::NintendoSwitchPackage);
}

#[test]
fn test_red_hat_package_manager() {
    let fmt = FileFormat::from_file("fixtures/package/sample.rpm").unwrap();
    assert_eq!(fmt, FileFormat::RedHatPackageManager);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_web_application_archive() {
    let fmt = FileFormat::from_file("fixtures/package/sample.war").unwrap();
    assert_eq!(fmt, FileFormat::WebApplicationArchive);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_windows_app_package() {
    let fmt = FileFormat::from_file("fixtures/package/sample.appx").unwrap();
    assert_eq!(fmt, FileFormat::WindowsAppPackage);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_xap() {
    let fmt = FileFormat::from_file("fixtures/package/sample.xap").unwrap();
    assert_eq!(fmt, FileFormat::Xap);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_xpinstall() {
    let fmt = FileFormat::from_file("fixtures/package/sample.xpi").unwrap();
    assert_eq!(fmt, FileFormat::Xpinstall);
}
