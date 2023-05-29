use file_format::FileFormat;

#[cfg(feature = "reader-zip")]
#[test]
fn test_android_package() {
    let format = FileFormat::from_file("fixtures/package/sample.apk").unwrap();
    assert_eq!(format, FileFormat::AndroidPackage);
}

#[test]
fn test_debian_binary_package() {
    let format = FileFormat::from_file("fixtures/package/sample.deb").unwrap();
    assert_eq!(format, FileFormat::DebianBinaryPackage);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_enterprise_application_archive() {
    let format = FileFormat::from_file("fixtures/package/sample.ear").unwrap();
    assert_eq!(format, FileFormat::EnterpriseApplicationArchive);
}

#[test]
fn test_google_chrome_extension() {
    let format = FileFormat::from_file("fixtures/package/sample.crx").unwrap();
    assert_eq!(format, FileFormat::GoogleChromeExtension);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_ios_app_store_package() {
    let format = FileFormat::from_file("fixtures/package/sample.ipa").unwrap();
    assert_eq!(format, FileFormat::IosAppStorePackage);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_java_archive() {
    let format = FileFormat::from_file("fixtures/package/sample.jar").unwrap();
    assert_eq!(format, FileFormat::JavaArchive);
}

#[cfg(feature = "reader-cfb")]
#[test]
fn test_microsoft_software_installer() {
    let format = FileFormat::from_file("fixtures/package/sample.msi").unwrap();
    assert_eq!(format, FileFormat::MicrosoftSoftwareInstaller);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_microsoft_visual_studio_extension() {
    let format = FileFormat::from_file("fixtures/package/sample.vsix").unwrap();
    assert_eq!(format, FileFormat::MicrosoftVisualStudioExtension);
}

#[test]
fn test_nintendo_switch_package() {
    let format = FileFormat::from_file("fixtures/package/sample.nsp").unwrap();
    assert_eq!(format, FileFormat::NintendoSwitchPackage);
}

#[test]
fn test_red_hat_package_manager() {
    let format = FileFormat::from_file("fixtures/package/sample.rpm").unwrap();
    assert_eq!(format, FileFormat::RedHatPackageManager);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_web_application_archive() {
    let format = FileFormat::from_file("fixtures/package/sample.war").unwrap();
    assert_eq!(format, FileFormat::WebApplicationArchive);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_windows_app_package() {
    let format = FileFormat::from_file("fixtures/package/sample.appx").unwrap();
    assert_eq!(format, FileFormat::WindowsAppPackage);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_xap() {
    let format = FileFormat::from_file("fixtures/package/sample.xap").unwrap();
    assert_eq!(format, FileFormat::Xap);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_xpinstall() {
    let format = FileFormat::from_file("fixtures/package/sample.xpi").unwrap();
    assert_eq!(format, FileFormat::Xpinstall);
}
