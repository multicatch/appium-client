use std::ops::{Deref, DerefMut};
use fantoccini::wd::Capabilities;
use serde_json::Value;
use crate::capabilities::{AppCapable, AppiumCapability, AppiumSettingsCapable, UdidCapable, UiAutomator2AppCompatible};
use crate::capabilities::automation::{ANDROID_UIAUTOMATOR2, ESPRESSO};

/// Android capabilities
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AndroidCapabilities {
    inner: Capabilities,
}

impl AndroidCapabilities {
    /// Creates new empty capability set for Android (with driver autoselected by Appium).
    pub fn new() -> AndroidCapabilities {
        let mut inner = Capabilities::new();
        inner.insert("platformName".to_string(), Value::String("android".to_string()));

        AndroidCapabilities {
            inner
        }
    }

    /// Creates empty capability set for UiAutomator2 Android driver.
    pub fn new_uiautomator() -> AndroidCapabilities {
        let mut capabilities = AndroidCapabilities::new();
        capabilities.automation_name(ANDROID_UIAUTOMATOR2);
        capabilities
    }

    /// Creates empty capability set for Espresso Android driver.
    pub fn new_espresso() -> AndroidCapabilities {
        let mut capabilities = AndroidCapabilities::new();
        capabilities.automation_name(ESPRESSO);
        capabilities
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

