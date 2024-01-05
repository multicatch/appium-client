//! Lock and unlock the device
use async_trait::async_trait;
use fantoccini::error::CmdError;
use http::Method;
use serde_json::json;
use crate::{AndroidClient, AppiumClientTrait, IOSClient};
use crate::commands::AppiumCommand;

/// Lock device
#[async_trait]
pub trait LocksDevice: AppiumClientTrait {

    /// Locks the device. Note: iOS can only be unlocked manually.
    async fn lock_device(&self) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/lock".to_string(),
            Some(json!({}))
        )).await?;

        Ok(())
    }
}

/// Unlock device
#[async_trait]
pub trait UnlocksDevice: AppiumClientTrait {

    /// Unlocks the device.
    ///
    /// To unlock a screen with code or pattern, use "unlockType" and "unlockKey" capabilities.
    /// See <https://github.com/appium/appium-android-driver/blob/master/docs/UNLOCK.md>.
    async fn unlock_device(&self) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/unlock".to_string(),
                None
        )).await?;

        Ok(())
    }
}

#[async_trait]
impl LocksDevice for AndroidClient {}

#[async_trait]
impl LocksDevice for IOSClient {}

#[async_trait]
impl UnlocksDevice for AndroidClient {}