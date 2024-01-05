//! Android-specific features
use std::collections::HashMap;
use async_trait::async_trait;
use fantoccini::elements::Element;
use fantoccini::error::CmdError;
use http::Method;
use serde_derive::Serialize;
use serde_repr::Serialize_repr;
use serde_json::{json, Value};
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

/// Start or check Android actitivies
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

/// Android device details
#[async_trait]
pub trait HasAndroidDeviceDetails :AppiumClientTrait {
    async fn display_density(&self) -> Result<u64, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::GET,
            "appium/device/display_density".to_string(),
            None
        )).await?;

        Ok(serde_json::from_value(value)?)
    }

    async fn system_bars(&self) -> Result<HashMap<String, HashMap<String, Value>>, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::GET,
            "appium/device/system_bars".to_string(),
            None
        )).await?;

        Ok(serde_json::from_value(value)?)
    }
}

#[async_trait]
impl HasAndroidDeviceDetails for AndroidClient {}

/// Device traits that Appium is able to read
#[async_trait]
pub trait HasSupportedPerformanceDataType : AppiumClientTrait {
    async fn supported_performance_data_type(&self) -> Result<Vec<String>, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/performanceData/types".to_string(),
            None
        )).await?;

        Ok(serde_json::from_value(value)?)
    }

    async fn performance_data(&self, package: &str, data_type: &str, read_timeout: u32) -> Result<Vec<Vec<Value>>, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/performanceData/types".to_string(),
            Some(json!({
                "packageName": package,
                "dataType": data_type,
                "dataReadTimeout": read_timeout
            }))
        )).await?;

        Ok(serde_json::from_value(value)?)
    }
}

#[async_trait]
impl HasSupportedPerformanceDataType for AndroidClient {}

#[derive(Debug, Serialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GsmCallAction {
    Call,
    Accept,
    Cancel,
    Hold,
}

#[derive(Debug, Serialize_repr, Eq, PartialEq)]
#[repr(u8)]
pub enum GsmSignalStrength {
    NoneOrUnknown = 0,
    Poor = 1,
    Moderate = 2,
    Good = 3,
    Great = 4,
}

#[derive(Debug, Serialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GsmVoiceState {
    ON,
    OFF,
    Denied,
    Searching,
    Roaming,
    Home,
    Unregistered,
}

#[derive(Debug, Serialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NetworkSpeed {
    GSM,
    SCSD,
    GPRS,
    EDGE,
    UMTS,
    HSDPA,
    LTE,
    EVDO,
    FULL,
}

/// Special Android emulator commands like "send SMS"
#[async_trait]
pub trait SupportsSpecialEmulatorCommands : AppiumClientTrait {

    async fn send_sms(&self, phone_number: &str, message: &str) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/send_sms".to_string(),
            Some(json!({
                "phoneNumber": phone_number,
                "message": message
            }))
        )).await?;

        Ok(())
    }

    async fn make_gsm_call(&self, phone_number: &str, action: GsmCallAction) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/gsm_call".to_string(),
            Some(json!({
                "phoneNumber": phone_number,
                "action": action
            }))
        )).await?;

        Ok(())
    }

    async fn set_signal_strength(&self, strength: GsmSignalStrength) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/gsm_signal".to_string(),
            Some(json!({
                "signalStrengh": strength,
                "signalStrength": strength
            }))
        )).await?;

        Ok(())
    }

    async fn set_gsm_voice_state(&self, state: GsmVoiceState) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/gsm_voice".to_string(),
            Some(json!({
                "state": state
            }))
        )).await?;

        Ok(())
    }

    async fn set_network_speed(&self, speed: NetworkSpeed) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/network_speed".to_string(),
            Some(json!({
                "netspeed": speed
            }))
        )).await?;

        Ok(())
    }

    async fn set_power_capacity(&self, percent: u8) -> Result<(), CmdError> {
        if percent > 100 {
            return Err(CmdError::InvalidArgument(
                "percent".to_string(),
                format!("{percent} should be between 0 and 100.")
            ))
        }

        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/power_capacity".to_string(),
            Some(json!({
                "percent": percent
            }))
        )).await?;

        Ok(())
    }

    async fn set_power_ac(&self, power: bool) -> Result<(), CmdError> {
        let state = if power {
            "on"
        } else {
            "off"
        };

        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/power_ac".to_string(),
            Some(json!({
                "state": state
            }))
        )).await?;

        Ok(())
    }
}

#[async_trait]
impl SupportsSpecialEmulatorCommands for AndroidClient {}

/// Chrome DevTools protocol commands (Chrome and webview)
#[async_trait]
pub trait ExecutesCDP : AppiumClientTrait {
    async fn execute_cdp_command(&self, command: &str, params: Option<HashMap<String, Value>>) -> Result<HashMap<String, Value>, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "goog/cdp/execute".to_string(),
            Some(json!({
                "cmd": command,
                "params": params
            }))
        )).await?;

        Ok(serde_json::from_value(value)?)
    }
}

#[async_trait]
impl ExecutesCDP for AndroidClient {}

/// Replacing element value (instead of retyping into field)
#[async_trait]
pub trait CanReplaceValue: AppiumClientTrait {
    async fn replace_value(&self, element: &Element, value: &str) -> Result<(), CmdError> {
        let id = element.element_id().to_string();
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            format!("appium/element/{}/replace_value", id),
            Some(json!({
                "id": id,
                "value": value
            }))
        )).await?;

        Ok(())
    }
}

#[async_trait]
impl CanReplaceValue for AndroidClient {}