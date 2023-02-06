use std::time::Duration;
use appium_client::ClientBuilder;
use appium_client::capabilities::*;
use appium_client::find::{AppiumFind, By};
use appium_client::wait::AppiumWait;

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

    let value = client.source().await?;
    println!("{value}");

    let more_button = client
        .appium_wait()
        .for_element(By::uiautomator("new UiSelector().className(\"android.widget.ImageView\");"))
        .await?;

    more_button.click().await?;

    let menu_elements = client
        .find_all_by(By::uiautomator("new UiSelector().className(\"android.widget.LinearLayout\");"))
        .await?;
    menu_elements.first().unwrap().click().await?;

    let element = client
        .appium_wait()
        .at_most(Duration::from_secs(20))
        .check_every(Duration::from_millis(500))
        .for_element(By::class_name("android.widget.LinearLayout"))
        .await?;

    element.click().await?;

    Ok(())
}