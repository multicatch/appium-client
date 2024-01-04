# Setup And Writing Your First Automation

In this little tutorial I will show you how to set up and write a simple automation.
We will be automating an Android app, because you can replicate the tutorial on Windows, Mac and Linux (for free).

If you find that [The Setup](#the-setup) is not comprehensible for you, then you can also
use [Quickstart from Appium docs](https://appium.io/docs/en/2.1/quickstart/)
to set up Android Studio and Appium.

After you set up Android Studio, Appium and UiAutomator2, you can write [your first automation](#the-first-automation).

## What Will You Need?

* Your favorite IDE and Rust,
* Android Studio (with an emulator),
* basic knowledge of system terminal (like bash, zsh or PowerShell),
* `npm` (or some package manager that will allow you to install npm),
* Java JDK 9+ (for UiAutomator2 driver).

## In this tutorial

* [The Setup](#the-setup)
    * [Android Studio setup](#android-studio-setup)
        * [Installing Android SDK](#installing-android-sdk-platform-tools-with-adb)
        * [Set Up The Emulator](#set-up-the-emulator)
    * [Appium Setup](#appium-setup)
        * [Installing npm](#installing-npm)
        * [Installing Java JDK 9+](#installing-java-jdk-9)
        * [Installing Appium via npm](#installing-appium-via-npm)
        * [Installing UiAutomator2 Driver](#installing-uiautomator2-driver)
        * [Installing Appium Inspector](#installing-appium-inspector)
* [The First Automation](#the-first-automation)
  * [Preparing The Environment](#preparing-the-environment)
    * [Download Demo APK](#download-demo-apk)
    * [Start The Emulator](#start-the-emulator)
    * [Start Appium](#start-appium)
    * [Test Configuration via Appium Inspector](#test-configuration-via-appium-inspector)
  * [The Development](#the-development)
    * [Set Up a Rust Project](#set-up-a-rust-project)
    * [Run App via Rust Code](#run-app-via-rust-code)

# The Setup

## Android Studio Setup

Android Studio is needed for two things:

* **ADB** (from Android SDK) - Appium needs this to communicate with the Android device,
* **the emulator** (optional) - the app has to be run somewhere.

You can skip the emulator part if you wish to use your own device.
But nonetheless you will need ADB, and we will use Android Studio to install it for us.

Download and install Android Studio from its official website: https://developer.android.com/studio.

### Installing Android SDK (Platform Tools with ADB)

When launch Android Studio, you should be greeted with a message saying that SDK is missing.

![](docimg/missing_sdk.png)

Just follow the steps in this wizard (click "Next") and remember to accept the license.
Android Studio will automatically download the SDK for you.

Now validate the configuration by clicking `More Actions` and `SDK Manager`.

![](docimg/android_studio_sdk_setup.png)

In this window validate the following:

* Android SDK Location (should not be empty),
* SDK Platforms (at least one should be selected, e.g. `Android 14.0 ("UpsideDownCake")` (API 34)),
* SDK Tools: Android SDK Build-Tools, Android Emulator, Android SDK Platform-Tools (those should be selected).

If you're missing any of the components, select the missing components.

If the window is greyed out, and you cannot select anything (or you have no SDK Location), then you have to fix the SDK.
Part of the window should look like on the screenshot below.
Click "Edit" and an SDK wizard should appear and will install missing components of your SDK.

![](docimg/incorrect_sdk.png)

Now set up the `ANDROID_HOME` environment variable, so it points to the SDK directory.
The SDK directory is the "Android SDK Location" from the screen shown above.

For example, if the SDK is installed in `/Users/multicatch/Library/Android/sdk`, then your variable should
be `ANDROID_HOME=/Users/multicatch/Library/Android/sdk`.

On Windows you can edit environment variables by searching "environment properties" in the Start Menu.
Click the first result and then click "Environment Variables..." to edit variables.

On Mac add the following line to `.zshrc` and on Linux you have to add the following line to `.bashrc`:

```bash
export ANDROID_HOME=/your/path/here
```

Both files should be in your home directory (they are hidden).

### Set Up The Emulator

![](docimg/android_studio_welcome_emulator.png)

From the Android Studio welcome screen, select `More Actions` and `Virtual Device Manager`.
In the next window, click the plus sign (or "Create virtual device...").

Select any virtual device model you want. I selected `Pixel 7` and clicked "Next".

After device model selection, you will have the option to choose Android version you want to install on the emulator.
This version is also tied to the SDK version, as you can see in the "API Level" column.

![](docimg/vm_system_image.png)

I have chosen "UpsideDownCake" (Android 14.0, API 34). If your "Next" button is greyed out, you need to download the SDK
first.
Click the download icon next to selected system image and "Next" should become clickable after a while.

In the next step, you can adjust some emulator settings and name the virtual device (AVD).
I left the defaults and clicked "Finish".

Start the AVD.

![](docimg/starting_avd.png)

You should be all set now.

## Appium Setup

We will need Appium and UiAutomator2 driver.

To install Appium, you will need `npm`.

### Installing npm

#### Windows

Download and install https://nodejs.org/en/download. Run `npm -v` to verify installation.

#### Mac

Download and install https://nodejs.org/en/download. If you have `homebrew`, you can just run:

```bash
brew install node
```

Run `npm -v` to verify installation (you will probably need to restart the Terminal first to reload the shell).

#### Linux

Use your favorite package manager to install `npm`.
For example:

```bash
sudo apt update
sudo apt install nodejs npm
```

### Installing Java JDK 9+

Use your favorite JDK. You can use one from https://jdk.java.net/, or https://adoptium.net/temurin/releases/.

Installation instructions might be different depending on which JDK release you choose.

Note: **The minimum JDK version must be 9** (but you can choose a newer one).

Java is needed to run UiAutomator2, this driver is written in Java and Appium will need to run it in order to control
the Android device.

Remember to set up `JAVA_HOME`. If you used an installer, then it might be configured automatically.
If not, then set it [like we set up `ANDROID_HOME`](#installing-android-sdk-platform-tools-with-adb), but it should
point to a location where Java is installed.

Verify that your `JAVA_HOME` points to the JDK home directory.
The following command should show you folders like `bin` and `include`.

```bash
ls "$JAVA_HOME"
```

If you got an error (or there is no `bin` and `include`), then the environment variable is configured incorrectly.

### Installing Appium via npm

To install Appium globally run the following command:

```bash
npm i -g appium
```

Verify your installation by running `appium`. Use ctrl+c to exit appium when you're done.

### Installing UiAutomator2 Driver

The UiAutomator2 driver is a tool that helps Appium "talk" with the Android device.
Depending on whether you test on iOS, or Android, or other platform, you need to install different drivers.

Apart from UiAutomator2, there is also another driver for Android called "Espresso".
But for our needs, we won't need it.

Run the following command to install UiAutomator2:

```bash
appium driver install uiautomator2
```

### Installing Appium Inspector

Appium Inspector is a tool that will help us understand what Appium knows about the currently displayed screen.

It's a really easy-to-use tool and I find it very useful during test automation development.

Download and install the tool from this page: https://github.com/appium/appium-inspector/releases.

You should be now ready to go!

# The First Automation

## Preparing The Environment

We need to do a few things to see whether we set up everything correctly (and before we start the development)

### Download Demo APK

Just download the newest demo apk from https://github.com/appium/android-apidemos/releases/.
Save it on your computer in a known location. We will need the path later.

For example, I have downloaded the APK to the following location:

```text
/Users/multicatch/apps/ApiDemos-debug.apk
```

### Start The Emulator

If the emulator is not running, then start Android Studio, click `More Actions` and `Virtual Device Manager`.
Select the emulator to run.

### Start Appium

Run `appium` in terminal.

### Test Configuration via Appium Inspector

Start Appium Inspector. It should look more or less like this:

![](docimg/appium_inspector_start.png)

Verify the following configuration:

* Remote Host: `127.0.0.1`
* Port: `4723`
* Remote Path: `/` (or empty)

Now we can set **Desired Capabilities** to start the APK via Appium.

Desired capabilities (or capabilities in short) are a set of configuration properties for Appium
that tell the Appium and the driver how to behave.

Capabilities can tell appium:

* which device to use,
* which app to use,
* how to start the app (if it doesn't start normally),
* how long to wait for the app to start,
* if you want to reset the app before each test,
* if you want to reinstall the app before each test,
* and many more.

Depending on which driver you use, you might need different capabilities.
You can read more about capabilities by clicking the link at the bottom of Inspector's window.

In our case, the configuration will be very simple.

Configure the following capabilities:

| Capability        | Type   | Value                                                                     |
|-------------------|--------|---------------------------------------------------------------------------|
| `platformName`    | `text` | `Android`                                                                 |
| `platformVersion` | `text` | `14` (use different value if you have installed other Android version)    |
| `automationName`  | `text` | `UiAutomator2`                                                            |
| `app`             | `text` | path to your apk, in my case: `/Users/multicatch/apps/ApiDemos-debug.apk` |

Make sure that "Automatically add necessary Appium vendor prefixes on start".

Click "Start Session". The app should start.

Appium Inspector will now show the captured app screen and the "App Source".

![](docimg/appium_inspector_start_session.png)

Appium Inspector does not update the screen live. 
It only shows a capture. 
To refresh the view, you need to click "refresh" icon at the top bar.

When you click any element on the screenshot, then Inspector will highlight
a part of the App Source, which represents the element.

In the right panel you will see properties of selected element.
You can simulate "tap" by clicking the crosshair button.

Try to play around in Appium Inspector on your own.

## The Development

### Set Up a Rust Project

Use `cargo init` or your IDE to create a new Rust project.
I use IntelliJ IDEA with Rust plugin.

Add the following dependencies to `Cargo.toml`:

```toml
appium-client = "0.2.1"
tokio = "1.25.0"
```

And create empty app in `main.rs` with tokio runtime.

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
```

Verify if everything was set up correctly with `cargo build`.

### Run App via Rust Code

Now to start an Appium session from Rust, we need to:
* create a set of capabilities (like in Appium Inspector),
* connect to Appium server and start the session (which here is the same as creating the client).

We are automating an Android test with UiAutomator2 driver. 
So we can use `AndroidCapabilities::new_uiautomator()` to create basic set of capabilities.

When you create `AndroidCapabilities` by using this function, then you will automatically set:
* `platformName` = `Android`                                                          |
* `automationName` = `UiAutomator2`

Now we need to set only the two last capabilities - `platformVersion` and `app`.

```rust
use appium_client::capabilities::android::AndroidCapabilities;
use appium_client::capabilities::{AppCapable, AppiumCapability};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut capabilities = AndroidCapabilities::new_uiautomator();
    capabilities.platform_version("14");
    capabilities.app("/Users/multicatch/apps/ApiDemos-debug.apk");

    Ok(())
}
```

To start a session with those capabilities, create a client using `ClientBuilder`.

We will use `ClientBuilder::native(capabilities)`, because we want to use a native HTTP client to connect to Appium server.
We also need to specify the address of Appium server, which is `http://localhost:4723/`.

```rust
use appium_client::capabilities::android::AndroidCapabilities;
use appium_client::capabilities::{AppCapable, AppiumCapability};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut capabilities = AndroidCapabilities::new_uiautomator();
    capabilities.platform_version("14");
    capabilities.app("/Users/multicatch/apps/ApiDemos-debug.apk");

    let _client = ClientBuilder::native(capabilities)
        .connect("http://localhost:4723/")
        .await?;

    Ok(())
}
```

Voilà! Try running the program. 

It should connect to the Appium server and start the app.

If it did not work, then check if:
* You did not copy the path to app (and the path is correct),
* Appium server is running,
* Appium and UiAutomator2 was set up correctly,
* Android emulator is running,
* You enabled debugging on your Android device.

