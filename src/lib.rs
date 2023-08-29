//! Rust client for Appium Server, for automated mobile app testing
//!
//! It is based on [fantoccini](https://github.com/jonhoo/fantoccini) and retains all capabilities
//! of fantoccini's client, such as screenshotting, touch actions, getting page source etc.
//!
//! This appium-client adds support for Appium locators such as iOS Class Chain, or UiAutomator.
//! See [find] for more info on Appium locators.
//!
//! Appium locators can be also waited on (just like you can wait for element with fantoccini),
//! see [wait] to learn how to wait.
//!
//! See the [readme](https://github.com/multicatch/appium-client/blob/master/README.md) or [examples](https://github.com/multicatch/appium-client/tree/master/examples)
//! to learn how to use this library.

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use fantoccini::error;
use hyper::client::connect;
use crate::capabilities::android::AndroidCapabilities;
use crate::capabilities::AppiumCapability;
use crate::capabilities::ios::IOSCapabilities;

pub mod capabilities;
pub mod commands;
pub mod find;
pub mod wait;

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

pub struct Client<Caps>
    where Caps: AppiumCapability {
    inner: fantoccini::Client,
    caps: PhantomData<Caps>,
}

pub trait AppiumClientTrait: DerefMut<Target=fantoccini::Client> {}

pub type AndroidClient = Client<AndroidCapabilities>;
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