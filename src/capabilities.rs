use std::ops::{Deref, DerefMut};
use std::time::Duration;
use fantoccini::wd::Capabilities;
use serde_json::{Number, Value};

///
/// Android capabilities
///
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AndroidCapabilities {
    inner: Capabilities,
}

impl AndroidCapabilities {
    pub fn new() -> AndroidCapabilities {
        let mut inner = Capabilities::new();
        inner.insert("platformName".to_string(), Value::String("android".to_string()));

        AndroidCapabilities {
            inner
        }
    }
}

impl Default for AndroidCapabilities {
    fn default() -> Self {
        Self::new()
    }
}

impl From<AndroidCapabilities> for Capabilities {
    fn from(value: AndroidCapabilities) -> Self {
        value.inner
    }
}

impl Deref for AndroidCapabilities {
    type Target = Capabilities;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for AndroidCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AppiumCapability for AndroidCapabilities {}

impl UdidCapable for AndroidCapabilities {}

impl AppCapable for AndroidCapabilities {}

impl AppiumSettingsCapable for AndroidCapabilities {}

impl UiAutomator2AppCompatible for AndroidCapabilities {}


///
/// iOS capabilities
///
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IOSCapabilities {
    inner: Capabilities,
}

impl IOSCapabilities {
    pub fn new() -> IOSCapabilities {
        let mut inner = Capabilities::new();
        inner.insert("platformName".to_string(), Value::String("iOS".to_string()));

        IOSCapabilities {
            inner
        }
    }
}

impl Default for IOSCapabilities {
    fn default() -> Self {
        Self::new()
    }
}

impl From<IOSCapabilities> for Capabilities {
    fn from(value: IOSCapabilities) -> Self {
        value.inner
    }
}

impl Deref for IOSCapabilities {
    type Target = Capabilities;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for IOSCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AppiumCapability for IOSCapabilities {}

impl UdidCapable for IOSCapabilities {}

impl AppCapable for IOSCapabilities {}

impl AppiumSettingsCapable for IOSCapabilities {}

impl XCUITestAppCompatible for IOSCapabilities {}

///
/// Extensions to easily define capabilities for Appium driver
///
pub trait AppiumCapability where Self: DerefMut<Target=Capabilities> {
    fn automation_name(&mut self, automation_name: &str) {
        self.set_str("automationName", automation_name);
    }

    fn platform_version(&mut self, version: &str) {
        self.set_str("platformVersion", version);
    }

    fn device_name(&mut self, device_name: &str) {
        self.set_str("deviceName", device_name);
    }

    fn set_str(&mut self, name: &str, value: &str) {
        self.insert(name.to_string(), Value::String(value.to_string()));
    }

    fn set_number(&mut self, name: &str, value: Number) {
        self.insert(name.to_string(), Value::Number(value));
    }

    fn set_bool(&mut self, name: &str, value: bool) {
        self.insert(name.to_string(), Value::Bool(value));
    }
}

pub trait UdidCapable: AppiumCapability {
    fn udid(&mut self, udid: &str) {
        self.set_str("udid", udid);
    }
}

pub trait AppCapable: AppiumCapability {
    fn app(&mut self, app_path: &str) {
        self.set_str("app", app_path);
    }

    fn other_apps(&mut self, paths: &[&str]) {
        let paths = paths.iter()
            .map(|p| Value::String(p.to_string()))
            .collect();

        self.insert("otherApps".to_string(), Value::Array(paths));
    }

    fn no_reset(&mut self, no_reset: bool) {
        self.set_bool("noReset", no_reset);
    }

    fn full_reset(&mut self, full_reset: bool) {
        self.set_bool("fullReset", full_reset);
    }

    fn print_page_source_on_find_failure(&mut self, value: bool) {
        self.set_bool("printPageSourceOnFindFailure", value);
    }
}

pub trait UiAutomator2AppCompatible: AppiumCapability {

    fn app_activity(&mut self, activity: &str) {
        self.set_str("appActivity", activity);
    }

    fn app_package(&mut self, activity: &str) {
        self.set_str("appPackage", activity);
    }

    fn app_wait_activity(&mut self, activity: &str) {
        self.set_str("appWaitActivity", activity);
    }

    fn app_wait_package(&mut self, activity: &str) {
        self.set_str("appWaitPackage", activity);
    }

    fn app_wait_duration(&mut self, duration: Duration) {
        self.set_number("appWaitDuration", Number::from(duration.as_millis() as u64));
    }

    fn android_install_timeout(&mut self, duration: Duration) {
        self.set_number("androidInstallTimeout", Number::from(duration.as_millis() as u64));
    }

    fn app_wait_for_launch(&mut self, value: bool) {
        self.set_bool("appWaitForLaunch", value);
    }

    fn force_app_launch(&mut self, value: bool) {
        self.set_bool("forceAppLaunch", value)
    }

    fn auto_launch(&mut self, value: bool) {
        self.set_bool("autoLaunch", value)
    }

    fn intent_category(&mut self, value: &str) {
        self.set_str("intentCategory", value);
    }

    fn intent_action(&mut self, value: &str) {
        self.set_str("intentAction", value);
    }
    
    fn intent_flags(&mut self, value: &str) {
        self.set_str("intentFlags", value);
    }

    fn optional_intent_arguments(&mut self, value: &str) {
        self.set_str("optionalIntentArguments", value);
    }

    fn dont_stop_app_on_reset(&mut self, value: bool) {
        self.set_bool("dontStopAppOnReset", value);
    }

    fn uninstall_other_packages(&mut self, value: &str) {
        self.set_str("uninstallOtherPackages", value);
    }

    fn remote_apps_cache_limit(&mut self, value: u64) {
        self.set_number("remoteAppsCacheLimit", Number::from(value));
    }

    fn allow_test_packages(&mut self, value: bool) {
        self.set_bool("allowTestPackages", value);
    }

    fn enforce_app_install(&mut self, value: bool) {
        self.set_bool("enforceAppInstall", value);
    }
}

pub trait AppiumSettingsCapable: AppiumCapability {
    fn set_setting(&mut self, name: &str, value: Value) {
        self.insert(format!("settings[{name}]"), value);
    }
}

pub trait XCUITestAppCompatible: AppiumCapability {
    fn bundle_id(&mut self, id: &str) {
        self.set_str("bundleId", id);
    }

    fn localizable_strings_dir(&mut self, dir: &str) {
        self.set_str("localizableStringsDir", dir);
    }

    fn language(&mut self, language: &str) {
        self.set_str("language", language);
    }

    fn locale(&mut self, locale: &str) {
        self.set_str("locale", locale);
    }

    fn calendar_format(&mut self, value: &str) {
        self.set_str("calendarFormat", value);
    }

    fn app_push_timeout(&mut self, duration: Duration) {
        self.set_number("appPushTimeout", Number::from(duration.as_millis() as u64));
    }

    fn app_install_strategy(&mut self, value: &str) {
        self.set_str("appInstallStrategy", value);
    }

    fn auto_accept_alerts(&mut self, value: bool) {
        self.set_bool("autoAcceptAlerts", value);
    }
}