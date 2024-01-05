//! Settings API (<https://appium.io/docs/en/2.1/guides/settings/>)
use std::collections::HashMap;
use async_trait::async_trait;
use fantoccini::error::CmdError;
use http::Method;
use serde_json::{json, Map, Value};
use crate::{AndroidClient, AppiumClientTrait, IOSClient};
use crate::commands::AppiumCommand;

/// Set or get setting from Settings API (<https://appium.io/docs/en/2.1/guides/settings/>)
#[async_trait]
pub trait HasSettings : AppiumClientTrait {
    async fn set_settings(&self, values: Map<String, Value>) -> Result<(), CmdError> {
        let value = Value::Object(values);
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/settings".to_string(),
            Some(json!({
                "settings": value
            }))
        )).await?;

        Ok(())
    }

    async fn set_setting(&self, name: &str, value: Value) -> Result<(), CmdError> {
        let mut map = Map::new();
        map.insert(name.to_string(), value);

        self.set_settings(map).await
    }

    async fn get_settings(&self) -> Result<HashMap<String, Value>, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::GET,
            "appium/settings".to_string(),
            None
        )).await?;

        Ok(serde_json::from_value(value)?)
    }
}

#[async_trait]
impl HasSettings for AndroidClient {}

#[async_trait]
impl HasSettings for IOSClient {}