use async_trait::async_trait;
use fantoccini::error::CmdError;
use http::Method;
use serde_json::json;
use crate::{AndroidClient, AppiumClientTrait, IOSClient};
use crate::commands::AppiumCommand;

#[async_trait]
pub trait HidesKeyboard: AppiumClientTrait {
    /// Tries to hide keyboard using default system mechanism.
    ///
    /// Note: On some devices, it defaults to "swipe" or "back" button.
    /// It unfortunately can cause side effects like going to the previous screen,
    /// or not hiding the keyboard at all in some apps.
    /// On iOS, the keyboard might not hide at all.
    ///
    /// In such cases, consider implementing your own "hide keyboard" with swipe or tap on screen.
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