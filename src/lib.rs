//! Rust client for Appium Server, for automated mobile app testing
//!
//! It is based on [fantoccini](https://github.com/jonhoo/fantoccini) and retains all capabilities
//! of fantoccini's client, such as screenshotting, touch actions, getting page source etc.
//!
//! Please note that this is only a client of [Appium](https://appium.io/docs/en/2.1/), so also check out [Appium docs](https://appium.io/docs/en/2.1/).
//!
//! ## Creating a client
//! To create a client, you need [capabilities] for the Appium session.
//! Capabilities describe what device you use and they will determine what features are available to you.
//!
//! After creating a desired set of capabilities, use [ClientBuilder] to create a client.
//! And you also need a running Appium server, see Appium docs for how to set up one (<https://appium.io/docs/en/2.1/quickstart/>).
//!
//! Creating an iOS capabilities and client:
//! ```no_run
//! use appium_client::capabilities::{AppCapable, UdidCapable};
//! use appium_client::capabilities::ios::IOSCapabilities;
//! use appium_client::ClientBuilder;
//! use appium_client::commands::ios::ShakesDevice;
//!
//!# #[tokio::main]
//!# async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut capabilities = IOSCapabilities::new_xcui();
//! capabilities.udid("000011114567899");
//! capabilities.app("/apps/sample.ipa");
//!
//! let client = ClientBuilder::native(capabilities)
//!    .connect("http://localhost:4723/wd/hub/")
//!    .await?;
//!
//! // now you can use the client to issue commands and find elements on screen
//! # Ok(())
//! # }
//! ```
//!
//! ## Finding elements
//! This appium-client adds support for Appium locators such as iOS Class Chain, or UiAutomator.
//! See [find] for more info on Appium locators.
//!
//! Basic usage:
//! ```no_run
//! use appium_client::capabilities::android::AndroidCapabilities;
//!# use appium_client::capabilities::{AppCapable, UdidCapable, UiAutomator2AppCompatible};
//! use appium_client::ClientBuilder;
//! use appium_client::find::{AppiumFind, By};
//!
//!# #[tokio::main]
//!# async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!# // create capabilities
//! let mut capabilities = AndroidCapabilities::new_uiautomator();
//!# capabilities.udid("emulator-5554");
//!# capabilities.app("/apps/sample.apk");
//!# capabilities.app_wait_activity("com.example.AppActivity");
//!#
//! // create the client
//! let client = ClientBuilder::native(capabilities)
//!     .connect("http://localhost:4723/wd/hub/")
//!     .await?;
//!
//! // locate an element (find it)
//! let element = client
//!     .find_by(By::accessibility_id("Click this"))
//!     .await?;
//!
//! element.click().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Wait for an element to appear
//! Appium locators can be also waited on (just like you can wait for element with fantoccini),
//! see [wait] to learn how to wait.
//!
//! Basic usage:
//! ```no_run
//! use appium_client::capabilities::android::AndroidCapabilities;
//!# use appium_client::capabilities::{AppCapable, UdidCapable, UiAutomator2AppCompatible};
//! use appium_client::ClientBuilder;
//! use appium_client::find::{AppiumFind, By};
//!
//!# #[tokio::main]
//!# async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!# // create capabilities
//!# use appium_client::wait::AppiumWait;
//! let mut capabilities = AndroidCapabilities::new_uiautomator();
//!# capabilities.udid("emulator-5554");
//!# capabilities.app("/apps/sample.apk");
//!# capabilities.app_wait_activity("com.example.AppActivity");
//!#
//! // create the client
//! let client = ClientBuilder::native(capabilities)
//!     .connect("http://localhost:4723/wd/hub/")
//!     .await?;
//!
//! // wait until element appears
//! let element = client
//!     .appium_wait()
//!     .for_element(By::accessibility_id("Click this"))
//!     .await?;
//!
//! element.click().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Commands
//! If you want to rotate the emulator's screen, or send keys, or do some other things supported by Appium,
//! then you can use features implemented in [commands] module.
//!
//! Those commands should be available to you depending whether you created [AndroidCapabilities] or [IOSCapabilities].
//!
//! If you wish to issue a custom command (not implemented by this lib), then use `issue_command(Custom)`.
//!
//! ```no_run
//! use http::Method;
//! use serde_json::json;
//! use appium_client::capabilities::android::AndroidCapabilities;
//! use appium_client::capabilities::{AppCapable, UdidCapable, UiAutomator2AppCompatible};
//! use appium_client::ClientBuilder;
//! use appium_client::commands::AppiumCommand;
//! use appium_client::commands::keyboard::HidesKeyboard;
//! use appium_client::find::{AppiumFind, By};
//!
//!# #[tokio::main]
//!# async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // create capabilities
//! let mut capabilities = AndroidCapabilities::new_uiautomator();
//! capabilities.udid("emulator-5554");
//! capabilities.app("/apps/sample.apk");
//! capabilities.app_wait_activity("com.example.AppActivity");
//!
//! let client = ClientBuilder::native(capabilities)
//!    .connect("http://localhost:4723/wd/hub/")
//!    .await?;
//!
//! // this feature is implemented in commands by this lib
//! client.hide_keyboard().await?;
//!
//! // use some quirky feature of Appium (not implemented in commands module)
//! // you can issue_cmd if you see that I didn't implement something
//! client.issue_cmd(AppiumCommand::Custom(
//!     Method::POST,
//!     "quirky_feature".to_string(),
//!     Some(json!({
//!         "tap": "everywhere"
//!     }))
//! )).await?;
//!
//!#     Ok(())
//!# }
//! ```
//!
//! ## More documentation
//!
//! See the [readme](https://github.com/multicatch/appium-client/blob/master/README.md) or [examples](https://github.com/multicatch/appium-client/tree/master/examples)
//! to learn how to use this library.

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use fantoccini::error;
use http::Method;
use hyper::client::connect;
use log::error;
use tokio::spawn;
use crate::capabilities::android::AndroidCapabilities;
use crate::capabilities::AppiumCapability;
use crate::capabilities::ios::IOSCapabilities;
use crate::commands::AppiumCommand;

pub mod capabilities;
pub mod commands;
pub mod find;
pub mod wait;

/// Client builder
///
/// Use this struct to build Appium client.
/// This struct has methods that will guide you through all necessary things needed to construct a client.
///
/// Do not create an instance of [Client] yourself, use this builder.
pub struct ClientBuilder<C, Caps>
    where
        C: connect::Connect + Send + Sync + Clone + Unpin,
        Caps: AppiumCapability
{
    fantoccini_builder: fantoccini::ClientBuilder<C>,
    caps: PhantomData<Caps>,
}

#[cfg(feature = "native-tls")]
impl<Caps> ClientBuilder<hyper_tls::HttpsConnector<hyper::client::HttpConnector>, Caps>
    where Caps: AppiumCapability
{
    pub fn native(capabilities: Caps) -> ClientBuilder<hyper_tls::HttpsConnector<hyper::client::HttpConnector>, Caps> {
        ClientBuilder::new(fantoccini::ClientBuilder::native(), capabilities)
    }
}

#[cfg(feature = "rustls-tls")]
impl<Caps> ClientBuilder<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>, Caps>
    where Caps: AppiumCapability
{
    pub fn rustls(capabilities: Caps) -> ClientBuilder<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>, Caps> {
        ClientBuilder::new(fantoccini::ClientBuilder::rustls(), capabilities)
    }
}

impl<C, Caps> ClientBuilder<C, Caps>
    where
        C: connect::Connect + Send + Sync + Clone + Unpin + 'static,
        Caps: AppiumCapability
{
    pub fn new(mut builder: fantoccini::ClientBuilder<C>, capabilities: Caps) -> ClientBuilder<C, Caps> {
        builder.capabilities(capabilities.clone());

        ClientBuilder {
            fantoccini_builder: builder,
            caps: PhantomData,
        }
    }

    pub async fn connect(&self, webdriver: &str) -> Result<Client<Caps>, error::NewSessionError> {
        let inner = self.fantoccini_builder.connect(webdriver).await?;
        Ok(Client {
            inner,
            caps: PhantomData,
        })
    }
}

/// Generic Appium client
///
/// This client represents an Appium client that will connect to an Appium server
/// and send command to said server.
///
/// Depending on chosen capabilities ([AppiumCapability]), the client will have different traits.
/// Which means - different available features.
///
/// Check out [AndroidClient] and [IOSClient] in docs to see their features (available commands).
///
/// **Note**: [Client] automatically ends Appium session on drop (end of lifetime). This is the only way to end session.
pub struct Client<Caps>
    where Caps: AppiumCapability {
    inner: fantoccini::Client,
    caps: PhantomData<Caps>,
}

pub trait AppiumClientTrait: DerefMut<Target=fantoccini::Client> {}

/// Client used to automate Android testing
///
/// To create [AndroidClient], you need to use [ClientBuilder] and [AndroidCapabilities].
/// Rust type system will automatically pick up that by using those capabilities, you mean to control an Android device.
///
/// See trait implementations to check available features (commands) of this client.
///
/// ```no_run
/// use appium_client::capabilities::{AppCapable, UdidCapable, UiAutomator2AppCompatible};
/// use appium_client::capabilities::android::AndroidCapabilities;
/// use appium_client::ClientBuilder;
///
///# #[tokio::main]
///# async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut capabilities = AndroidCapabilities::new_uiautomator();
/// capabilities.udid("emulator-5554");
/// capabilities.app("/apps/sample.apk");
/// capabilities.app_wait_activity("com.example.AppActivity");
///
/// let client = ClientBuilder::native(capabilities)
///    .connect("http://localhost:4723/wd/hub/")
///    .await?;
///
/// // congratulations, you have successfully created an AndroidClient
/// # Ok(())
/// # }
/// ```
pub type AndroidClient = Client<AndroidCapabilities>;

/// Client used to automate iOS testing
///
/// To create [IOSClient], you need to use [ClientBuilder] and [IOSCapabilities].
/// Rust type system will automatically pick up that by using those capabilities, you mean to control an iOS device.
///
/// See trait implementations to check available features (commands) of this client.
///
/// ```no_run
/// use appium_client::capabilities::{AppCapable, UdidCapable};
/// use appium_client::capabilities::ios::IOSCapabilities;
/// use appium_client::ClientBuilder;
///
///# #[tokio::main]
///# async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut capabilities = IOSCapabilities::new_xcui();
/// capabilities.udid("000011114567899");
/// capabilities.app("/apps/sample.ipa");
///
/// let client = ClientBuilder::native(capabilities)
///    .connect("http://localhost:4723/wd/hub/")
///    .await?;
///
/// // congratulations, you have successfully created an IOSClient
/// # Ok(())
/// # }
/// ```
pub type IOSClient = Client<IOSCapabilities>;

impl<Caps> AppiumClientTrait for Client<Caps>
    where Caps: AppiumCapability {}

impl<Caps> Deref for Client<Caps>
    where Caps: AppiumCapability
{
    type Target = fantoccini::Client;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<Caps> DerefMut for Client<Caps>
    where Caps: AppiumCapability
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<Caps> Drop for Client<Caps>
    where Caps: AppiumCapability {
    fn drop(&mut self) {
        let client = Arc::new(self.inner.clone());
        spawn(async move {
            let client = client.deref().clone();
            // end session
            if let Err(e) = client.issue_cmd(AppiumCommand::Custom(
                Method::DELETE,
                "".to_string(),
                None
            )).await {
                error!("Error while ending session: {e}");
            }

            // clean up fantoccini
            if let Err(e) = client.close().await {
                error!("Error while issuing shutdown: {e}");
            };
        });
    }
}