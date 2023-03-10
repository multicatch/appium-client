use fantoccini::wd::WebDriverCompatibleCommand;
use http::Method;
use crate::find::By;

#[derive(Debug, PartialEq)]
pub enum AppiumCommand {
    FindElement(By),
    FindElementWithContext(By, String),
    FindElements(By),
    FindElementsWithContext(By, String),
}

impl WebDriverCompatibleCommand for AppiumCommand {
    fn endpoint(
        &self,
        base_url: &url::Url,
        session_id: Option<&str>,
    ) -> Result<url::Url, url::ParseError> {
        let base = { base_url.join(&format!("session/{}/", session_id.as_ref().unwrap()))? };
        match self {
            AppiumCommand::FindElement(..) =>
                base.join("element"),
            AppiumCommand::FindElements(..) =>
                base.join("elements"),
            AppiumCommand::FindElementWithContext(.., context) =>
                base.join("element")
                    .and_then(|url| url.join(context))
                    .and_then(|url| url.join("element")),
            AppiumCommand::FindElementsWithContext(.., context) =>
                base.join("element")
                    .and_then(|url| url.join(context))
                    .and_then(|url| url.join("elements")),
        }
    }

    fn method_and_body(&self, _request_url: &url::Url) -> (Method, Option<String>) {
        match self {
            AppiumCommand::FindElement(by)
            | AppiumCommand::FindElements(by)
            | AppiumCommand::FindElementWithContext(by, ..)
            | AppiumCommand::FindElementsWithContext(by, ..) => {
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