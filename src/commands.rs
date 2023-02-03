use fantoccini::wd::WebDriverCompatibleCommand;
use http::Method;
use serde::Serializer;
use serde_derive::Serialize;

#[derive(Debug, PartialEq)]
pub enum AppiumCommand {
    FindElement(AppiumBy),
    FindElements(AppiumBy),
}

#[derive(Debug, PartialEq, Clone)]
pub enum AppiumBy {
    AiAutomator(LocatorParameters),
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct LocatorParameters {
    pub using: String,
    pub value: String,
}

impl AppiumBy {
    pub fn uiautomator(query: &str) -> AppiumBy {
        AppiumBy::AiAutomator(LocatorParameters {
            using: "-android uiautomator".to_string(),
            value: query.to_string(),
        })
    }
}

impl serde::Serialize for AppiumBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            AppiumBy::AiAutomator(params) => params.serialize(serializer),
        }
    }
}

impl WebDriverCompatibleCommand for AppiumCommand {
    fn endpoint(
        &self,
        base_url: &url::Url,
        session_id: Option<&str>,
    ) -> Result<url::Url, url::ParseError> {

        let base = { base_url.join(&format!("session/{}/", session_id.as_ref().unwrap()))? };
        match self {
            AppiumCommand::FindElement(..) => base.join("element"),
            AppiumCommand::FindElements(..) => base.join("elements"),
        }
    }

    fn method_and_body(&self, _request_url: &url::Url) -> (Method, Option<String>) {
        match self {
            AppiumCommand::FindElement(by)
            | AppiumCommand::FindElements(by) => {
                let method = Method::POST;
                let body = Some(serde_json::to_string(&by).unwrap());

                (method, body)
            }
        }
    }

    fn is_new_session(&self) -> bool {
        false
    }

    fn is_legacy(&self) -> bool {
        false
    }
}