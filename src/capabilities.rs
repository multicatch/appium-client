use std::ops::{Deref, DerefMut};
use fantoccini::wd::Capabilities;
use serde_json::Value;

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

impl ActivityCapable for AndroidCapabilities {}


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

impl ActivityCapable for IOSCapabilities {}


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

pub trait ActivityCapable: AppiumCapability {

    fn app_activity(&mut self, activity: &str) {
        self.set_str("appWaitActivity", activity);
    }

    fn app_package(&mut self, activity: &str) {
        self.set_str("appWaitActivity", activity);
    }

    fn app_wait_activity(&mut self, activity: &str) {
        self.set_str("appWaitActivity", activity);
    }

    fn app_wait_package(&mut self, activity: &str) {
        self.set_str("appWaitActivity", activity);
    }
}

pub trait AppiumSettingsCapable: AppiumCapability {
    fn set_setting(&mut self, name: &str, value: Value) {
        self.insert(format!("settings[{name}]"), value);
    }
}