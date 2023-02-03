use std::collections::HashMap;
use std::mem::transmute;
use std::time::Duration;
use fantoccini::elements::{Element, ElementRef};
use fantoccini::Client;
use fantoccini::error::CmdError;
use crate::commands::{AppiumBy, AppiumCommand};
use async_trait::async_trait;
use fantoccini::wait::Wait;
use tokio::time::Instant;

#[async_trait]
pub trait AppiumFindBy {
    async fn find_by(&self, search: AppiumBy) -> Result<Element, CmdError>;
    async fn find_all_by(&self, search: AppiumBy) -> Result<Vec<Element>, CmdError>;
}

#[async_trait]
impl AppiumFindBy for Client {
    async fn find_by(&self, search: AppiumBy) -> Result<Element, CmdError> {
        let value = self.issue_cmd(AppiumCommand::FindElement(search)).await?;
        let map: HashMap<String, String> = serde_json::from_value(value.clone())?;

        map.get("ELEMENT")
            .ok_or_else(|| CmdError::NotW3C(value))
            .map(|element| Element::from_element_id(
                self.clone(),
                ElementRef::from(element.clone())
            ))
    }

    async fn find_all_by(&self, search: AppiumBy) -> Result<Vec<Element>, CmdError> {
        let value = self.issue_cmd(AppiumCommand::FindElements(search)).await?;
        let result: Vec<HashMap<String, String>> = serde_json::from_value(value.clone())?;

        let elements = result.into_iter()
            .filter_map(|map| map.get("ELEMENT").cloned())
            .map(|element| Element::from_element_id(
                self.clone(),
                ElementRef::from(element.clone())
            ))
            .collect();

        Ok(elements)
    }
}

#[async_trait]
pub trait AppiumWaitFor {
    async fn for_appium_element(self, search: AppiumBy) -> Result<Element, CmdError>;
}

macro_rules! wait_on {
    ($self:ident, $ready:expr) => {{
        let start = Instant::now();
        loop {
            match $self.timeout {
                Some(timeout) if start.elapsed() > timeout => break Err(CmdError::WaitTimeout),
                _ => {}
            }
            match $ready? {
                Some(result) => break Ok(result),
                None => {
                    tokio::time::sleep($self.period).await;
                }
            };
        }
    }};
}

#[async_trait]
impl<'a> AppiumWaitFor for Wait<'a> {
    async fn for_appium_element(self, search: AppiumBy) -> Result<Element, CmdError> {
        let wait: UnsafeWait<'a> = unsafe {
            // this is the only way to handle timeouts with our custom trait
            transmute(self)
        };

        wait_on!(wait, {
            match wait.client.find_by(search.clone()).await {
                Ok(element) => Ok(Some(element)),
                Err(CmdError::NoSuchElement(_)) => Ok(None),
                Err(err) => Err(err),
            }
        })
    }
}

/// Struct used internally to transmute [Wait].
///
/// We really need to access private fields in [Wait] to properly handle timeouts with Appium selectors.
/// Unfortunately, this struct is implemented in a way that prevents this.
struct UnsafeWait<'c> {
    client: &'c Client,
    timeout: Option<Duration>,
    period: Duration,
}