use async_trait::async_trait;
use fantoccini::error::CmdError;
use http::Method;
use serde_json::json;
use crate::{AndroidClient, AppiumClientTrait};
use crate::commands::AppiumCommand;

pub struct AndroidActivity {
    pub app_package: String,
    pub app_activity: String,
    pub app_wait_package: String,
    pub app_wait_activity: String,
    pub intent_action: String,
    pub intent_category: String,
    pub intent_flags: String,
    pub optional_intent_arguments: String,
    pub stop_app: bool,
}

#[async_trait]
pub trait StartsActivity: AppiumClientTrait {
    async fn start_activity(&self, activity: AndroidActivity) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/start_activity".to_string(),
            Some(json!({
                "appPackage": activity.app_package,
                "appWaitPackage": activity.app_wait_package,
                "appWaitActivity": activity.app_wait_activity,
                "dontStopAppOnReset": !activity.stop_app,
                "intentAction": activity.intent_action,
                "intentCategory": activity.intent_category,
                "intentFlags": activity.intent_flags,
                "optionalIntentArguments": activity.optional_intent_arguments
            }))
        )).await?;

        Ok(())
    }

    async fn current_activity(&self) -> Result<String, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/current_activity".to_string(),
            None
        )).await?;

        Ok(serde_json::from_value(value)?)
    }

    async fn current_package(&self) -> Result<String, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/current_package".to_string(),
            None
        )).await?;

        Ok(serde_json::from_value(value)?)
    }
}

#[async_trait]
impl StartsActivity for AndroidClient {}