use std::ops::{Deref, DerefMut};
use fantoccini::wd::Capabilities;
use serde_json::Value;
use crate::capabilities::{AppCapable, AppiumCapability, AppiumSettingsCapable, UdidCapable, XCUITestAppCompatible};
use crate::capabilities::automation::IOS_XCUI_TEST;

/// iOS capabilities
///
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IOSCapabilities {
    inner: Capabilities,
}

impl IOSCapabilities {
    /// Creates new empty capability set for iOS (with driver autoselected by Appium).
    pub fn new() -> IOSCapabilities {
        let mut inner = Capabilities::new();
        inner.insert("platformName".to_string(), Value::String("iOS".to_string()));

        IOSCapabilities {
            inner
        }
    }
    
    /// Creates empty capability set for XCuiTest iOS driver.
    pub fn new_xcui() -> IOSCapabilities {
        let mut capabilities = IOSCapabilities::new();
        capabilities.automation_name(IOS_XCUI_TEST);
        capabilities
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
