use async_trait::async_trait;
use base64::Engine;
use base64::engine::general_purpose;
use fantoccini::error::CmdError;
use http::Method;
use serde_json::json;
use crate::{AndroidClient, AppiumClientTrait, IOSClient};
use crate::commands::AppiumCommand;

#[async_trait]
pub trait PullsFiles : AppiumClientTrait{

    /// Pulls a single file from device
    async fn pull_file(&self, path: &str) -> Result<Vec<u8>, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/pull_file".to_string(),
            Some(json!({
                "path": path
            }))
        )).await?;

        let value: String = serde_json::from_value(value)?;

        Ok(general_purpose::STANDARD.decode(value)
            .map_err(|e| CmdError::NotJson(format!("{e}")))?)
    }

    /// Pulls folder and returns zip file containing the content
    async fn pull_folder(&self, path: &str) -> Result<Vec<u8>, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/pull_folder".to_string(),
            Some(json!({
                "path": path
            }))
        )).await?;

        let value: String = serde_json::from_value(value)?;

        Ok(general_purpose::STANDARD.decode(value)
            .map_err(|e| CmdError::NotJson(format!("{e}")))?)
    }
}

#[async_trait]
impl PullsFiles for AndroidClient {}

#[async_trait]
impl PullsFiles for IOSClient {}

#[async_trait]
pub trait PushesFiles : AppiumClientTrait {
    async fn push_file(&self, path: &str, data: &[u8]) -> Result<(), CmdError> {
        let data = general_purpose::STANDARD.encode(data);

        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/push_file".to_string(),
            Some(json!({
                "path": path,
                "data": data
            }))
        )).await?;

        Ok(())
    }
}


#[async_trait]
impl PushesFiles for AndroidClient {}

#[async_trait]
impl PushesFiles for IOSClient {}