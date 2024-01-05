//! Commands that you can issue to Appium server
//!
//! The commands in submodules are a facade to low-level `issue_cmd` ([fantoccini::Client::issue_cmd]).
//! So in most cases, you need a specific function from one of those modules (e.g. [keyboard::HidesKeyboard::hide_keyboard]).
//!
//! ## Available commands
//! **Please check all submodules if you want to learn what features are implemented in this lib.**
//! See traits in below modules to learn what you can do with the client.
//!
//! Alternatively, you can check out [crate::IOSClient] and [crate::AndroidClient] to see all traits of those clients in the docs.
//!
//! ## How to use commands
//! [AppiumCommand] is a struct used by low-level `issue_cmd` ([fantoccini::Client::issue_cmd]).
//! So unless you're implementing missing features yourself, you don't need to wory about it.
//!
//! This lib exposes both APIs to be more flexible.
//! So a rule of thumb is:
//! * use a command from submodule if it's available (in other words - use by default),
//! * use [AppiumCommand::Custom] in other cases
//!
//! ```no_run
//!# use http::Method;
//!# use serde_json::json;
//!# use appium_client::capabilities::android::AndroidCapabilities;
//!# use appium_client::capabilities::{AppCapable, UdidCapable, UiAutomator2AppCompatible};
//!# use appium_client::ClientBuilder;
//!# use appium_client::commands::AppiumCommand;
//!# use appium_client::commands::keyboard::HidesKeyboard;
//!# use appium_client::find::{AppiumFind, By};
//!
//!# #[tokio::main]
//!# async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!# // create capabilities
//! let mut capabilities = AndroidCapabilities::new_uiautomator();
//!# capabilities.udid("emulator-5554");
//!# capabilities.app("/apps/sample.apk");
//!# capabilities.app_wait_activity("com.example.AppActivity");
//!
//! let client = ClientBuilder::native(capabilities)
//!    .connect("http://localhost:4723/wd/hub/")
//!    .await?;
//!
//! // this feature is implemented in keyboard submodule (recommended)
//! client.hide_keyboard().await?;
//!
//! // this is a low-level implementation of the same command (not recommended, unless you have a specific use case for this)
//! client.issue_cmd(AppiumCommand::Custom(
//!     Method::POST,
//!     "appium/device/hide_keyboard".to_string(),
//!     Some(json!({})),
//! )).await?;
//!
//!#     Ok(())
//!# }
//! ```
//!

pub mod rotation;
pub mod keyboard;
pub mod lock;
pub mod contexts;
pub mod location;
pub mod time;
pub mod files;
pub mod apps;
pub mod strings;
pub mod network;
pub mod android;
pub mod settings;
pub mod authentication;
pub mod recording;
pub mod clipboard;
pub mod battery;
pub mod ios;

use fantoccini::wd::WebDriverCompatibleCommand;
use http::Method;
use serde_json::Value;
use crate::find::By;

/// Basic Appium commands
///
/// Use Custom if you want to implement anything non-standard.
/// Those commands are to be used with `issue_cmd` ([fantoccini::Client::issue_cmd]).
#[derive(Debug, PartialEq)]
pub enum AppiumCommand {
    FindElement(By),
    FindElementWithContext(By, String),
    FindElements(By),
    FindElementsWithContext(By, String),
    Custom(Method, String, Option<Value>),
}

impl WebDriverCompatibleCommand for AppiumCommand {
    fn endpoint(
        &self,
        base_url: &url::Url,
        session_id: Option<&str>,
    ) -> Result<url::Url, url::ParseError> {
        let base = { base_url.join(&format!("session/{}/", session_id.as_ref().unwrap()))? };
        match self {
            AppiumCommand::FindElement(..) =>
                base.join("element"),
            AppiumCommand::FindElements(..) =>
                base.join("elements"),
            AppiumCommand::FindElementWithContext(.., context) =>
                base.join("element")
                    .and_then(|url| url.join(context))
                    .and_then(|url| url.join("element")),
            AppiumCommand::FindElementsWithContext(.., context) =>
                base.join("element")
                    .and_then(|url| url.join(context))
                    .and_then(|url| url.join("elements")),
            AppiumCommand::Custom(_, command, ..) =>
                base.join(command),
        }
    }

    fn method_and_body(&self, _request_url: &url::Url) -> (Method, Option<String>) {
        match self {
            AppiumCommand::FindElement(by)
            | AppiumCommand::FindElements(by)
            | AppiumCommand::FindElementWithContext(by, ..)
            | AppiumCommand::FindElementsWithContext(by, ..) => {
                let method = Method::POST;
                let body = Some(serde_json::to_string(&by).unwrap());

                (method, body)
            },

            AppiumCommand::Custom(method, .., value) => {
                let body = value.clone()
                    .map(|v| v.to_string());

                (method.clone(), body)
            }
        }
    }

    fn is_new_session(&self) -> bool {
        false
    }

    fn is_legacy(&self) -> bool {
        false
    }
}