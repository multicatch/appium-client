use std::time::Duration;
use fantoccini::Client;
use fantoccini::elements::Element;
use fantoccini::error::CmdError;
use tokio::time::{Instant, interval};
use crate::find::{AppiumFind, By};
use async_trait::async_trait;

pub trait AppiumWait {
    fn appium_wait(&self) -> Wait;
}

impl AppiumWait for Client {
    fn appium_wait(&self) -> Wait {
        Wait {
            client: self,
            timeout: Duration::from_secs(30),
            check_delay: Duration::from_millis(250),
        }
    }
}

#[derive(Debug)]
pub struct Wait<'c> {
    client: &'c Client,
    timeout: Duration,
    check_delay: Duration,
}

impl Wait<'_> {
    /// Set the timeout for maximum wait.
    ///
    /// Checks are performed in a loop, with an interval.
    /// To prevent infinite wait, the loop will exit after this timeout and the wait will result in an error indicating timeout.
    ///
    /// It is not guaranteed that the loop exits at the exact duration, as the check interval may hold it off.
    /// It works like this:
    /// 1. is the timeout exceeded?
    /// 2. try to locate
    /// 3. wait for interval
    /// 4. repeat
    pub fn at_most(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the period to delay checks.
    ///
    /// Checks are performed in a loop, with an interval defined by this method.
    /// For example, if you set it to 250 ms,
    /// then the loop will check if element is present, wait 250 ms and repeat.
    pub fn check_every(mut self, interval: Duration) -> Self {
        self.check_delay = interval;
        self
    }

    /// Waits for element using Appium locator.
    ///
    /// Tries to locate element in loop, with interval defined by "check delay".
    /// If the timeout is exceeded, then it returns an error.
    pub async fn for_element(self, search: By) -> Result<Element, CmdError> {
        WaitOnSingle(WaitSelector::new(self, search))
            .wait()
            .await
    }

    /// Waits for a list of elements using Appium locator.
    ///
    /// Tries to locate list of elements in loop, with interval defined by "check delay".
    /// If the timeout is exceeded, then it returns an error.
    pub async fn for_elements(self, search: By) -> Result<Vec<Element>, CmdError> {
        WaitOnMultiple(WaitSelector::new(self, search))
            .wait()
            .await
    }
}

#[async_trait]
trait AppiumWaitOnSelector<T> where Self: Sized {
    /// Checks if target can be located, then returns the result.
    /// If not found, waits for given delay and retries.
    /// Loops until a timeout is exceeded.
    async fn wait(self) -> Result<T, CmdError> {
        let wait = self.get_wait();
        let mut interval = interval(wait.check_delay);
        let timeout = wait.timeout;

        let start = Instant::now();
        loop {
            if start.elapsed() > timeout {
                return Err(CmdError::WaitTimeout);
            }

            {
                let find_element = self.locate();
                if let Some(result) = find_element.await? {
                    return Ok(result);
                }
            }

            interval.tick().await;
        }
    }

    /// Returns wait parameters
    fn get_wait(&self) -> &Wait;

    /// Logic for locating the target.
    async fn locate(&self) -> Result<Option<T>, CmdError>;
}


struct WaitSelector<'a> {
    wait: Wait<'a>,
    selector: By,
}

impl<'a> WaitSelector<'a> {
    pub fn new(wait: Wait, selector: By) -> WaitSelector {
        WaitSelector {
            wait,
            selector,
        }
    }
}

struct WaitOnSingle<'a>(WaitSelector<'a>);

struct WaitOnMultiple<'a>(WaitSelector<'a>);

#[async_trait]
impl<'a> AppiumWaitOnSelector<Element> for WaitOnSingle<'a> {
    fn get_wait(&self) -> &Wait {
        &self.0.wait
    }

    async fn locate(&self) -> Result<Option<Element>, CmdError> {
        find_element(&self.0.wait, self.0.selector.clone()).await
    }
}

#[async_trait]
impl<'a> AppiumWaitOnSelector<Vec<Element>> for WaitOnMultiple<'a> {
    fn get_wait(&self) -> &Wait {
        &self.0.wait
    }

    async fn locate(&self) -> Result<Option<Vec<Element>>, CmdError> {
        find_all_elements(&self.0.wait, self.0.selector.clone()).await
    }
}

async fn find_element(wait: &Wait<'_>, selector: By) -> Result<Option<Element>, CmdError> {
    match wait.client.find_by(selector).await {
        Ok(element) => Ok(Some(element)),
        Err(CmdError::NoSuchElement(_)) => Ok(None),
        Err(err) => Err(err),
    }
}

async fn find_all_elements(wait: &Wait<'_>, selector: By) -> Result<Option<Vec<Element>>, CmdError> {
    match wait.client.find_all_by(selector).await {
        Ok(result) => Ok(Some(result)),
        Err(CmdError::NoSuchElement(_)) => Ok(None),
        Err(err) => Err(err),
    }
}