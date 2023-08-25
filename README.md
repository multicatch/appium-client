# appium-client

Rust client for Appium Server, for automated mobile app testing. It is based on [fantoccini](https://github.com/jonhoo/fantoccini).

To learn more about Appium-specific features implemented here, [see the documentation](https://multicatch.github.io/appium-client/appium_client/). 
Below are examples that will help you quickly learn how to use key features.

Also check out the [examples](examples).

## Sample usage

### Creating the client

Create Appium client that will be used to issue commands and locate elements.

A client is an object that manages the connection to Appium server and issues commands to it.
Thus, we need capabilities that describe the automation environment and the server URL to create a client.

You can read more about capabilities in [Appium's docs](https://appium.io/docs/en/2.1/guides/caps/).

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

### Finding an element on screen

Locate an element by using your favorite location strategy (eg. by UiAutomator 2).

Other strategies, like name, XPath, iOS Class Chain etc. are also supported.

```rust
// you need this to use Appium locators with fantoccini's Client
use appium_client::find::{AppiumFind, By};

let element = client
    .find_by(By::accessibility_id("Click this"))
    .await?;

element.click().await?;
```

### Waiting for an element to appear

You can wait for element if it does not appear immediately on screen.

The wait, by default, is 30 seconds.
During the wait, the client performs a search every 250 ms until the element finally appears, or it hits the timeout.

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

### Limiting the wait

You can define how long to wait for the element and how often to check if it's already appeared.

This is useful in situations when you know something should appear sooner. 
And if it doesn't, then something else happened, and you don't want to bother waiting full 30 seconds until timeout.

The search interval may be also too adjusted so that the Appium server has more time to breathe.

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

### Locating many elements

To locate multiple elements, use `find_all_by` or `.appium_wait().for_elements(..)`.

The first method works just like `find_by` - it yields results immediately.
The second one just waits given time until at least one element appears. It works like the above example.

```rust
// you need these to use Appium-enhanced wait with fantoccini's Client
use appium_client::find::{AppiumFind, By};
use appium_client::wait::AppiumWait;

let result = client
    .appium_wait()
    .for_elements(By::class_name("android.widget.LinearLayout"))
    .await?;

result.first().unwrap().click().await?;
```

### Nested search

You can also perform search inside elements you found.

It is useful in cases when you want to find the parent element first, and then find a specific child inside.
No matter how bizarre that sounds, it is a useful feature when working with DOM.

```rust
// you need this to use Appium locators with fantoccini's Client
use appium_client::find::{AppiumFind, By};

let element = client
    .find_by(By::accessibility_id("Click this"))
    .await?;

// now let's find a child of element
let image_child = element
    .find_by(By::class_name("android.widget.ImageButton"))
    .await?;
```

### Scrolling

To scroll, you can use touch actions. For example, let's scroll up by simulating a swipe.

Remember that the swipe will "pull" the screen, so you need to swipe down to "pull" the screen down, revealing top content.

```rust
    let swipe_down = TouchActions::new("finger".to_string())
        // position the finger first
        .then(PointerAction::MoveTo {
            duration: Some(Duration::from_millis(0)),
            x,
            y
        })
        // THEN touch the screen
        .then(PointerAction::Down {
            button: MOUSE_BUTTON_LEFT // believe me, it is not a mouse, but a simple touch
        })
        // THEN move the finger through the screen
        .then(PointerAction::MoveTo {
            duration: Some(Duration::from_millis(500)),
            x,
            y
        });

    client.perform_actions(swipe_down)
        .await?;
```

[See basic example here.](examples/simple.rs)

