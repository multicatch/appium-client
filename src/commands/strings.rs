use std::collections::HashMap;
use async_trait::async_trait;
use fantoccini::error::CmdError;
use http::Method;
use serde_json::json;
use crate::{AndroidClient, AppiumClientTrait, IOSClient};
use crate::commands::AppiumCommand;

#[async_trait]
pub trait HasAppStrings : AppiumClientTrait {
    async fn app_strings_default_lang(&self) -> Result<HashMap<String, String>, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/app/strings".to_string(),
            None
        )).await?;

        Ok(serde_json::from_value(value)?)
    }

    async fn app_strings(&self, lang: &str) -> Result<HashMap<String, String>, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/app/strings".to_string(),
            Some(json!({
                "language": lang
            }))
        )).await?;

        Ok(serde_json::from_value(value)?)
    }

    async fn app_strings_from_file(&self, lang: &str, file: &str) -> Result<HashMap<String, String>, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/app/strings".to_string(),
            Some(json!({
                "language": lang,
                "stringFile": file
            }))
        )).await?;

        Ok(serde_json::from_value(value)?)
    }
}

#[async_trait]
impl HasAppStrings for AndroidClient {}

#[async_trait]
impl HasAppStrings for IOSClient {}