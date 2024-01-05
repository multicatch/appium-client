//! Network management
use async_trait::async_trait;
use bitflags::bitflags;
use fantoccini::error::CmdError;
use http::Method;
use serde_json::json;
use crate::{AndroidClient, AppiumClientTrait};
use crate::commands::AppiumCommand;

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ConnectionState: u16 {
        const AIRPLANE_MODE_MASK = 0b001;
        const WIFI_MASK = 0b010;
        const DATA_MASK = 0b100;
    }
}

/// Check network status (wifi, mobile data, airplane, or change status in emulator)
#[async_trait]
pub trait HasNetworkState: AppiumClientTrait {
    async fn set_connection(&self, state: &ConnectionState) -> Result<(), CmdError> {
        let bitmask = state.bits();

        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "network_connection".to_string(),
            Some(json!({
                "name": "network_connection",
                "parameters": {
                    "type": bitmask
                }
            })),
        )).await?;

        Ok(())
    }

    async fn get_connection(&self) -> Result<ConnectionState, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::GET,
            "network_connection".to_string(),
            None,
        )).await?;

        let bits: u16 = serde_json::from_value(value)?;

        Ok(ConnectionState::from_bits_truncate(bits))
    }
}

#[async_trait]
impl HasNetworkState for AndroidClient {}

/// Toggle network status
#[async_trait]
pub trait SupportsNetworkStateManagement: AppiumClientTrait {

    async fn toggle_wifi(&self) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/toggle_wifi".to_string(),
            None,
        )).await?;

        Ok(())
    }

    async fn toggle_airplane_mode(&self) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/toggle_airplane_mode".to_string(),
            None,
        )).await?;

        Ok(())
    }

    async fn toggle_data(&self) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/toggle_data".to_string(),
            None,
        )).await?;

        Ok(())
    }
}

#[async_trait]
impl SupportsNetworkStateManagement for AndroidClient {}