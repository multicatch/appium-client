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

    let value = client.source().await?;
    println!("{value}");
    Ok(())
}