//! Device authentication
use async_trait::async_trait;
use fantoccini::error::CmdError;
use http::Method;
use serde_json::json;
use crate::{AndroidClient, AppiumClientTrait, IOSClient};
use crate::commands::AppiumCommand;

/// Finger authentication (Android authentication)
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

/// TouchID (iPhone authentication)
#[async_trait]
pub trait PerformsTouchID : AppiumClientTrait {
    /// Simulate touchId event.
    async fn perform_touch_id(&self, successful_scan: bool) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/simulator/touch_id".to_string(),
            Some(json!({
                "match": successful_scan
            })),
        )).await?;
        Ok(())
    }

    /// Enrolls touchId in iOS Simulators. This call will only work if Appium process
    /// or its parent application (e.g. Terminal.app or Appium.app) has access to Mac OS accessibility
    /// in System Preferences > Security & Privacy > Privacy > Accessibility list.
    async fn toggle_touch_id_enrollment(&self, enabled: bool) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/simulator/toggle_touch_id_enrollment".to_string(),
            Some(json!({
                "enabled": enabled
            })),
        )).await?;
        Ok(())
    }
}

#[async_trait]
impl PerformsTouchID for IOSClient {}