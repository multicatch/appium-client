//! Empty capabilities (for creating a blank client)
//!
//! A blank client is an appium client that is not tied to any specific driver (or a driver not supported by this lib).
//! You can use [EmptyCapabilities] to manually set any capability and create a featureless Appium client.
//!
//! Such featureless Appium client has a basic feature of [crate::find], but any other feature has to be implemented by using [crate::commands::AppiumCommand::Custom].
//!
//! You can use a featureless client to use any Appium driver not currently supported by this lib (other than Android and iOS).
//! Upside: you can use this lib.
//! Downside: no features (yet).
//!
//! ```no_run
//! use http::Method;
//! use serde_json::json;
//! use appium_client::capabilities::AppiumCapability;
//! use appium_client::capabilities::automation::ANDROID_UIAUTOMATOR2;
//! use appium_client::capabilities::empty::EmptyCapabilities;
//! use appium_client::ClientBuilder;
//! use appium_client::commands::AppiumCommand;
//! use appium_client::find::{AppiumFind, By};
//!
//!# #[tokio::main]
//!# async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut capabilities = EmptyCapabilities::new();
//! capabilities.automation_name(ANDROID_UIAUTOMATOR2);
//!
//! let client = ClientBuilder::native(capabilities)
//!    .connect("http://localhost:4723/wd/hub/")
//!    .await?;
//!
//! // find works out of the box
//! let element = client.find_by(By::Id("elementId".to_string())).await?;
//!
//! // any other feature must be implemented by issuing a custom command
//! // for example, this is a command used to set geolocation on Android
//! client.issue_cmd(AppiumCommand::Custom(
//!     Method::POST,
//!     "location".to_string(),
//!     Some(json!({
//!         "location": {
//!             "latitude": 121.21,
//!             "longitude": 11.56,
//!             "altitude": 94.23
//!         }
//!     }))
//! )).await?;
//!# Ok(())
//!# }
//! ```

use std::ops::{Deref, DerefMut};
use fantoccini::wd::Capabilities;
use crate::capabilities::AppiumCapability;

/// Empty capabilities - for use in tests or with a platform not implemented by this lib.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct EmptyCapabilities {
    inner: Capabilities,
}

impl EmptyCapabilities {
    /// Creates new empty capability set
    pub fn new() -> EmptyCapabilities {
        EmptyCapabilities {
            inner: Capabilities::new()
        }
    }
}

impl Default for EmptyCapabilities {
    fn default() -> Self {
        Self::new()
    }
}

impl From<EmptyCapabilities> for Capabilities {
    fn from(value: EmptyCapabilities) -> Self {
        value.inner
    }
}

impl Deref for EmptyCapabilities {
    type Target = Capabilities;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for EmptyCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AppiumCapability for EmptyCapabilities {}