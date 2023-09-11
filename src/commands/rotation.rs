use std::fmt::{Display, Formatter};
use async_trait::async_trait;
use fantoccini::error::CmdError;
use http::Method;
use serde::Deserialize;
use serde_derive::Serialize;
use serde_json::{json, Map, Value};
use crate::{AndroidClient, AppiumClientTrait, IOSClient};
use crate::commands::AppiumCommand;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Orientation {
    Landscape,
    Portrait,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize)]
pub struct DeviceRotation {
    x: u16,
    y: u16,
    z: u16,
}

impl DeviceRotation {
    pub fn new(x: u16, y: u16, z: u16) -> Result<DeviceRotation, CmdError> {
        for (name, value) in [("x", x), ("y", y), ("z", z)] {
            if value >= 360 {
                return Err(CmdError::InvalidArgument(
                    name.to_string(),
                    format!("{value} should be less than 360 deg.")
                ))
            }
        }

        Ok(DeviceRotation {
            x, y, z
        })
    }
}

#[async_trait]
pub trait SupportsRotation : AppiumClientTrait {
    async fn orientation(&self) -> Result<Orientation, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::GET, "orientation".to_string(), None
        )).await?;
        Ok(serde_json::from_value(value.clone())?)
    }

    async fn set_orientation(&self, orientation: Orientation) -> Result<Orientation, CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST, "orientation".to_string(), Some(json!({
                "orientation": orientation
            }))
        )).await?;

        Ok(orientation)
    }

    async fn rotation(&self) -> Result<DeviceRotation, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(Method::GET, "rotation".to_string(), None)).await?;
        let rotation: DeviceRotation = serde_json::from_value(value.clone())?;
        Ok(rotation)
    }

    async fn set_rotation(&self, rotation: DeviceRotation) -> Result<DeviceRotation, CmdError> {
        let mut map: Map<String, Value> = Map::new();
        map.insert("x".to_string(), rotation.x.into());
        map.insert("y".to_string(), rotation.y.into());
        map.insert("z".to_string(), rotation.z.into());

        self.issue_cmd(AppiumCommand::Custom(
            Method::POST, "rotation".to_string(), Some(Value::Object(map))
        )).await?;

        Ok(rotation)
    }
}

#[async_trait]
impl SupportsRotation for AndroidClient {}

#[async_trait]
impl SupportsRotation for IOSClient {}