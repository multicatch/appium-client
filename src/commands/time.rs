//! Device time
use async_trait::async_trait;
use fantoccini::error::CmdError;
use http::Method;
use serde_json::json;
use crate::{AndroidClient, AppiumClientTrait, IOSClient};
use crate::commands::AppiumCommand;

/// Get device time
#[async_trait]
pub trait HasDeviceTime : AppiumClientTrait {

    /// Gets device date and time for both iOS (host time is returned for simulators) and Android devices.
    /// The default format since Appium 1.8.2 is `YYYY-MM-DDTHH:mm:ssZ`, which complies to ISO-8601.
    async fn device_time(&self) -> Result<String, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::GET,
            "appium/device/system_time".to_string(),
            None
        )).await?;

        Ok(serde_json::from_value(value)?)
    }

    /// Gets device date and time for both iOS and Android devices with given format.
    ///
    /// See <https://momentjs.com/docs/> to get the full list of supported datetime format specifiers.
    /// The default format is `YYYY-MM-DDTHH:mm:ssZ`, which complies to ISO-8601.
    async fn device_time_with_format(&self, format: &str) -> Result<String, CmdError> {
        let value = self.execute("mobile: getDeviceTime", vec![
            json!({"format": format})
        ]).await?;

        Ok(serde_json::from_value(value)?)
    }
}

#[async_trait]
impl HasDeviceTime for AndroidClient {}

#[async_trait]
impl HasDeviceTime for IOSClient {}