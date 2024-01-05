//! Screen recording
use std::collections::HashMap;
use std::time::Duration;
use async_trait::async_trait;
use fantoccini::error::CmdError;
use http::Method;
use serde_derive::Serialize;
use serde_json::{Error, json, Value};
use crate::{AndroidClient, AppiumClientTrait, IOSClient};
use crate::commands::AppiumCommand;

#[derive(Clone, Debug)]
pub struct ScreenRecordingUploadOptions {
    /// Path to the remote location, where the resulting video should be uploaded.
    pub remote_path: Option<String>,
    /// Credentials for remote ftp/http authentication (if needed).
    pub credentials: Option<ScreenRecordingCredentials>,
    /// Method name for http(s) upload. PUT is used by default. This option only has an effect if remotePath is provided.
    pub method: Option<Method>,
    /// Form field name containing the binary payload in multipart/form-data requests.
    pub file_field_name: Option<String>,
    /// Additional headers in multipart/form-data requests.
    pub headers: Option<HashMap<String, String>>,
    /// Additional form fields in multipart/form-data requests.
    pub form_fields: Option<HashMap<String, Value>>,
}

#[derive(Clone, Debug)]
pub struct ScreenRecordingCredentials {
    pub user: String,
    pub pass: String,
}

impl ScreenRecordingUploadOptions {
    pub fn empty() -> ScreenRecordingUploadOptions {
        Self {
            remote_path: None,
            credentials: None,
            method: None,
            file_field_name: None,
            headers: None,
            form_fields: None,
        }
    }

    pub fn to_map(self) -> Result<HashMap<String, Value>, Error> {
        let mut result = HashMap::new();
        if let Some(remote_path) = self.remote_path {
            result.insert("remotePath".to_string(), Value::String(remote_path));
        }
        if let Some(credentials) = self.credentials {
            result.insert("user".to_string(), Value::String(credentials.user));
            result.insert("pass".to_string(), Value::String(credentials.pass));
        }
        if let Some(method) = self.method {
            result.insert("method".to_string(), Value::String(method.to_string()));
        }
        if let Some(file_field_name) = self.file_field_name {
            result.insert("fileFieldName".to_string(), Value::String(file_field_name));
        }
        if let Some(form_fields) = self.form_fields {
            result.insert("formFields".to_string(), serde_json::to_value(form_fields)?);
        }
        if let Some(headers) = self.headers {
            result.insert("headers".to_string(), serde_json::to_value(headers)?);
        }
        Ok(result)
    }
}

/// Record screen
#[async_trait]
pub trait CanRecordScreen: AppiumClientTrait {
    async fn start_recording_screen(&self) -> Result<String, CmdError> {
        self.start_recording_with_options(None, None, HashMap::new()).await
    }

    async fn start_recording_with_options(&self, force_restart: Option<bool>, time_limit: Option<Duration>, mut options: HashMap<String, Value>) -> Result<String, CmdError> {
        if let Some(force_restart) = force_restart {
            options.insert("forceRestart".to_string(), Value::Bool(force_restart));
        }
        if let Some(time_limit) = time_limit {
            options.insert("timeLimit".to_string(), Value::Number(time_limit.as_secs().into()));
        }

        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/start_recording_screen".to_string(),
            Some(json!({
                "options": options
            })),
        )).await?;

        Ok(serde_json::from_value(value)?)
    }

    async fn stop_recording_screen(&self) -> Result<String, CmdError> {
        self.stop_recording_with_options(HashMap::new()).await
    }

    async fn stop_recording_with_options(&self, options: HashMap<String, Value>) -> Result<String, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/stop_recording_screen".to_string(),
            Some(json!({
                "options": options
            })),
        )).await?;

        Ok(serde_json::from_value(value)?)
    }
}

#[async_trait]
impl CanRecordScreen for AndroidClient {}

#[async_trait]
impl CanRecordScreen for IOSClient {}

/// Record screen with Android-specific encoding options
#[async_trait]
pub trait AndroidCanRecordScreen: CanRecordScreen {
    /// Starts screen recording (Android).
    ///
    /// **bit_rate** - The video bit rate for the video, in megabits per second.
    /// The default value is 4000000 (4 Mb/s) for Android API level below 27 and 20000000 (20 Mb/s) for API level 27 and above.
    ///
    /// **vide_size** - The video size of the generated media file. The format is WIDTHxHEIGHT.
    /// The default value is the device's native display resolution (if supported), 1280x720 if not.
    ///
    /// **bug_report** - Makes the recorder to display an additional information on the video overlay,
    /// such as a timestamp, that is helpful in videos captured to illustrate bugs.
    /// This option is only supported since API level 27 (Android P).
    async fn start_recording(&self,
                             bit_rate: Option<u32>,
                             video_size: Option<String>,
                             bug_report: Option<bool>,
                             force_restart: Option<bool>,
                             time_limit: Option<Duration>,
                             options: ScreenRecordingUploadOptions
    ) -> Result<String, CmdError> {
        let mut options = options.to_map()?;
        if let Some(bit_rate) = bit_rate {
            options.insert("bitRate".to_string(), Value::Number(bit_rate.into()));
        }
        if let Some(video_size) = video_size {
            options.insert("videoSize".to_string(), Value::String(video_size));
        }
        if let Some(bug_report) = bug_report {
            options.insert("bugReport".to_string(), Value::Bool(bug_report));
        }
        self.start_recording_with_options(force_restart, time_limit, options).await
    }

    async fn stop_recording(&self, options: ScreenRecordingUploadOptions) -> Result<String, CmdError> {
        self.stop_recording_with_options(options.to_map()?).await
    }
}

#[async_trait]
impl AndroidCanRecordScreen for AndroidClient {}

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IOSVideoQuality {
    Low,
    Medium,
    High,
    Photo,
}

/// Record screen with iOS-specific encoding options
#[async_trait]
pub trait IOSCanRecordScreen : CanRecordScreen {
    /// Starts screen recording (Android).
    ///
    /// **video_codec** - ffmpeg video codec type used for encoding of the recorded screen capture (see `ffmpeg -codecs`).
    ///
    /// **video_quality** - Quality of video encoding. Only works for real devices.
    ///
    /// **fps** - The Frames Per Second rate of the recorded video (1..60). Defaults to 10.
    ///
    /// **video_scale** - ffmpeg video scaling, none by default (<https://trac.ffmpeg.org/wiki/Scaling>).
    ///
    /// **video_filters** - ffmpeg video filters (eg. `transpose=1`, <https://ffmpeg.org/ffmpeg-filters.html>).
    async fn start_recording(&self,
                             video_codec: Option<String>,
                             video_quality: Option<IOSVideoQuality>,
                             fps: Option<u8>,
                             video_scale: Option<String>,
                             video_filters: Option<String>,
                             force_restart: Option<bool>,
                             time_limit: Option<Duration>,
                             options: ScreenRecordingUploadOptions
    ) -> Result<String, CmdError> {
        let mut options = options.to_map()?;
        if let Some(video_codec) = video_codec {
            options.insert("videoType".to_string(), Value::String(video_codec));
        }
        if let Some(video_quality) = video_quality {
            options.insert("videoQuality".to_string(), serde_json::to_value(video_quality)?);
        }
        if let Some(fps) = fps {
            if !(1..=60).contains(&fps) {
                return Err(CmdError::InvalidArgument(
                    "fps".to_string(),
                    format!("{fps} should be between 1 and 60.")
                ))
            }
            options.insert("videoFps".to_string(), Value::Number(fps.into()));
        }
        if let Some(video_scale) = video_scale {
            options.insert("videoScale".to_string(), Value::String(video_scale));
        }
        if let Some(video_filters) = video_filters {
            options.insert("videoFilters".to_string(), Value::String(video_filters));
        }
        self.start_recording_with_options(force_restart, time_limit, options).await
    }

    async fn stop_recording(&self, options: ScreenRecordingUploadOptions) -> Result<String, CmdError> {
        self.stop_recording_with_options(options.to_map()?).await
    }
}

#[async_trait]
impl IOSCanRecordScreen for IOSClient {}