use std::collections::HashMap;
use fantoccini::elements::{Element, ElementRef};
use fantoccini::Client;
use fantoccini::error::CmdError;
use serde::Serializer;
use serde_derive::Serialize;
use crate::commands::AppiumCommand;
use async_trait::async_trait;

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
}

impl From<By> for LocatorParameters {
    fn from(val: By) -> Self {
        let (using, value) = match val {
            By::Id(value) => ("id", value),
            By::Name(value) => ("name", value),
            By::Xpath(value) => ("xpath", value),
            By::UiAutomator(value) => ("-android uiautomator", value),
            By::AndroidDataMatcher(value) => ("-android datamatcher", value),
            By::AndroidViewMatcher(value) => ("-android viewmatcher", value),
            By::AndroidViewTag(value) => ("-android viewtag", value),
            By::IosClassChain(value) => ("-ios class chain", value),
            By::IosNsPredicate(value) => ("-ios predicate string", value),
            By::AccessibilityId(value) => ("accessibility id", value),
            By::Image(value) => ("-image", value),
            By::ClassName(value) => ("class name", value),
            By::Custom(value) => ("-custom", value),
        };

        LocatorParameters {
            using: using.to_string(),
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