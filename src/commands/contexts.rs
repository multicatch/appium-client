//! Context API (<https://appium.io/docs/en/2.1/guides/context/>)
use async_trait::async_trait;
use fantoccini::error::CmdError;
use http::Method;
use serde_json::json;
use crate::{AndroidClient, AppiumClientTrait, IOSClient};
use crate::commands::AppiumCommand;

/// Check current context or switch context (<https://appium.io/docs/en/2.1/guides/context/>)
#[async_trait]
pub trait SupportsContextSwitching: AppiumClientTrait {
    async fn set_context(&self, context: &str) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "context".to_string(),
            Some(json!({"name": context})),
        )).await?;
        Ok(())
    }

    async fn current_context(&self) -> Result<Option<String>, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::GET,
            "context".to_string(),
            None
        )).await?;

        let value: Option<String> = serde_json::from_value(value)?;
        Ok(value.and_then(|v| if v != "null" {
            Some(v)
        } else {
            None
        }))
    }

    async fn available_contexts(&self) -> Result<Vec<String>, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::GET,
            "contexts".to_string(),
            None
        )).await?;

        let value: Vec<String> = serde_json::from_value(value)?;
        Ok(value)
    }
}

#[async_trait]
impl SupportsContextSwitching for AndroidClient {}

#[async_trait]
impl SupportsContextSwitching for IOSClient {}