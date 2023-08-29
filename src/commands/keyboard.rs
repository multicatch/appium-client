use async_trait::async_trait;
use fantoccini::error::CmdError;
use http::Method;
use serde_json::json;
use crate::{AndroidClient, AppiumClientTrait, IOSClient};
use crate::commands::AppiumCommand;

#[async_trait]
pub trait HidesKeyboard: AppiumClientTrait {
    async fn hide_keyboard(&self) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            HIDE_KEYBOARD_ENDPOINT.to_string(),
            Some(json!({}))
        )).await?;
        Ok(())
    }
}


const HIDE_KEYBOARD_ENDPOINT: &str = "appium/device/hide_keyboard";

#[async_trait]
impl HidesKeyboard for AndroidClient {}

#[async_trait]
impl HidesKeyboard for IOSClient {}