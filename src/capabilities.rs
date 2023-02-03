use std::ops::{Deref, DerefMut};
use fantoccini::wd::Capabilities;
use serde_json::Value;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AndroidCapabilities {
    inner: Capabilities
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

impl Into<Capabilities> for AndroidCapabilities {
    fn into(self) -> Capabilities {
        self.inner
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

pub trait AppiumCapability where Self: DerefMut<Target=Capabilities> {
    fn set_str(&mut self, name: &str, value: &str) {
        self.insert(name.to_string(), Value::String(value.to_string()));
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
}

pub trait AppWaitActivityCapable: AppiumCapability{
    fn app_wait_activity(&mut self, wait_activity: &str) {
        self.set_str("appWaitActivity", wait_activity);
    }
}

impl AppiumCapability for AndroidCapabilities {}
impl UdidCapable for AndroidCapabilities {}
impl AppCapable for AndroidCapabilities {}
impl AppWaitActivityCapable for AndroidCapabilities {}