use appium_client::ClientBuilder;
use appium_client::capabilities::*;
use appium_client::commands::AppiumBy;
use appium_client::find::AppiumFindBy;
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

    let result = client
        .appium_wait()
        .for_element(AppiumBy::uiautomator("new UiSelector().className(\"android.widget.ImageView\");"))
        .await?;

    result.click().await?;

    let result = client.find_all_by(AppiumBy::uiautomator("new UiSelector().className(\"android.widget.LinearLayout\");")).await?;
    result.first().unwrap().click().await?;
    Ok(())
}