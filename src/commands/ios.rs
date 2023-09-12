use async_trait::async_trait;
use fantoccini::error::CmdError;
use http::Method;
use crate::AppiumClientTrait;
use crate::commands::AppiumCommand;

#[async_trait]
pub trait ShakesDevice : AppiumClientTrait {
    /// Simulate shaking the device.
    async fn shake(&self) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/shake".to_string(),
            None
        )).await?;

        Ok(())
    }
}