use async_trait::async_trait;
use fantoccini::error::CmdError;
use http::Method;
use serde_json::json;
use crate::{AndroidClient, AppiumClientTrait};
use crate::commands::AppiumCommand;

#[async_trait]
pub trait AuthenticatesByFinger : AppiumClientTrait {
    async fn use_finger_print(&self, id: u8) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/finger_print".to_string(),
            Some(json!({
                "fingerprintId": id
            }))
        )).await?;
        
        Ok(())
    }
}

#[async_trait]
impl AuthenticatesByFinger for AndroidClient {}