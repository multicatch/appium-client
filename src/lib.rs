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

pub mod capabilities;
pub mod commands;
pub mod find;
pub mod wait;

pub type ClientBuilder<C> = fantoccini::ClientBuilder<C>;
