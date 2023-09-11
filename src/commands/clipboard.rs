use async_trait::async_trait;
use base64::Engine;
use base64::engine::general_purpose;
use fantoccini::error::CmdError;
use http::Method;
use serde_derive::Serialize;
use serde_json::json;

use crate::{AndroidClient, AppiumClientTrait, IOSClient};
use crate::commands::AppiumCommand;

#[derive(Copy, Clone, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ClipboardContentType {
    PlainText,
    Image,
    URL,
}

#[async_trait]
pub trait HasClipboard: AppiumClientTrait {
    async fn get_clipboard(&self, content_type: ClipboardContentType) -> Result<Vec<u8>, CmdError> {
        let value = self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/get_clipboard".to_string(),
            Some(json!({
                "contentType": content_type
            })),
        )).await?;

        let base64: String = serde_json::from_value::<String>(value)?
            .replace('\n', "");

        Ok(general_purpose::STANDARD.decode(base64)
            .map_err(|e| CmdError::NotJson(format!("{e}")))?)
    }

    async fn set_clipboard<CT>(&self, content_type: ClipboardContentType, content: CT) -> Result<(), CmdError>
        where CT: AsRef<[u8]> + Send
    {
        let content = general_purpose::STANDARD.encode(content);

        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/set_clipboard".to_string(),
            Some(json!({
                "contentType": content_type,
                "content": content
            })),
        )).await?;

        Ok(())
    }

    async fn set_clipboard_text<CT>(&self, content: CT) -> Result<(), CmdError>
        where CT: AsRef<[u8]> + Send
    {
        self.set_clipboard(ClipboardContentType::PlainText, content).await
    }

    async fn get_clipboard_text(&self) -> Result<String, CmdError> {
        let clipboard = self.get_clipboard(ClipboardContentType::PlainText).await?;
        Ok(String::from_utf8(clipboard)
            .map_err(|e| CmdError::NotJson(format!("{e}")))?)
    }
}

#[async_trait]
impl HasClipboard for AndroidClient {}

#[async_trait]
impl HasClipboard for IOSClient {}

#[async_trait]
pub trait HasAndroidClipboard: HasClipboard {
    async fn set_clipboard_labeled<CT>(&self, label: &str, content_type: ClipboardContentType, content: CT) -> Result<(), CmdError>
        where CT: AsRef<[u8]> + Send
    {
        let content = general_purpose::STANDARD.encode(content);

        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            "appium/device/set_clipboard".to_string(),
            Some(json!({
                "label": label,
                "contentType": content_type,
                "content": content
            })),
        )).await?;

        Ok(())
    }

    async fn set_clipboard_text_labeled<CT>(&self, label: &str, content: CT) -> Result<(), CmdError>
        where CT: AsRef<[u8]> + Send {
        self.set_clipboard_labeled(label, ClipboardContentType::PlainText, content).await
    }
}

#[async_trait]
impl HasAndroidClipboard for AndroidClient {}