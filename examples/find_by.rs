use std::time::Duration;
use appium_client::ClientBuilder;
use appium_client::capabilities::*;
use appium_client::capabilities::android::AndroidCapabilities;
use appium_client::commands::rotation::SupportsRotation;
use appium_client::find::{AppiumFind, By};
use appium_client::wait::AppiumWait;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Capabilities describe the automation environment,
    // for example: what app are we testing
    let mut capabilities = AndroidCapabilities::new();
    capabilities.app("https://github.com/appium/android-apidemos/releases/download/v3.1.0/ApiDemos-debug.apk");

    // To add custom capability that is not supported by this library just use "insert".
    // Alternatively, there are helpful functions like "set_bool", "set_str".
    // You can read more about capabilities on Appium website - https://appium.io/docs/en/2.1/guides/caps/.
    capabilities.set_str("appium:automationName", "uiautomator2");
    capabilities.set_bool("appium:fullReset", true);

    // To start automation, you need to build a client.
    let client = ClientBuilder::native(capabilities)
        .connect("http://localhost:4723/")
        .await?;

    // The app should automatically start, let's print the DOM of current app screen.
    let value = client.source().await?;
    println!("{value}");

    // Screen orientation is another Appium perk
    let orientation = client.orientation().await?;
    println!("Screen orientation: {orientation}");

    // Now we try to locate a button using UiAutomator API.
    // Notice that the program will wait until the button appears on screen (but maximum of 30 seconds).
    let views_button = client
        .appium_wait()
        .for_element(By::uiautomator("new UiSelector().text(\"Views\");"))
        .await?;

    views_button.click().await?;

    // Search for a vec of elements, because we know that there will be more than one result.
    // Notice that this time we don't wait, just find everything that's on screen as is.
    let menu_elements = client
        .find_all_by(By::uiautomator("new UiSelector().className(\"android.widget.TextView\");"))
        .await?;

    menu_elements.get(1)
        .unwrap()
        .click()
        .await?;

    // To add a timeout for wait, use "at_most".
    // "check_every" limits how often Appium has to perform the search during wait.
    //
    // Sometimes it's better to use one or both of those methods, because:
    // 1) We know that something should appear sooner, and if it doesn't, we don't want to wait full 30 seconds.
    // 2) We don't want to slow down Appium server by checking again too often.
    let element = client
        .appium_wait()
        .at_most(Duration::from_secs(20))
        .check_every(Duration::from_millis(500))
        .for_element(By::class_name("android.widget.ListView"))
        .await?;

    // This is a simple search for one element, without waiting for it to appear. And then we click on it.
    // Notice that we are searching for an element inside "element" (which is a ListView).
    element
        .find_by(By::accessibility_id("3D Transition"))
        .await?
        .click()
        .await?;

    Ok(())
}