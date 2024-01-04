//! Find API for locating elements on screen
//!
//! This API can be used to find elements on screen or in a known parent.
//!
//! ## Basic usage
//! ```no_run
//!# use appium_client::capabilities::android::AndroidCapabilities;
//!# use appium_client::capabilities::{AppCapable, UdidCapable, UiAutomator2AppCompatible};
//!# use appium_client::ClientBuilder;
//!# use appium_client::find::{AppiumFind, By};
//!#
//!# #[tokio::main]
//!# async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // create capabilities & client
//! let mut capabilities = AndroidCapabilities::new_uiautomator();
//!# capabilities.udid("emulator-5554");
//!# capabilities.app("/apps/sample.apk");
//!# capabilities.app_wait_activity("com.example.AppActivity");
//!
//! let client = ClientBuilder::native(capabilities)
//!     .connect("http://localhost:4723/wd/hub/")
//!     .await?;
//!
//! // locate an element (find it)
//! let element = client
//!     .find_by(By::accessibility_id("Click this"))
//!     .await?;
//!
//! // locate all matching elements by given xpath
//! let elements = client
//!     .find_all_by(By::xpath("//*[contains(@resource-id, 'test')]"))
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! Notice that if you wish to get only one element (the first match), you can use [AppiumFind::find_by].
//! If you want all matches on a screen, you use [AppiumFind::find_all_by].
//!
//! ## Custom locator strategy
//!
//! If none of the options available in [By] work with your driver, then you might use [By::custom_kind] to specify custom location strategy (and search query).
//!
//! Using [By::custom_kind] is basically the same as using any other [By] variant, but you need to write more and the compiler won't tell you that you made a typo.
//!
//! Some Appium docs on the matter of locators (selectors): <https://appium.github.io/appium.io/docs/en/writing-running-appium/finding-elements/>
//!
//! Example:
//! ```no_run
//!# use appium_client::capabilities::android::AndroidCapabilities;
//!# use appium_client::capabilities::{AppCapable, UdidCapable, UiAutomator2AppCompatible};
//!# use appium_client::ClientBuilder;
//!# use appium_client::find::{AppiumFind, By};
//!#
//!# #[tokio::main]
//!# async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!# let mut capabilities = AndroidCapabilities::new_uiautomator();
//!# capabilities.udid("emulator-5554");
//!# capabilities.app("/apps/sample.apk");
//!# capabilities.app_wait_activity("com.example.AppActivity");
//!#
//!# let client = ClientBuilder::native(capabilities)
//!#     .connect("http://localhost:4723/wd/hub/")
//!#     .await?;
//!#
//! // locate an element (find it)
//! let element = client
//!     .find_by(By::accessibility_id("Find me"))
//!     .await?;
//!
//! // do the same, but more explicitly
//! let element = client
//!     .find_by(By::custom_kind("accessibility id", "Find me"))
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
use std::collections::HashMap;
use fantoccini::elements::{Element, ElementRef};
use fantoccini::Client;
use fantoccini::error::CmdError;
use serde::Serializer;
use serde_derive::Serialize;
use crate::commands::AppiumCommand;
use async_trait::async_trait;

/// Locators supported by Appium
///
/// If you wish to use your very own locator (e.g. something I didn't implement in this enum),
/// just use [By::CustomKind].
#[derive(Debug, PartialEq, Clone)]
pub enum By {
    Id(String),
    Name(String),
    Xpath(String),
    UiAutomator(String),
    AndroidDataMatcher(String),
    AndroidViewMatcher(String),
    AndroidViewTag(String),
    IosClassChain(String),
    IosNsPredicate(String),
    AccessibilityId(String),
    ClassName(String),
    Image(String),
    Custom(String),
    CustomKind(String, String)
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct LocatorParameters {
    pub using: String,
    pub value: String,
}

impl By {
    /// Native element identifier. resource-id for android; name for iOS.
    pub fn id(id: &str) -> By {
        By::Id(id.to_string())
    }

    /// Name of element.
    pub fn name(name: &str) -> By {
        By::Name(name.to_string())
    }

    /// Search the app XML source using xpath (not recommended, has performance issues).
    pub fn xpath(query: &str) -> By {
        By::UiAutomator(query.to_string())
    }

    /// Use the UI Automator API, in particular the UiSelector class to locate elements. (UiAutomator2 only).
    ///
    /// In Appium you send the Java code, as a string, to the server, which executes it in the application’s environment,
    /// returning the element or elements.
    pub fn uiautomator(query: &str) -> By {
        By::UiAutomator(query.to_string())
    }

    /// Locate an element using Espresso [DataMatcher](https://developer.android.com/reference/android/support/test/espresso/Espresso#ondata). (Espresso only)
    pub fn android_data_matcher(query: &str) -> By {
        By::AndroidDataMatcher(query.to_string())
    }

    /// Locate an element using Espresso [ViewMatcher](https://developer.android.com/reference/android/support/test/espresso/matcher/ViewMatchers). (Espresso only)
    pub fn android_view_matcher(query: &str) -> By {
        By::AndroidViewMatcher(query.to_string())
    }

    /// Locate an element by its [view tag](https://developer.android.com/reference/android/support/test/espresso/matcher/ViewMatchers.html#withTagValue%28org.hamcrest.Matcher%3Cjava.lang.Object%3E). (Espresso only)
    pub fn android_view_tag(query: &str) -> By {
        By::AndroidViewTag(query.to_string())
    }

    /// Locate an element by a [class chain](https://pavankovurru.github.io/Appium_Mobile_Automation_Framework/documents/README_IOS.html#ios-class-chain-strategy) - a faster, but less powerful alternative to XPath on iOS.
    pub fn ios_class_chain(query: &str) -> By {
        By::IosClassChain(query.to_string())
    }

    /// A string corresponding to a recursive element search using the [iOS Predicate](https://github.com/appium/appium-xcuitest-driver/blob/master/docs/ios/ios-predicate.md). (iOS 10.0 and above)
    pub fn ios_ns_predicate(query: &str) -> By {
        By::IosNsPredicate(query.to_string())
    }

    /// Read a unique identifier for a UI element.
    ///
    /// For XCUITest it is the element's accessibility-id attribute. For Android it is the element's content-desc attribute.
    pub fn accessibility_id(id: &str) -> By {
        By::AccessibilityId(id.to_string())
    }

    /// Locate element by its class name.
    ///
    /// For IOS it is the full name of the XCUI element and begins with XCUIElementType.
    /// For Android it is the full name of the UIAutomator2 class (e.g.: android.widget.TextView)
    pub fn class_name(class_name: &str) -> By {
        By::ClassName(class_name.to_string())
    }

    /// Locate an element by matching it with a base 64 encoded image file
    pub fn image(base64_template: &str) -> By {
        By::Image(base64_template.to_string())
    }

    /// Custom locator for use with plugins registered via the customFindModules capability.
    pub fn custom(query: &str) -> By {
        By::Custom(query.to_string())
    }

    /// A locator for non-standard locators
    ///
    /// You can define what type of locator to use, so you're free to use anything here.
    pub fn custom_kind(using: &str, value: &str) -> By {
        By::CustomKind(using.to_string(), value.to_string())
    }
}

impl From<By> for LocatorParameters {
    fn from(val: By) -> Self {
        let (using, value) = match val {
            By::Id(value) => ("id".to_string(), value),
            By::Name(value) => ("name".to_string(), value),
            By::Xpath(value) => ("xpath".to_string(), value),
            By::UiAutomator(value) => ("-android uiautomator".to_string(), value),
            By::AndroidDataMatcher(value) => ("-android datamatcher".to_string(), value),
            By::AndroidViewMatcher(value) => ("-android viewmatcher".to_string(), value),
            By::AndroidViewTag(value) => ("-android viewtag".to_string(), value),
            By::IosClassChain(value) => ("-ios class chain".to_string(), value),
            By::IosNsPredicate(value) => ("-ios predicate string".to_string(), value),
            By::AccessibilityId(value) => ("accessibility id".to_string(), value),
            By::Image(value) => ("-image".to_string(), value),
            By::ClassName(value) => ("class name".to_string(), value),
            By::Custom(value) => ("-custom".to_string(), value),
            By::CustomKind(kind, value) => (kind, value)
        };

        LocatorParameters {
            using,
            value
        }
    }
}

impl serde::Serialize for By {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let locator_params: LocatorParameters = self.clone().into();
        locator_params.serialize(serializer)
    }
}

#[async_trait]
pub trait AppiumFind {
    /// Locates an element by given strategy.
    async fn find_by(&self, search: By) -> Result<Element, CmdError>;

    /// Locates all elements matching criteria.
    async fn find_all_by(&self, search: By) -> Result<Vec<Element>, CmdError>;
}

#[async_trait]
impl AppiumFind for Client {
    async fn find_by(&self, search: By) -> Result<Element, CmdError> {
        let value = self.issue_cmd(AppiumCommand::FindElement(search)).await?;
        let map: HashMap<String, String> = serde_json::from_value(value.clone())?;

        map.get("ELEMENT")
            .ok_or_else(|| CmdError::NotW3C(value))
            .map(|element| Element::from_element_id(
                self.clone(),
                ElementRef::from(element.clone())
            ))
    }

    async fn find_all_by(&self, search: By) -> Result<Vec<Element>, CmdError> {
        let value = self.issue_cmd(AppiumCommand::FindElements(search)).await?;
        let result: Vec<HashMap<String, String>> = serde_json::from_value(value)?;

        let elements = result.into_iter()
            .filter_map(|map| map.get("ELEMENT").cloned())
            .map(|element| Element::from_element_id(
                self.clone(),
                ElementRef::from(element)
            ))
            .collect();

        Ok(elements)
    }
}

#[async_trait]
impl AppiumFind for Element {
    async fn find_by(&self, search: By) -> Result<Element, CmdError> {
        let client = self.clone().client();
        let element_ref = self.element_id();
        let value = client.issue_cmd(AppiumCommand::FindElementWithContext(search, element_ref.to_string())).await?;
        let map: HashMap<String, String> = serde_json::from_value(value.clone())?;

        map.get("ELEMENT")
            .ok_or_else(|| CmdError::NotW3C(value))
            .map(|element| Element::from_element_id(
                client,
                ElementRef::from(element.clone())
            ))
    }

    async fn find_all_by(&self, search: By) -> Result<Vec<Element>, CmdError> {
        let client = self.clone().client();
        let element_ref = self.element_id();
        let value = client.issue_cmd(AppiumCommand::FindElementsWithContext(search, element_ref.to_string())).await?;
        let result: Vec<HashMap<String, String>> = serde_json::from_value(value)?;

        let elements = result.into_iter()
            .filter_map(|map| map.get("ELEMENT").cloned())
            .map(|element| Element::from_element_id(
                client.clone(),
                ElementRef::from(element)
            ))
            .collect();

        Ok(elements)
    }
}