use std::time::Duration;
use fantoccini::actions::{InputSource, MOUSE_BUTTON_LEFT, PointerAction, TouchActions};
use appium_client::ClientBuilder;
use appium_client::capabilities::*;
use appium_client::capabilities::android::AndroidCapabilities;
use appium_client::find::{AppiumFind, By};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut capabilities = AndroidCapabilities::new_uiautomator();
    capabilities.app("https://github.com/appium/android-apidemos/releases/download/v3.1.0/ApiDemos-debug.apk");

    let client = ClientBuilder::native(capabilities)
        .connect("http://localhost:4723/")
        .await?;

    // Go to a screen with a long list
    client.find_by(By::accessibility_id("Views"))
        .await?
        .click()
        .await?;

    // Let's calculate some things first
    let (width, height) = client.get_window_size().await?;

    // This is the horizontal center, it will be our x for swipe.
    let horizontal_center = (width / 2) as i64;

    // The swipe will start at 80% of screen height, and end at 20% of screen height.
    // So we will swipe UP through most of the screen.
    let almost_top = (height as f64 * 0.2) as i64;
    let almost_bottom = (height as f64 * 0.8) as i64;

    let swipe_down = TouchActions::new("finger".to_string())
        // position the finger first
        .then(PointerAction::MoveTo {
            duration: Some(Duration::from_millis(0)),
            x: horizontal_center,
            y: almost_bottom,
        })
        // THEN touch the screen
        .then(PointerAction::Down {
            button: MOUSE_BUTTON_LEFT // believe me, it is not a mouse, but a simple touch
        })
        // THEN move the finger through the screen
        .then(PointerAction::MoveTo {
            duration: Some(Duration::from_millis(500)),
            x: horizontal_center,
            y: almost_top,
        });

    client.perform_actions(swipe_down)
        .await?;

    Ok(())
}