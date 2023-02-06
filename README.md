# appium-client

Rust client for Appium Server, for automated mobile app testing. It is based on [fantoccini](https://github.com/jonhoo/fantoccini).

## Sample usage

1. Create Appium client that will be used to issue commands and locate elements.

```rust
use appium_client::ClientBuilder;
use appium_client::capabilities::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut capabilities = AndroidCapabilities::new();
    capabilities.udid("emulator-5554");
    capabilities.app("/apps/sample.apk");
    capabilities.app_wait_activity("com.example.AppActivity");

    let client = ClientBuilder::native()
        .capabilities(capabilities.into())
        .connect("http://localhost:4723/wd/hub/")
        .await?;
    
    Ok(())
}
```

2. Locate an element by using your favorite location strategy (eg. by UiAutomator 2).
```rust
// you need this to use Appium locators with fantoccini's Client
use appium_client::find::{AppiumFind, By};

let element = client
    .find_by(By::accessibility_id("Click this"))
    .await?;

element.click().await?;
```

3. You can wait for element if it does not appear immediately on screen.
```rust
// you need these to use Appium-enhanced wait with fantoccini's Client
use appium_client::find::{AppiumFind, By};
use appium_client::wait::AppiumWait;

let element = client
    .appium_wait()
    .for_element(By::uiautomator("new UiSelector().className(\"android.widget.ImageView\");"))
    .await?;

element.click().await?;
```

4. You can define how long to wait for the element and how often to check if it's already appeared.
```rust
// you need these to use Appium-enhanced wait with fantoccini's Client
use appium_client::find::{AppiumFind, By};
use appium_client::wait::AppiumWait;

let element = client
    .appium_wait()
    .at_most(Duration::from_secs(20))
    .check_every(Duration::from_millis(500))
    .for_element(By::uiautomator("new UiSelector().className(\"android.widget.ImageView\");"))
    .await?;

element.click().await?;
```

4. To locate multiple elements, use `find_all_by`.
```rust
// you need these to use Appium-enhanced wait with fantoccini's Client
use appium_client::find::{AppiumFind, By};
use appium_client::wait::AppiumWait;

let result = client
    .appium_wait()
    .find_all_by(By::class_name("android.widget.LinearLayout"))
    .await?;

result.first().unwrap().click().await?;
```

[See example.](examples/simple.rs)

