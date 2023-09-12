# appium-client

Rust client for Appium Server, for automated mobile app testing. It is based on [fantoccini](https://github.com/jonhoo/fantoccini).

To learn more about Appium-specific features implemented here, [see the documentation](https://multicatch.github.io/appium-client/appium_client/). 
Below are examples that will help you quickly learn how to use key features.

Also check out the [examples](examples).

## Features

- [x] Predefined iOS and Android capabilities (at least some of them).
- [x] Locking and unlocking screen.
- [x] Getting the time of the device/emulator.
- [x] Getting the rotation and orientation of device.
- [x] Screen recording support.
- [x] Battery state support.
- [x] Android network state.
- [x] Changing device settings.
- [x] Pushing and pulling files.
- [x] Getting app strings.
- [x] Accessing device clipboard.
- [x] Touch ID and fingerprint authentication simulation.
- [x] Keyboard simulation.

## How to use?

You need to start an [Appium Server](http://appium.io) first. 
You also need to connect a device (or an emulator) to the machine that runs the Appium Server.

If you have set up Appium properly, then you can use this library to connect to the server and control the device.

To connect to the Appium Server, you need to create an instance of a `Client`.
To do so, create appropriate capabilities (e.g. `AndroidCapabilities::new()`) and then supply then to a `ClientBuilder`.

You need a Tokio async runtime for it to work properly.

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut capabilities = AndroidCapabilities::new();
    capabilities.udid("emulator-5554");
    capabilities.app("/apps/sample.apk");
    capabilities.app_wait_activity("com.example.AppActivity");

    let client = ClientBuilder::native(capabilities)
        .connect("http://localhost:4723/wd/hub/")
        .await?;

    // now you can start testing

    Ok(())
}
```

Appium interprets the screen of the device using DOM (Document Object Model).
Elements on the screen are translated into some kind of XML that is compliant with the W3C standard of Selenium.

Appium is some kind of extension to this, with drivers that allow to control mobile devices, emulators or desktop OS.

To see how Appium interprets the device screen (and to interact with the device using Appium),
you can use a tool called [Appium Inspector](https://github.com/appium/appium-inspector).
It's a very useful GUI tool that can help during automation development.

## What if there is a missing feature?

You can make a PR and add the missing feature. 

If you don't have time for this, you can also issue commands directly, without relying on traits from this library.

For example, let's assume Appium added a new feature called "simulate barrel roll on the device".
Appium Server has a new API for this - `POST /session/:sessionId/appium/device/barrel_roll`.
We can specify how many times the device will do a barrel roll in the request as `{"times": number}`.

You don't have to wait until I add this feature to the library. You can issue a custom command:

```rust
client.issue_cmd(AppiumCommand::Custom(
    Method::POST,
    "appium/device/barrel_roll".to_string(),
    Some(json!({
        "times": 2
    }))
)).await?;
```

As you can see, I didn't add `/session/:sessionId` from the original endpoint. 
There is no need to - the Appium client adds this automatically.

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

    let client = ClientBuilder::native(capabilities)
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

## Examples

- [See basic example here.](examples/simple.rs)
- [Scrolling example here.](examples/scroll.rs)

