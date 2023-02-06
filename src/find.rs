use std::collections::HashMap;
use fantoccini::elements::{Element, ElementRef};
use fantoccini::Client;
use fantoccini::error::CmdError;
use crate::commands::{AppiumBy, AppiumCommand};
use async_trait::async_trait;

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
