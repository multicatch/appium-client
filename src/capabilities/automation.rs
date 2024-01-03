//! Automation name constants 
//! 
//! These constants contain names of build-in Appium drivers (at least as of Appium 2.1).
//! Use them to with [crate::capabilities::AppiumCapability::automation_name].
//! 
//! Note: [crate::capabilities::android::AndroidCapabilities] and [crate::capabilities::ios::IOSCapabilities] set them automatically.
  
/// <https://github.com/appium/appium-xcuitest-driver>
pub const IOS_XCUI_TEST: &str = "XCuiTest";

/// <https://github.com/appium/appium-uiautomator2-driver>
pub const ANDROID_UIAUTOMATOR2: &str = "UIAutomator2";

/// <https://github.com/appium/appium-espresso-driver>
pub const ESPRESSO: &str = "Espresso";

/// <https://github.com/appium/appium-mac2-driver>
pub const MAC2: &str = "Mac2";

/// <https://github.com/appium/appium-windows-driver>
pub const WINDOWS: &str = "Windows";

/// <https://github.com/appium/appium-safari-driver>
pub const SAFARI: &str = "Safari";

/// <https://github.com/appium/appium-geckodriver>
pub const GECKO: &str = "Gecko";

/// Third-party drivers
/// <https://github.com/YOU-i-Labs/appium-youiengine-driver>
pub const YOUI_ENGINE: &str = "youiengine";