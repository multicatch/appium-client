use async_trait::async_trait;
use fantoccini::error::CmdError;
use http::Method;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use crate::{AndroidClient, AppiumClientTrait, IOSClient};
use crate::commands::AppiumCommand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}

impl Location {
    pub fn new(latitude: f64, longitude: f64, altitude: f64) -> Location {
        Location {
            latitude, longitude, altitude
        }
    }
}

#[async_trait]
pub trait SupportsLocation : AppiumClientTrait {
    async fn location(&self) -> Result<Location, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::GET,
            "location".to_string(),
            None
        )).await?;

        Ok(serde_json::from_value(value)?)
    }

    /// Tries to set location if the driver/device supports it. Returns location of device after the attempt.
    ///
    /// Due to configuration or limitation of device, the location change may fail silently.
    /// The returned [Location] is the location that the device currently uses (or where is actually is).
    async fn set_location(&self, location: Location) -> Result<Location, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "location".to_string(),
            Some(json!({
                "location": location
            }))
        )).await?;

        Ok(serde_json::from_value(value)?)
    }
}

#[async_trait]
impl SupportsLocation for AndroidClient {}

#[async_trait]
impl SupportsLocation for IOSClient {}

#[derive(Clone, Debug, Serialize)]
pub struct AndroidGeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub satellites: u32,
    pub speed: f64,
}

impl AndroidGeoLocation {
    pub fn new(location: Location, satellites: u32, speed: f64) -> AndroidGeoLocation {
        AndroidGeoLocation {
            latitude: location.latitude,
            longitude: location.longitude,
            altitude: location.altitude,
            satellites,
            speed
        }
    }
}

#[async_trait]
pub trait SupportsAndroidLocation : AppiumClientTrait {
    async fn set_android_location(&self, location: AndroidGeoLocation) -> Result<Location, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "location".to_string(),
            Some(json!({
                "location": location
            }))
        )).await?;

        Ok(serde_json::from_value(value)?)
    }

}

#[async_trait]
impl SupportsAndroidLocation for AndroidClient {}