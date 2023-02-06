use std::time::Duration;
use fantoccini::Client;
use fantoccini::elements::Element;
use fantoccini::error::CmdError;
use tokio::time::{Instant, interval};
use crate::commands::AppiumBy;
use crate::find::AppiumFindBy;

pub trait AppiumWait {
    fn appium_wait(&self) -> Wait;
}

impl AppiumWait for Client {
    fn appium_wait(&self) -> Wait {
        Wait {
            client: self,
            timeout: Duration::from_secs(30),
            period: Duration::from_millis(250),
        }
    }
}

#[derive(Debug)]
pub struct Wait<'c> {
    client: &'c Client,
    timeout: Duration,
    period: Duration,
}

impl Wait<'_> {
    /// Set the timeout until the operation should wait.
    pub fn at_most(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the period to delay checks.
    pub fn every(mut self, period: Duration) -> Self {
        self.period = period;
        self
    }

    /// Waits for element using Appium selector
    pub async fn for_element(self, search: AppiumBy) -> Result<Element, CmdError> {
        let wait_on_selector = AppiumWaitOnSelector::new(self, search);
        wait_on_selector.wait().await
    }
}

struct AppiumWaitOnSelector<'a> {
    wait: Wait<'a>,
    selector: AppiumBy,
}

impl AppiumWaitOnSelector<'_> {
    pub fn new(wait: Wait, selector: AppiumBy) -> AppiumWaitOnSelector {
        AppiumWaitOnSelector {
            wait,
            selector,
        }
    }

    pub async fn wait(self) -> Result<Element, CmdError> {
        let mut interval = interval(self.wait.period);
        let timeout = self.wait.timeout;

        let start = Instant::now();
        loop {
            if start.elapsed() > timeout {
                return Err(CmdError::WaitTimeout);
            }

            match find_element(&self.wait, self.selector.clone()).await? {
                Some(result) => return Ok(result),
                None => interval.tick().await,
            };
        }
    }

}

async fn find_element(wait: &Wait<'_>, selector: AppiumBy) -> Result<Option<Element>, CmdError> {
    match wait.client.find_by(selector).await {
        Ok(element) => Ok(Some(element)),
        Err(CmdError::NoSuchElement(_)) => Ok(None),
        Err(err) => Err(err),
    }
}