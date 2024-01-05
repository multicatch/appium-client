//! Battery info
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Deref;
use async_trait::async_trait;
use fantoccini::error::CmdError;
use serde_json::Value;
use crate::{AndroidClient, AppiumClientTrait, IOSClient};
use crate::capabilities::android::AndroidCapabilities;
use crate::capabilities::AppiumCapability;
use crate::capabilities::ios::IOSCapabilities;

/// Device battery level and state
#[async_trait]
pub trait HasBattery<Caps>: AppiumClientTrait
    where Caps: AppiumCapability
{
    async fn battery_info(&self) -> Result<BatteryInfo<Caps>, CmdError> {
        let value = self.execute("mobile: batteryInfo", vec![]).await?;
        Ok(BatteryInfo {
            inner: serde_json::from_value(value)?,
            caps: PhantomData,
        })
    }
}


#[async_trait]
impl HasBattery<AndroidCapabilities> for AndroidClient {}

#[async_trait]
impl HasBattery<IOSCapabilities> for IOSClient {}

pub struct BatteryInfo<Caps>
    where Caps: AppiumCapability {
    inner: HashMap<String, Value>,
    caps: PhantomData<Caps>,
}

impl<Caps> BatteryInfo<Caps>
    where Caps: AppiumCapability {

    pub fn level(&self) -> f64 {
        self.get("level")
            .cloned()
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or(0f64)
    }
}

impl<Caps> Deref for BatteryInfo<Caps>
    where Caps: AppiumCapability {
    type Target = HashMap<String, Value>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub trait CanBeCharged {
    fn is_full(&self) -> bool;
    fn is_charging(&self) -> bool;
    fn is_plugged(&self) -> bool;
    fn is_invalid(&self) -> bool;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AndroidBatteryState {
    Unknown,
    Charging,
    Discharging,
    NotCharging,
    Full,
}

impl BatteryInfo<AndroidCapabilities> {
    pub fn state(&self) -> AndroidBatteryState {
        self.get("state")
            .cloned()
            .and_then(|v| serde_json::from_value::<u32>(v).ok())
            .map(|state| match state {
                2 => AndroidBatteryState::Charging,
                3 => AndroidBatteryState::Discharging,
                4 => AndroidBatteryState::NotCharging,
                5 => AndroidBatteryState::Full,
                _ => AndroidBatteryState::Unknown
            })
            .unwrap_or(AndroidBatteryState::Unknown)
    }
}

impl CanBeCharged for BatteryInfo<AndroidCapabilities> {
    fn is_full(&self) -> bool {
        self.state() == AndroidBatteryState::Full
    }

    fn is_charging(&self) -> bool {
        self.state() == AndroidBatteryState::Charging
    }

    fn is_plugged(&self) -> bool {
        let state = self.state();
        state == AndroidBatteryState::NotCharging || state == AndroidBatteryState::Discharging
    }

    fn is_invalid(&self) -> bool {
        self.state() == AndroidBatteryState::Unknown
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IOSBatteryState {
    Unknown,
    Unplugged,
    Charging,
    Full,
}

impl BatteryInfo<IOSCapabilities> {
    pub fn state(&self) -> IOSBatteryState {
        self.get("state")
            .cloned()
            .and_then(|v| serde_json::from_value::<u32>(v).ok())
            .map(|state| match state {
                1 => IOSBatteryState::Unplugged,
                2 => IOSBatteryState::Charging,
                3 => IOSBatteryState::Full,
                _ => IOSBatteryState::Unknown
            })
            .unwrap_or(IOSBatteryState::Unknown)
    }
}

impl CanBeCharged for BatteryInfo<IOSCapabilities> {
    fn is_full(&self) -> bool {
        self.state() == IOSBatteryState::Full
    }

    fn is_charging(&self) -> bool {
        self.state() == IOSBatteryState::Charging
    }

    fn is_plugged(&self) -> bool {
        self.state() != IOSBatteryState::Unplugged
    }

    fn is_invalid(&self) -> bool {
        self.state() == IOSBatteryState::Unknown
    }
}