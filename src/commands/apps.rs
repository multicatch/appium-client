use std::time::Duration;
use async_trait::async_trait;
use fantoccini::error::CmdError;
use http::Method;
use serde_json::json;
use crate::{AndroidClient, AppiumClientTrait, IOSClient};
use crate::commands::AppiumCommand;

#[async_trait]
pub trait InteractsWithApps: AppiumClientTrait {
    async fn install_app(&self, path: &str) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/install_app".to_string(),
            Some(json!({
                "appPath": path
            })),
        )).await?;
        Ok(())
    }

    async fn is_app_installed(&self, bundle_id: &str) -> Result<bool, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/app_installed".to_string(),
            Some(json!({
                "bundleId": bundle_id
            })),
        )).await?;

        Ok(serde_json::from_value(value)?)
    }

    async fn run_app_in_background(&self, duration: Duration) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/app/background".to_string(),
            Some(json!({
                "seconds": duration.as_secs()
            })),
        )).await?;

        Ok(())
    }

    async fn remove_app(&self, bundle_id: &str) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/remove_app".to_string(),
            Some(json!({
                "bundleId": bundle_id
            })),
        )).await?;

        Ok(())
    }

    async fn activate_app(&self, bundle_id: &str) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/activate_app".to_string(),
            Some(json!({
                "bundleId": bundle_id
            })),
        )).await?;

        Ok(())
    }

    async fn app_state(&self, bundle_id: &str) -> Result<AppState, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/app_state".to_string(),
            Some(json!({
                "bundleId": bundle_id
            })),
        )).await?;

        Ok(serde_json::from_value(value)?)
    }

    async fn terminate_app(&self, bundle_id: &str) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/terminate_app".to_string(),
            Some(json!({
                "bundleId": bundle_id
            })),
        )).await?;

        Ok(())
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct AppState: u8 {
        const NOT_INSTALLED = 0;
        const NOT_RUNNING = 1;
        const RUNNING_IN_BACKGROUND_SUSPENDED = 2,
        const RUNNING_IN_BACKGROUND = 3;
        const RUNNING_IN_FOREGROUND = 4;
    }
}

#[async_trait]
impl InteractsWithApps for AndroidClient {}

#[async_trait]
impl InteractsWithApps for IOSClient {}