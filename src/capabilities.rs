//! A set of parameters used to start an Appium session.
//!
//! ## What are capabilities?
//! The information in the set is used to describe what sort of "capabilities" you want your session to have,
//! for example, a certain mobile operating system or a certain version of a device.
//!
//! When you start your Appium session, your Appium client will include the set of capabilities
//! you've defined as an object in the JSON-formatted body of the request.
//!
//! Capabilities are represented as key-value pairs, with values allowed to be any valid JSON type, including other objects.
//! Appium will then examine the capabilities and make sure that it can satisfy them before proceeding to start the session and
//! return an ID representing the session to your client library.
//!
//! See also <https://appium.io/docs/en/2.1/guides/caps/>.
//!
//! ## Platform-specific capabilities
//! You can create capabilities to pass to Appium sercer by using either [android::AndroidCapabilities] or [ios::IOSCapabilities].
//!
//! For example, if you wish to use UiAutomator2 as a driver for Android tests, you can write:
//! ```rust
//! use appium_client::capabilities::android::AndroidCapabilities;
//! use appium_client::capabilities::{AppCapable, AppiumCapability, UdidCapable, UiAutomator2AppCompatible};
//!
//! let mut capabilities = AndroidCapabilities::new_uiautomator();
//! capabilities.udid("emulator-5554");  // This capability selects the device you wish to run test on
//! capabilities.app("/apps/sample.apk"); // This is a path to apk on your computer
//! capabilities.app_wait_activity("com.example.AppActivity"); // This is an activity of a home screen of the app.
//!
//! // If you wish to specify appium capabilities manually (without predefined methods),
//! // then use methods like set_bool, set_str, set_number
//! capabilities.set_bool("appium:noReset", true);
//! ```
//!
//! ## Blank capabilities
//! To use blank capabilities (without any predefined methods for ease of configuration), use [empty::EmptyCapabilities].
//! [empty::EmptyCapabilities] lets you configure Appium for any driver that was not implemented out of the box.
//!
//! Note that you will loose many built in features for Android and iOS, basically [empty::EmptyCapabilities] requires
//! that you setup everything by yourself (including some Appium commands).

pub mod ios;
pub mod automation;
pub mod android;
pub mod empty;

use std::ops::{Deref, DerefMut};
use std::time::Duration;
use fantoccini::wd::Capabilities;
use serde_json::{Number, Value};

/// Extensions to easily define capabilities for Appium driver. See <https://appium.io/docs/en/2.1/guides/caps/>.
pub trait AppiumCapability
    where Self: Deref<Target=Capabilities>,
          Self: DerefMut<Target=Capabilities> {

    /// Set the automation driver to use (the engine for tests, eg. XCuiTest for iOS).
    /// 
    /// Appium usually autoselects the driver based on platform, but you choose the preferred driver.
    /// See [automation] for possible values.
    fn automation_name(&mut self, automation_name: &str) {
        self.set_str("appium:automationName", automation_name);
    }

    /// The version of a platform, e.g., for iOS, "16.0"
    fn platform_version(&mut self, version: &str) {
        self.set_str("appium:platformVersion", version);
    }

    /// The name of a particular device to automate.
    ///
    /// For example "iPhone 14".
    /// Currently only actually useful for specifying iOS simulators,
    /// since in other situations it's typically recommended to use a specific device
    /// id via the `appium:udid` capability.
    fn device_name(&mut self, device_name: &str) {
        self.set_str("appium:deviceName", device_name);
    }

    /// Sets a string capability.
    ///
    /// For example `set_str("appium:deviceName", "iPhone 14")`.
    fn set_str(&mut self, name: &str, value: &str) {
        self.insert(name.to_string(), Value::String(value.to_string()));
    }

    /// Sets a number capability.
    ///
    /// For example `set_number("appium:newCommandTimeout", Number::from(120u64))`.
    fn set_number(&mut self, name: &str, value: Number) {
        self.insert(name.to_string(), Value::Number(value));
    }

    /// Sets a boolean capability.
    ///
    /// For example `set_bool("appium:noReset", true)`
    fn set_bool(&mut self, name: &str, value: bool) {
        self.insert(name.to_string(), Value::Bool(value));
    }
}

/// Capabilities for drivers that are used to run test on a device.
pub trait UdidCapable: AppiumCapability {
    /// Device id.
    ///
    /// Android id can be retrieved by using ADB (`adb devices`).
    /// For iOS, it's the phone's serial number.
    fn udid(&mut self, udid: &str) {
        self.set_str("appium:udid", udid);
    }
}

/// Capabilities for drivers that are used to run an app.
pub trait AppCapable: AppiumCapability {
    /// The path to an installable application.
    fn app(&mut self, app_path: &str) {
        self.set_str("appium:app", app_path);
    }

    /// App or list of apps (as a JSON array) to install prior to running tests.
    ///
    /// Note that it will not work with `automationName` of `Espresso` and iOS real devices
    fn other_apps(&mut self, paths: &[&str]) {
        let paths = paths.iter()
            .map(|p| Value::String(p.to_string()))
            .collect();

        self.insert("appium:otherApps".to_string(), Value::Array(paths));
    }

    /// Don't reset app state before this session.
    ///
    /// "Reset" means to delete app data (like a fresh install).
    /// If true, instruct an Appium driver to avoid its usual reset logic during session start and cleanup (default false).
    fn no_reset(&mut self, no_reset: bool) {
        self.set_bool("appium:noReset", no_reset);
    }

    /// Perform a complete reset.
    ///
    /// "Complete reset" usually means a reinstall.
    /// If true, instruct an Appium driver to augment its usual reset logic with additional steps to ensure maximum environmental reproducibility (default false)
    fn full_reset(&mut self, full_reset: bool) {
        self.set_bool("appium:fullReset", full_reset);
    }

    /// When a find operation fails, print the current page source. Defaults to false.
    ///
    /// When the element you're looking for is not found on screen, then this setting will print DOM
    /// of the visible app screen.
    /// This DOM can be further inspected to check if the locator is correct, or if the correct page is displayed.
    fn print_page_source_on_find_failure(&mut self, value: bool) {
        self.set_bool("appium:printPageSourceOnFindFailure", value);
    }
}

/// Capabilities for UiAutomator2 (Android).
pub trait UiAutomator2AppCompatible: AppiumCapability {
    /// Activity name for the Android activity you want to launch from your package.
    ///
    /// This often needs to be preceded by a `.` (a dot, e.g., `.MainActivity` instead of `MainActivity`).
    /// By default this capability is received from the package manifest.
    fn app_activity(&mut self, activity: &str) {
        self.set_str("appium:appActivity", activity);
    }

    /// Java package of the Android app you want to run.
    ///
    /// By default this capability is received from the package manifest.
    fn app_package(&mut self, activity: &str) {
        self.set_str("appium:appPackage", activity);
    }

    /// Activity name/names, comma separated, for the Android activity you want to wait for.
    ///
    /// By default the value of this capability is the same as for appActivity.
    /// You must set it to the very first focused application activity name in case it is different
    /// from the one which is set as appActivity if your capability has `appActivity` and `appPackage`.
    /// You can also use wildcards (*).
    fn app_wait_activity(&mut self, activity: &str) {
        self.set_str("appium:appWaitActivity", activity);
    }

    /// Java package of the Android app you want to wait for.
    ///
    /// By default the value of this capability is the same as for appActivity.
    fn app_wait_package(&mut self, activity: &str) {
        self.set_str("appWaitPackage", activity);
    }

    /// Timeout in milliseconds used to wait for the appWaitActivity to launch (default 20000)
    fn app_wait_duration(&mut self, duration: Duration) {
        self.set_number("appium:appWaitDuration", Number::from(duration.as_millis() as u64));
    }

    /// Timeout in milliseconds used to wait for an apk to install to the device. Defaults to 90000
    fn android_install_timeout(&mut self, duration: Duration) {
        self.set_number("appium:androidInstallTimeout", Number::from(duration.as_millis() as u64));
    }

    /// Block until app starts.
    ///
    /// Whether to block until the app under test returns the control to the caller after its activity
    /// has been started by Activity Manager (true, the default value) or to continue the test without waiting for that (false)
    fn app_wait_for_launch(&mut self, value: bool) {
        self.set_bool("appWaitForLaunch", value);
    }

    /// Always start app forcefully when testing starts.
    ///
    /// Set it to true if you want the application under test to be always forcefully
    /// restarted on session startup even if appium:noReset is true,
    /// and the app was already running. If noReset is falsy, then the app under test is going
    /// to be restarted if either this capability set to true or appium:dontStopAppOnReset is falsy
    /// (the default behavior). false by default. Available since driver version 2.12
    fn force_app_launch(&mut self, value: bool) {
        self.set_bool("appium:forceAppLaunch", value)
    }

    /// Whether to launch the application under test automatically after a test starts.
    ///
    /// Default: true
    fn auto_launch(&mut self, value: bool) {
        self.set_bool("appium:autoLaunch", value)
    }

    /// Set an optional intent category to be applied when starting the given appActivity by Activity Manager.
    ///
    /// Defaults to `android.intent.category.LAUNCHER`.
    /// Please use `mobile:startActivity` in case you don't set an explicit value.
    fn intent_category(&mut self, value: &str) {
        self.set_str("appium:intentCategory", value);
    }

    /// Set an optional intent action to be applied when starting the given appActivity by Activity Manager.
    ///
    /// Defaults to `android.intent.action.MAIN`. Please use `mobile:startActivity` in case you don't set an explicit value.
    fn intent_action(&mut self, value: &str) {
        self.set_str("appium:intentAction", value);
    }

    /// Set an optional intent flags to be applied when starting the given appActivity by Activity Manager.
    ///
    /// Defaults to 0x10200000 (FLAG_ACTIVITY_NEW_TASK)
    fn intent_flags(&mut self, value: &str) {
        self.set_str("appium:intentFlags", value);
    }

    /// Set an optional intent arguments to be applied when starting the given appActivity by Activity Manager
    fn optional_intent_arguments(&mut self, value: &str) {
        self.set_str("appium:optionalIntentArguments", value);
    }

    /// Set it to true if you don't want the application to be restarted if it was already running.
    ///
    /// If appium:noReset is falsy, then the app under test is going to be restarted if either
    /// this capability is falsy (the default behavior) or appium:forceAppLaunch is set to true.
    ///
    /// `false` by default
    fn dont_stop_app_on_reset(&mut self, value: bool) {
        self.set_bool("appium:dontStopAppOnReset", value);
    }

    /// Allows to set one or more comma-separated package identifiers to be uninstalled from the device before a test starts.
    fn uninstall_other_packages(&mut self, value: &str) {
        self.set_str("appium:uninstallOtherPackages", value);
    }

    /// Sets the maximum amount of application packages to be cached on the device under test.
    ///
    /// This is needed for devices that don't support streamed installs (Android 7 and below),
    /// because ADB must push app packages to the device first in order to install them, which takes some time.
    ///
    /// Setting this capability to zero disables apps caching. 10 by default.
    fn remote_apps_cache_limit(&mut self, value: u64) {
        self.set_number("appium:remoteAppsCacheLimit", Number::from(value));
    }

    /// Use packages built with test flag.
    ///
    /// If set to true then it would be possible to use packages built with the test flag
    /// for the automated testing (literally adds -t flag to the adb install command).
    ///
    /// false by default
    fn allow_test_packages(&mut self, value: bool) {
        self.set_bool("appium:allowTestPackages", value);
    }

    /// Reinstall app (even if it's a downgrade).
    ///
    /// If set to true then the application under test is always reinstalled even if a newer version
    /// of it already exists on the device under test.
    /// This capability has no effect if `appium:noReset` is set to true.
    ///
    /// `false` by default
    fn enforce_app_install(&mut self, value: bool) {
        self.set_bool("appium:enforceAppInstall", value);
    }
}

/// Capabilities for Settings API (<https://appium.io/docs/en/2.1/guides/settings/>).
pub trait AppiumSettingsCapable: AppiumCapability {
    fn set_setting(&mut self, name: &str, value: Value) {
        self.insert(format!("appium:settings[{name}]"), value);
    }
}

/// Capabilities for XCUITest (iOS).
pub trait XCUITestAppCompatible: AppiumCapability {
    /// Bundle id of app. Looks like app package (`com.my.app`).
    fn bundle_id(&mut self, id: &str) {
        self.set_str("appium:bundleId", id);
    }

    /// Where to look for localizable strings. Default en.lproj
    fn localizable_strings_dir(&mut self, dir: &str) {
        self.set_str("appium:localizableStringsDir", dir);
    }

    /// Language to set for the simulator / emulator.
    ///
    /// You need to set this manually on physical devices.
    fn language(&mut self, language: &str) {
        self.set_str("appium:language", language);
    }

    /// Locale to set for the simulator / emulator.
    ///
    /// You need to set this manually on physical devices.
    fn locale(&mut self, locale: &str) {
        self.set_str("appium:locale", locale);
    }

    /// Calendar format to set for the iOS Simulator (eg. `gregorian`).
    fn calendar_format(&mut self, value: &str) {
        self.set_str("appium:calendarFormat", value);
    }

    /// Timeout for application upload in millisecond, on real devices
    fn app_push_timeout(&mut self, duration: Duration) {
        self.set_number("appium:appPushTimeout", Number::from(duration.as_millis() as u64));
    }

    /// Select application installation strategy for real devices.
    ///
    /// The following strategies are supported:
    /// * `serial` (default) - pushes app files to the device in a sequential order; this is the least performant strategy, although the most reliable;
    /// * `parallel` - pushes app files simultaneously; this is usually the the most performant strategy, but sometimes could not be very stable;
    /// * `ios-deploy` - tells the driver to use a third-party tool ios-deploy to install the app; obviously the tool must be installed separately first and must be present in PATH before it could be used.
    fn app_install_strategy(&mut self, value: &str) {
        self.set_str("appium:appInstallStrategy", value);
    }

    /// Accept all iOS alerts automatically if they pop up.
    ///
    /// This includes privacy access permission alerts (e.g., location, contacts, photos). Default is false.
    fn auto_accept_alerts(&mut self, value: bool) {
        self.set_bool("appium:autoAcceptAlerts", value);
    }
}
