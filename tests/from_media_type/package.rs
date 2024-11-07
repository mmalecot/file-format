use file_format::FileFormat;

#[test]
#[cfg(feature = "from-media-type")]
fn test_adobe_integrated_runtime() {
    let fmt = FileFormat::from_media_type("application/vnd.adobe.air-application-installer-package+zip");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AdobeIntegratedRuntime)), "{:?} does not contain {}", fmt, FileFormat::AdobeIntegratedRuntime);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_android_app_bundle() {
    let fmt = FileFormat::from_media_type("application/vnd.android.aab");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AndroidAppBundle)), "{:?} does not contain {}", fmt, FileFormat::AndroidAppBundle);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_android_package() {
    let fmt = FileFormat::from_media_type("application/vnd.android.package-archive");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AndroidPackage)), "{:?} does not contain {}", fmt, FileFormat::AndroidPackage);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_appimage(){
    let fmt = FileFormat::from_media_type("application/x-appimage");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Appimage)), "{:?} does not contain {}", fmt, FileFormat::Appimage);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_debian_package() {
    let fmt = FileFormat::from_media_type("application/vnd.debian.binary-package");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::DebianPackage)), "{:?} does not contain {}", fmt, FileFormat::DebianPackage);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_enterprise_application_archive() {
    let fmt = FileFormat::from_media_type("application/java-archive");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::EnterpriseApplicationArchive)), "{:?} does not contain {}", fmt, FileFormat::EnterpriseApplicationArchive);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_google_chrome_extension() {
    let fmt = FileFormat::from_media_type("application/x-google-chrome-extension");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::GoogleChromeExtension)), "{:?} does not contain {}", fmt, FileFormat::GoogleChromeExtension);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_ios_app_store_package() {
    let fmt = FileFormat::from_media_type("application/x-ios-app");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::IosAppStorePackage)), "{:?} does not contain {}", fmt, FileFormat::IosAppStorePackage);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_java_archive() {
    let fmt = FileFormat::from_media_type("application/java-archive");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::JavaArchive)), "{:?} does not contain {}", fmt, FileFormat::JavaArchive);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_microsoft_software_installer() {
    let fmt = FileFormat::from_media_type("application/x-msi");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftSoftwareInstaller)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftSoftwareInstaller);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_microsoft_visual_studio_extension() {
    let fmt = FileFormat::from_media_type("application/vsix");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MicrosoftVisualStudioExtension)), "{:?} does not contain {}", fmt, FileFormat::MicrosoftVisualStudioExtension);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_nintendo_switch_package() {
    let fmt = FileFormat::from_media_type("application/x-nintendo-switch-package");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::NintendoSwitchPackage)), "{:?} does not contain {}", fmt, FileFormat::NintendoSwitchPackage);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_red_hat_package_manager() {
    let fmt = FileFormat::from_media_type("application/x-rpm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::RedHatPackageManager)), "{:?} does not contain {}", fmt, FileFormat::RedHatPackageManager);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_web_application_archive() {
    let fmt = FileFormat::from_media_type("application/java-archive");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WebApplicationArchive)), "{:?} does not contain {}", fmt, FileFormat::WebApplicationArchive);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_windows_app_bundle() {
    let fmt = FileFormat::from_media_type("application/vnd.ms-appx.bundle");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WindowsAppBundle)), "{:?} does not contain {}", fmt, FileFormat::WindowsAppBundle);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_windows_app_package() {
    let fmt = FileFormat::from_media_type("application/vnd.ms-appx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WindowsAppPackage)), "{:?} does not contain {}", fmt, FileFormat::WindowsAppPackage);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_xap(){
    let fmt = FileFormat::from_media_type("application/x-silverlight-app");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Xap)), "{:?} does not contain {}", fmt, FileFormat::Xap);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_xpinstall(){
    let fmt = FileFormat::from_media_type("application/x-xpinstall");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Xpinstall)), "{:?} does not contain {}", fmt, FileFormat::Xpinstall);
}

