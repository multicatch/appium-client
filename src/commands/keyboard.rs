//! Keyboard management
use async_trait::async_trait;
use fantoccini::error::CmdError;
use http::Method;
use serde_derive::{Serialize, Deserialize};
use serde_json::json;
use serde_repr::Serialize_repr;
use crate::{AndroidClient, AppiumClientTrait, IOSClient};
use crate::commands::AppiumCommand;

/// Hide onscreen keyboard
#[async_trait]
pub trait HidesKeyboard: AppiumClientTrait {
    /// Tries to hide keyboard using default system mechanism.
    ///
    /// Note: On some devices, it defaults to "swipe" or "back" button.
    /// It unfortunately can cause side effects like going to the previous screen,
    /// or not hiding the keyboard at all in some apps.
    /// On iOS, the keyboard might not hide at all.
    ///
    /// In such cases, consider implementing your own "hide keyboard" with swipe or tap on screen.
    async fn hide_keyboard(&self) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            HIDE_KEYBOARD_ENDPOINT.to_string(),
            Some(json!({})),
        )).await?;
        Ok(())
    }
    
    async fn hide_keyboard_with_key(&self, key_name: &str) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            HIDE_KEYBOARD_ENDPOINT.to_string(),
            Some(json!({
                "keyName": key_name
            })),
        )).await?;
        Ok(())
    }

    async fn hide_keyboard_with_strategy(&self, strategy: HideKeyboardStrategy, key_name: &str) -> Result<(), CmdError> {
        self.issue_cmd(AppiumCommand::Custom(
            Method::POST,
            HIDE_KEYBOARD_ENDPOINT.to_string(),
            Some(json!({
                "keyName": key_name,
                "strategy": strategy
            })),
        )).await?;
        Ok(())
    }
}


const HIDE_KEYBOARD_ENDPOINT: &str = "appium/device/hide_keyboard";

#[async_trait]
impl HidesKeyboard for AndroidClient {}

#[async_trait]
impl HidesKeyboard for IOSClient {}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum HideKeyboardStrategy {
    Press,
    PressKey,
    SwipeDown,
    TapOut,
    TapOutside,
    Default,
}

#[derive(Debug, Clone, Serialize)]
pub struct KeyEvent {
    pub keycode: AndroidKey,
    pub metastate: u32,
    pub flags: u32,
}

impl KeyEvent {
    pub fn new(keycode: AndroidKey) -> KeyEvent {
        KeyEvent {
            keycode,
            metastate: 0,
            flags: 0,
        }
    }

    pub fn with_metamodifier(self, metamodifier: AndroidKeyMetaModifier) -> Self {
        let mut event = self;
        event.metastate |= metamodifier.bits();
        event
    }

    pub fn add_metamodifier(&mut self, metamodifier: AndroidKeyMetaModifier) {
        self.metastate |= metamodifier.bits();
    }

    pub fn remove_metamodifier(&mut self, metamodifier: AndroidKeyMetaModifier) {
        self.metastate &= !metamodifier.bits();
    }

    pub fn with_flag(self, flag: AndroidKeyFlag) -> Self {
        let mut event = self;
        event.flags |= flag.bits();
        event
    }

    pub fn add_flag(&mut self, flag: AndroidKeyMetaModifier) {
        self.flags |= flag.bits();
    }

    pub fn remove_flag(&mut self, flag: AndroidKeyMetaModifier) {
        self.flags &= !flag.bits();
    }
}

impl From<AndroidKey> for KeyEvent {
    fn from(value: AndroidKey) -> Self {
        KeyEvent {
            keycode: value,
            metastate: 0,
            flags: 0,
        }
    }
}

/// Send key presses to device
#[async_trait]
pub trait PressesKey: AppiumClientTrait {
    async fn press_key(&self, event: KeyEvent) -> Result<(), CmdError> {
        self.issue_cmd(
            AppiumCommand::Custom(
                Method::POST,
                "appium/device/press_keycode".to_string(),
                Some(serde_json::to_value(event)?),
            )
        ).await?;

        Ok(())
    }

    async fn long_press_key(&self, event: KeyEvent) -> Result<(), CmdError> {
        self.issue_cmd(
            AppiumCommand::Custom(
                Method::POST,
                "appium/device/long_press_keycode".to_string(),
                Some(serde_json::to_value(event)?),
            )
        ).await?;

        Ok(())
    }
}

#[async_trait]
impl PressesKey for AndroidClient {}

/// Check onscreen keyboard
#[async_trait]
pub trait HasOnScreenKeyboard: AppiumClientTrait {
    async fn keyboard_shown(&self) -> Result<bool, CmdError> {
        let value = self.issue_cmd(
            AppiumCommand::Custom(
                Method::GET,
                "appium/device/is_keyboard_shown".to_string(),
                None,
            )
        ).await?;

        Ok(serde_json::from_value(value)?)
    }
}

#[async_trait]
impl HasOnScreenKeyboard for AndroidClient {}

#[async_trait]
impl HasOnScreenKeyboard for IOSClient {}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize_repr)]
#[repr(u16)]
pub enum AndroidKey {
    /// Key code constant: Unknown key code.
    Unknown = 0,

    /// Key code constant: Soft Left key.
    /// Usually situated below the display on phones and used as a multi-function
    /// feature key for selecting a software defined function shown on the bottom left
    /// of the display.
    SoftLeft = 1,

    /// Key code constant: Soft Right key.
    /// Usually situated below the display on phones and used as a multi-function
    /// feature key for selecting a software defined function shown on the bottom right
    /// of the display.
    SoftRight = 2,

    /// Key code constant: Home key.
    /// This key is handled by the framework and is never delivered to applications.
    Home = 3,

    /// Key code constant: Back key.
    Back = 4,

    /// Key code constant: Call key.
    Call = 5,

    /// Key code constant: End Call key.
    EndCall = 6,

    /// Key code constant: '0' key.
    Digit0 = 7,

    /// Key code constant: '1' key.
    Digit1 = 8,

    /// Key code constant: '2' key.
    Digit2 = 9,

    /// Key code constant: '3' key.
    Digit3 = 10,

    /// Key code constant: '4' key.
    Digit4 = 11,

    /// Key code constant: '5' key.
    Digit5 = 12,

    /// Key code constant: '6' key.
    Digit6 = 13,

    /// Key code constant: '7' key.
    Digit7 = 14,

    /// Key code constant: '8' key.
    Digit8 = 15,

    /// Key code constant: '9' key.
    Digit9 = 16,

    /// Key code constant: '*' key.
    Asterisk = 17,

    /// Key code constant: '#' key.
    Pound = 18,

    /// Key code constant: Directional Pad Up key.
    /// May also be synthesized from trackball motions.
    DPadUp = 19,

    /// Key code constant: Directional Pad Down key.
    /// May also be synthesized from trackball motions.
    DPadDown = 20,

    /// Key code constant: Directional Pad Left key.
    /// May also be synthesized from trackball motions.
    DPadLeft = 21,

    /// Key code constant: Directional Pad Right key.
    /// May also be synthesized from trackball motions.
    DpadRight = 22,

    /// Key code constant: Directional Pad Center key.
    /// May also be synthesized from trackball motions.
    DPadCenter = 23,

    /// Key code constant: Volume Up key.
    /// Adjusts the speaker volume up.
    VolumeUp = 24,

    /// Key code constant: Volume Down key.
    /// Adjusts the speaker volume down.
    VolumeDown = 25,

    /// Key code constant: Power key.
    Power = 26,

    /// Key code constant: Camera key.
    /// Used to launch a camera application or take pictures.
    Camera = 27,

    /// Key code constant: Clear key.
    Clear = 28,

    /// Key code constant: 'A' key.
    A = 29,

    /// Key code constant: 'B' key.
    B = 30,

    /// Key code constant: 'C' key.
    C = 31,

    /// Key code constant: 'D' key.
    D = 32,

    /// Key code constant: 'E' key.
    E = 33,

    /// Key code constant: 'F' key.
    F = 34,

    /// Key code constant: 'G' key.
    G = 35,

    /// Key code constant: 'H' key.
    H = 36,

    /// Key code constant: 'I' key.
    I = 37,

    /// Key code constant: 'J' key.
    J = 38,

    /// Key code constant: 'K' key.
    K = 39,

    /// Key code constant: 'L' key.
    L = 40,

    /// Key code constant: 'M' key.
    M = 41,

    /// Key code constant: 'N' key.
    N = 42,

    /// Key code constant: 'O' key.
    O = 43,

    /// Key code constant: 'P' key.
    P = 44,

    /// Key code constant: 'Q' key.
    Q = 45,

    /// Key code constant: 'R' key.
    R = 46,

    /// Key code constant: 'S' key.
    S = 47,

    /// Key code constant: 'T' key.
    T = 48,

    /// Key code constant: 'U' key.
    U = 49,

    /// Key code constant: 'V' key.
    V = 50,

    /// Key code constant: 'W' key.
    W = 51,

    /// Key code constant: 'X' key.
    X = 52,

    /// Key code constant: 'Y' key.
    Y = 53,

    /// Key code constant: 'Z' key.
    Z = 54,

    /// Key code constant: ',' key.
    Comma = 55,

    /// Key code constant: '.' key.
    Period = 56,

    /// Key code constant: Left Alt modifier key.
    AltLeft = 57,

    /// Key code constant: Right Alt modifier key.
    AltRight = 58,

    /// Key code constant: Left Shift modifier key.
    ShiftLeft = 59,

    /// Key code constant: Right Shift modifier key.
    ShiftRight = 60,

    /// Key code constant: Tab key.
    Tab = 61,

    /// Key code constant: Space key.
    Space = 62,

    /// Key code constant: Symbol modifier key.
    /// Used to enter alternate symbols.
    SYM = 63,

    /// Key code constant: Explorer special function key.
    /// Used to launch a browser application.
    Explorer = 64,

    /// Key code constant: Envelope special function key.
    /// Used to launch a mail application.
    Envelope = 65,

    /// Key code constant: Enter key.
    Enter = 66,

    /// Key code constant: Backspace key.
    /// Deletes characters before the insertion point, unlike [AndroidKey::ForwardDel].
    DEL = 67,

    /// Key code constant: '`' (backtick) key.
    Grave = 68,

    /// Key code constant: '-'.
    Minus = 69,

    /// Key code constant: '=' key.
    Equals = 70,

    /// Key code constant: '[' key.
    LeftBracket = 71,

    /// Key code constant: ']' key.
    RightBracket = 72,

    /// Key code constant: '\' key.
    Backslash = 73,

    /// Key code constant: ';' key.
    Semicolon = 74,

    /// Key code constant: ''' (apostrophe) key.
    Apostrophe = 75,

    /// Key code constant: '/' key.
    Slash = 76,

    /// Key code constant: '@' key.
    At = 77,

    /// Key code constant: Number modifier key.
    /// Used to enter numeric symbols.
    /// This key is not Num Lock; it is more like [AndroidKey::AltLeft] and is
    /// interpreted as an ALT key
    NUM = 78,

    /// Key code constant: Headset Hook key.
    /// Used to hang up calls and stop media.
    HeadsetHook = 79,

    /// Key code constant: Camera Focus key.
    /// Used to focus the camera.
    Focus = 80,

    /// Key code constant: '+' key.
    Plus = 81,

    /// Key code constant: Menu key.
    Menu = 82,

    /// Key code constant: Notification key.
    Notification = 83,

    /// Key code constant: Search key.
    Search = 84,

    /// Key code constant: Play/Pause media key.
    MediaPlayPause = 85,

    /// Key code constant: Stop media key.
    MediaStop = 86,

    /// Key code constant: Play Next media key.
    MediaNext = 87,

    /// Key code constant: Play Previous media key.
    MediaPrevious = 88,

    /// Key code constant: Rewind media key.
    MediaRewind = 89,

    /// Key code constant: Fast Forward media key.
    MediaFastForward = 90,

    /// Key code constant: Mute key.
    /// Mutes the microphone, unlike [AndroidKey::VolumeMute].
    Mute = 91,

    /// Key code constant: Page Up key.
    PageUp = 92,

    /// Key code constant: Page Down key.
    PageDown = 93,

    /// Key code constant: Picture Symbols modifier key.
    /// Used to switch symbol sets (Emoji, Kao-moji).
    PictSymbols = 94,

    /// Key code constant: Switch Charset modifier key.
    /// Used to switch character sets (Kanji, Katakana).
    SwitchCharset = 95,

    /// Key code constant: A Button key.
    /// On a game controller, the A button should be either the button labeled A
    /// or the first button on the bottom row of controller buttons.
    ButtonA = 96,

    /// Key code constant: B Button key.
    /// On a game controller, the B button should be either the button labeled B
    /// or the second button on the bottom row of controller buttons.
    ButtonB = 97,

    /// Key code constant: C Button key.
    /// On a game controller, the C button should be either the button labeled C
    /// or the third button on the bottom row of controller buttons.
    ButtonC = 98,

    /// Key code constant: X Button key.
    /// On a game controller, the X button should be either the button labeled X
    /// or the first button on the upper row of controller buttons.
    ButtonX = 99,

    /// Key code constant: Y Button key.
    /// On a game controller, the Y button should be either the button labeled Y
    /// or the second button on the upper row of controller buttons.
    ButtonY = 100,

    /// Key code constant: Z Button key.
    /// On a game controller, the Z button should be either the button labeled Z
    /// or the third button on the upper row of controller buttons.
    ButtonZ = 101,

    /// Key code constant: L1 Button key.
    /// On a game controller, the L1 button should be either the button labeled L1 (or L)
    /// or the top left trigger button.
    ButtonL1 = 102,

    /// Key code constant: R1 Button key.
    /// On a game controller, the R1 button should be either the button labeled R1 (or R)
    /// or the top right trigger button.
    ButtonR1 = 103,

    /// Key code constant: L2 Button key.
    /// On a game controller, the L2 button should be either the button labeled L2
    /// or the bottom left trigger button.
    ButtonL2 = 104,

    /// Key code constant: R2 Button key.
    /// On a game controller, the R2 button should be either the button labeled R2
    /// or the bottom right trigger button.
    ButtonR2 = 105,

    /// Key code constant: Left Thumb Button key.
    /// On a game controller, the left thumb button indicates that the left (or only)
    /// joystick is pressed.
    ButtonThumbL = 106,

    /// Key code constant: Right Thumb Button key.
    /// On a game controller, the right thumb button indicates that the right
    /// joystick is pressed.
    ButtonThumbR = 107,

    /// Key code constant: Start Button key.
    /// On a game controller, the button labeled Start.
    ButtonStart = 108,

    /// Key code constant: Select Button key.
    /// On a game controller, the button labeled Select.
    ButtonSelect = 109,

    /// Key code constant: Mode Button key.
    /// On a game controller, the button labeled Mode.
    ButtonMode = 110,

    /// Key code constant: Escape key.
    Escape = 111,

    /// Key code constant: Forward Delete key.
    /// Deletes characters ahead of the insertion point, unlike [AndroidKey::DEL].
    ForwardDel = 112,

    /// Key code constant: Left Control modifier key.
    CtrlLeft = 113,

    /// Key code constant: Right Control modifier key.
    CtrlRight = 114,

    /// Key code constant: Caps Lock key.
    CapsLock = 115,

    /// Key code constant: Scroll Lock key.
    ScrollLock = 116,

    /// Key code constant: Left Meta modifier key.
    MetaLeft = 117,

    /// Key code constant: Right Meta modifier key.
    MetaRight = 118,

    /// Key code constant: Function modifier key.
    Function = 119,

    /// Key code constant: System Request / Print Screen key.
    SYSRQ = 120,

    /// Key code constant: Break / Pause key.
    Break = 121,

    /// Key code constant: Home Movement key.
    /// Used for scrolling or moving the cursor around to the start of a line
    /// or to the top of a list.
    MoveHome = 122,

    /// Key code constant: End Movement key.
    /// Used for scrolling or moving the cursor around to the end of a line
    /// or to the bottom of a list.
    MoveEnd = 123,

    /// Key code constant: Insert key.
    /// Toggles insert / overwrite edit mode.
    Insert = 124,

    /// Key code constant: Forward key.
    /// Navigates forward in the history stack.  Complement of [AndroidKey::Back].
    Forward = 125,

    /// Key code constant: Play media key.
    MediaPlay = 126,

    /// Key code constant: Pause media key.
    MediaPause = 127,

    /// Key code constant: Close media key.
    /// May be used to close a CD tray, for example.
    MediaClose = 128,

    /// Key code constant: Eject media key.
    /// May be used to eject a CD tray, for example.
    MediaEject = 129,

    /// Key code constant: Record media key.
    MediaRecord = 130,

    /// Key code constant: F1 key.
    F1 = 131,

    /// Key code constant: F2 key.
    F2 = 132,

    /// Key code constant: F3 key.
    F3 = 133,

    /// Key code constant: F4 key.
    F4 = 134,

    /// Key code constant: F5 key.
    F5 = 135,

    /// Key code constant: F6 key.
    F6 = 136,

    /// Key code constant: F7 key.
    F7 = 137,

    /// Key code constant: F8 key.
    F8 = 138,

    /// Key code constant: F9 key.
    F9 = 139,

    /// Key code constant: F10 key.
    F10 = 140,

    /// Key code constant: F11 key.
    F11 = 141,

    /// Key code constant: F12 key.
    F12 = 142,

    /// Key code constant: Num Lock key.
    /// This is the Num Lock key; it is different from [AndroidKey::NUM].
    /// This key alters the behavior of other keys on the numeric keypad.
    NumLock = 143,

    /// Key code constant: Numeric keypad '0' key.
    Numpad0 = 144,

    /// Key code constant: Numeric keypad '1' key.
    Numpad1 = 145,

    /// Key code constant: Numeric keypad '2' key.
    Numpad2 = 146,

    /// Key code constant: Numeric keypad '3' key.
    Numpad3 = 147,

    /// Key code constant: Numeric keypad '4' key.
    Numpad4 = 148,

    /// Key code constant: Numeric keypad '5' key.
    Numpad5 = 149,

    /// Key code constant: Numeric keypad '6' key.
    Numpad6 = 150,

    /// Key code constant: Numeric keypad '7' key.
    Numpad7 = 151,

    /// Key code constant: Numeric keypad '8' key.
    Numpad8 = 152,

    /// Key code constant: Numeric keypad '9' key.
    Numpad9 = 153,

    /// Key code constant: Numeric keypad '/' key (for division).
    NumpadDivide = 154,

    /// Key code constant: Numeric keypad '*' key (for multiplication).
    NumpadMultiply = 155,

    /// Key code constant: Numeric keypad '-' key (for subtraction).
    NumpadSubtract = 156,

    /// Key code constant: Numeric keypad '+' key (for addition).
    NumpadAdd = 157,

    /// Key code constant: Numeric keypad '.' key (for decimals or digit grouping).
    NumpadDot = 158,

    /// Key code constant: Numeric keypad ',' key (for decimals or digit grouping).
    NumpadComma = 159,

    /// Key code constant: Numeric keypad Enter key.
    NumpadEnter = 160,

    /// Key code constant: Numeric keypad '=' key.
    NumpadEquals = 161,

    /// Key code constant: Numeric keypad '(' key.
    NumpadLeftParen = 162,

    /// Key code constant: Numeric keypad ')' key.
    NumpadRightParen = 163,

    /// Key code constant: Volume Mute key.
    /// Mutes the speaker, unlike [AndroidKey::Mute].
    /// This key should normally be implemented as a toggle such that the first press
    /// mutes the speaker and the second press restores the original volume.
    VolumeMute = 164,

    /// Key code constant: Info key.
    /// Common on TV remotes to show additional information related to what is
    /// currently being viewed.
    Info = 165,

    /// Key code constant: Channel up key.
    /// On TV remotes, increments the television channel.
    ChannelUp = 166,

    /// Key code constant: Channel down key.
    /// On TV remotes, decrements the television channel.
    ChannelDown = 167,

    /// Key code constant: Zoom in key.
    KeycodeZoomIn = 168,

    /// Key code constant: Zoom out key.
    KeycodeZoomOut = 169,

    /// Key code constant: TV key.
    /// On TV remotes, switches to viewing live TV.
    TV = 170,

    /// Key code constant: Window key.
    /// On TV remotes, toggles picture-in-picture mode or other windowing functions.
    Window = 171,

    /// Key code constant: Guide key.
    /// On TV remotes, shows a programming guide.
    Guide = 172,

    /// Key code constant: DVR key.
    /// On some TV remotes, switches to a DVR mode for recorded shows.
    DVR = 173,

    /// Key code constant: Bookmark key.
    /// On some TV remotes, bookmarks content or web pages.
    Bookmark = 174,

    /// Key code constant: Toggle captions key.
    /// Switches the mode for closed-captioning text, for example during television shows.
    Captions = 175,

    /// Key code constant: Settings key.
    /// Starts the system settings activity.
    Settings = 176,

    /// Key code constant: TV power key.
    /// On TV remotes, toggles the power on a television screen.
    TVPower = 177,

    /// Key code constant: TV input key.
    /// On TV remotes, switches the input on a television screen.
    TVInput = 178,

    /// Key code constant: Set-top-box power key.
    /// On TV remotes, toggles the power on an external Set-top-box.
    STBPower = 179,

    /// Key code constant: Set-top-box input key.
    /// On TV remotes, switches the input mode on an external Set-top-box.
    STBInput = 180,

    /// Key code constant: A/V Receiver power key.
    /// On TV remotes, toggles the power on an external A/V Receiver.
    AVRPower = 181,

    /// Key code constant: A/V Receiver input key.
    /// On TV remotes, switches the input mode on an external A/V Receiver.
    AVRInput = 182,

    /// Key code constant: Red "programmable" key.
    /// On TV remotes, acts as a contextual/programmable key.
    ProgRed = 183,

    /// Key code constant: Green "programmable" key.
    /// On TV remotes, actsas a contextual/programmable key.
    ProgGreen = 184,

    /// Key code constant: Yellow "programmable" key.
    /// On TV remotes, acts as a contextual/programmable key.
    ProgYellow = 185,

    /// Key code constant: Blue "programmable" key.
    /// On TV remotes, acts as a contextual/programmable key.
    ProgBlue = 186,

    /// Key code constant: App switch key.
    /// Should bring up the application switcher dialog.
    AppSwitch = 187,

    /// Key code constant: Generic Game Pad Button #1.
    Button1 = 188,

    /// Key code constant: Generic Game Pad Button #2.
    Button2 = 189,

    /// Key code constant: Generic Game Pad Button #3.
    Button3 = 190,

    /// Key code constant: Generic Game Pad Button #4.
    Button4 = 191,

    /// Key code constant: Generic Game Pad Button #5.
    Button5 = 192,

    /// Key code constant: Generic Game Pad Button #6.
    Button6 = 193,

    /// Key code constant: Generic Game Pad Button #7.
    Button7 = 194,

    /// Key code constant: Generic Game Pad Button #8.
    Button8 = 195,

    /// Key code constant: Generic Game Pad Button #9.
    Button9 = 196,

    /// Key code constant: Generic Game Pad Button #10.
    Button10 = 197,

    /// Key code constant: Generic Game Pad Button #11.
    Button11 = 198,

    /// Key code constant: Generic Game Pad Button #12.
    Button12 = 199,

    /// Key code constant: Generic Game Pad Button #13.
    Button13 = 200,

    /// Key code constant: Generic Game Pad Button #14.
    Button14 = 201,

    /// Key code constant: Generic Game Pad Button #15.
    Button15 = 202,

    /// Key code constant: Generic Game Pad Button #16.
    Button16 = 203,

    /// Key code constant: Language Switch key.
    /// Toggles the current input language such as switching between English and Japanese on
    /// a QWERTY keyboard.  On some devices, the same function may be performed by
    /// pressing Shift+Spacebar.
    LanguageSwitch = 204,

    /// Key code constant: Manner Mode key.
    /// Toggles silent or vibrate mode on and off to make the device behave more politely
    /// in certain settings such as on a crowded train.  On some devices, the key may only
    /// operate when long-pressed.
    MannerMode = 205,

    /// Key code constant: 3D Mode key.
    /// Toggles the display between 2D and 3D mode.
    Mode3D = 206,

    /// Key code constant: Contacts special function key.
    /// Used to launch an address book application.
    Contacts = 207,

    /// Key code constant: Calendar special function key.
    /// Used to launch a calendar application.
    Calendar = 208,

    /// Key code constant: Music special function key.
    /// Used to launch a music player application.
    Music = 209,

    /// Key code constant: Calculator special function key.
    /// Used to launch a calculator application.
    Calculator = 210,

    /// Key code constant: Japanese full-width / half-width key.
    ZenkakuHankaku = 211,

    /// Key code constant: Japanese alphanumeric key.
    Eisu = 212,

    /// Key code constant: Japanese non-conversion key.
    Muhenkan = 213,

    /// Key code constant: Japanese conversion key.
    Henkan = 214,

    /// Key code constant: Japanese katakana / hiragana key.
    KatakanaHiragana = 215,

    /// Key code constant: Japanese Yen key.
    Yen = 216,

    /// Key code constant: Japanese Ro key.
    Ro = 217,

    /// Key code constant: Japanese kana key.
    Kana = 218,

    /// Key code constant: Assist key.
    /// Launches the global assist activity.  Not delivered to applications.
    Assist = 219,

    /// Key code constant: Brightness Down key.
    /// Adjusts the screen brightness down.
    BrightnessDown = 220,

    /// Key code constant: Brightness Up key.
    /// Adjusts the screen brightness up.
    BrightnessUp = 221,

    /// Key code constant: Audio Track key.
    /// Switches the audio tracks.
    MediaAudioTrack = 222,

    /// Key code constant: Sleep key.
    /// Puts the device to sleep.  Behaves somewhat like [AndroidKey::Power] but it
    /// has no effect if the device is already asleep.
    Sleep = 223,

    /// Key code constant: Wakeup key.
    /// Wakes up the device.  Behaves somewhat like [AndroidKey::Power] but it
    /// has no effect if the device is already awake.
    WakeUp = 224,

    /// Key code constant: Pairing key.
    /// Initiates peripheral pairing mode. Useful for pairing remote control
    /// devices or game controllers, especially if no other input mode is
    /// available.
    Pairing = 225,

    /// Key code constant: Media Top Menu key.
    /// Goes to the top of media menu.
    MediaTopMenu = 226,

    /// Key code constant: '11' key.
    Key11 = 227,

    /// Key code constant: '12' key.
    Key12 = 228,

    /// Key code constant: Last Channel key.
    /// Goes to the last viewed channel.
    LastChannel = 229,

    /// Key code constant: TV data service key.
    /// Displays data services like weather, sports.
    TVDataService = 230,

    /// Key code constant: Voice Assist key.
    /// Launches the global voice assist activity. Not delivered to applications.
    VoiceAssist = 231,

    /// Key code constant: Radio key.
    /// Toggles TV service / Radio service.
    TVRadioService = 232,

    /// Key code constant: Teletext key.
    /// Displays Teletext service.
    TVTeletext = 233,

    /// Key code constant: Number entry key.
    /// Initiates to enter multi-digit channel nubmber when each digit key is assigned
    /// for selecting separate channel. Corresponds to Number Entry Mode (0x1D) of CEC
    /// User Control Code.
    TVNumberEntry = 234,

    /// Key code constant: Analog Terrestrial key.
    /// Switches to analog terrestrial broadcast service.
    TVTerrestrialAnalog = 235,

    /// Key code constant: Digital Terrestrial key.
    /// Switches to digital terrestrial broadcast service.
    TVTerrestrialDigital = 236,

    /// Key code constant: Satellite key.
    /// Switches to digital satellite broadcast service.
    TVSatellite = 237,

    /// Key code constant: BS key.
    /// Switches to BS digital satellite broadcasting service available in Japan.
    TVSatelliteBS = 238,

    /// Key code constant: CS key.
    /// Switches to CS digital satellite broadcasting service available in Japan.
    TVSatelliteCS = 239,

    /// Key code constant: BS/CS key.
    /// Toggles between BS and CS digital satellite services.
    TVSatelliteService = 240,

    /// Key code constant: Toggle Network key.
    /// Toggles selecting broacast services.
    TVNetwork = 241,

    /// Key code constant: Antenna/Cable key.
    /// Toggles broadcast input source between antenna and cable.
    TVAntennaCable = 242,

    /// Key code constant: HDMI #1 key.
    /// Switches to HDMI input #1.
    TVInputHdmi1 = 243,

    /// Key code constant: HDMI #2 key.
    /// Switches to HDMI input #2.
    TVInputHdmi2 = 244,

    /// Key code constant: HDMI #3 key.
    /// Switches to HDMI input #3.
    TVInputHdmi3 = 245,

    /// Key code constant: HDMI #4 key.
    /// Switches to HDMI input #4.
    TVInputHdmi4 = 246,

    /// Key code constant: Composite #1 key.
    /// Switches to composite video input #1.
    TVInputComposite1 = 247,

    /// Key code constant: Composite #2 key.
    /// Switches to composite video input #2.
    TVInputComposite2 = 248,

    /// Key code constant: Component #1 key.
    /// Switches to component video input #1.
    TVInputComponent1 = 249,

    /// Key code constant: Component #2 key.
    /// Switches to component video input #2.
    TVInputComponent2 = 250,

    /// Key code constant: VGA #1 key.
    /// Switches to VGA (analog RGB) input #1.
    TVInputVga1 = 251,

    /// Key code constant: Audio description key.
    /// Toggles audio description off / on.
    TVAudioDescription = 252,

    /// Key code constant: Audio description mixing volume up key.
    /// Louden audio description volume as compared with normal audio volume.
    TVAudioDescriptionMixUp = 253,

    /// Key code constant: Audio description mixing volume down key.
    /// Lessen audio description volume as compared with normal audio volume.
    TVAudioDescriptionMixDown = 254,

    /// Key code constant: Zoom mode key.
    /// Changes Zoom mode (Normal, Full, Zoom, Wide-zoom, etc.)
    TVZoomMode = 255,

    /// Key code constant: Contents menu key.
    /// Goes to the title list. Corresponds to Contents Menu (0x0B) of CEC User Control
    /// Code
    TVContentsMenu = 256,

    /// Key code constant: Media context menu key.
    /// Goes to the context menu of media contents. Corresponds to Media Context-sensitive
    /// Menu (0x11) of CEC User Control Code.
    TVMediaContextMenu = 257,

    /// Key code constant: Timer programming key.
    /// Goes to the timer recording menu. Corresponds to Timer Programming (0x54) of
    /// CEC User Control Code.
    TVTimerProgramming = 258,

    /// Key code constant: Help key.
    Help = 259,

    /// Key code constant: Navigate to previous key.
    /// Goes backward by one item in an ordered collection of items.
    NavigatePrevious = 260,

    /// Key code constant: Navigate to next key.
    /// Advances to the next item in an ordered collection of items.
    NavigateNext = 261,

    /// Key code constant: Navigate in key.
    /// Activates the item that currently has focus or expands to the next level of a navigation
    /// hierarchy.
    NavigateIn = 262,

    /// Key code constant: Navigate out key.
    /// Backs out one level of a navigation hierarchy or collapses the item that currently has
    /// focus.
    NavigateOut = 263,

    /// Key code constant: Primary stem key for Wear.
    /// Main power/reset button on watch.
    StemPrimary = 264,

    /// Key code constant: Generic stem key 1 for Wear.
    Stem1 = 265,

    /// Key code constant: Generic stem key 2 for Wear.
    Stem2 = 266,

    /// Key code constant: Generic stem key 3 for Wear.
    Stem3 = 267,

    /// Key code constant: Directional Pad Up-Left.
    DpadUpLeft = 268,

    /// Key code constant: Directional Pad Down-Left.
    DpadDownLeft = 269,

    /// Key code constant: Directional Pad Up-Right.
    DpadUpRight = 270,

    /// Key code constant: Directional Pad Down-Right.
    DpadDownRight = 271,

    /// Key code constant: Skip forward media key.
    MediaSkipForward = 272,

    /// Key code constant: Skip backward media key.
    MediaSkipBackward = 273,

    /// Key code constant: Step forward media key.
    /// Steps media forward, one frame at a time.
    MediaStepForward = 274,

    /// Key code constant: Step backward media key.
    /// Steps media backward, one frame at a time.
    MediaStepBackward = 275,

    /// Key code constant: put device to sleep unless a wakelock is held.
    SoftSleep = 276,

    /// Key code constant: Cut key.
    Cut = 277,

    /// Key code constant: Copy key.
    Copy = 278,

    /// Key code constant: Paste key.
    Paste = 279,
}


bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct AndroidKeyMetaModifier: u32 {
            
        /// SHIFT key locked in CAPS mode.
        /// Reserved for use by MetaKeyKeyListener for a published constant in its API.
        const CAP_LOCKED = 0x100;

        /// ALT key locked.
        /// Reserved for use by MetaKeyKeyListener for a published constant in its API.
        const ALT_LOCKED = 0x200;

        /// SYM key locked.
        /// Reserved for use by MetaKeyKeyListener for a published constant in its API.
        const SYM_LOCKED = 0x400;

        /// Text is in selection mode.
        /// Reserved for use by MetaKeyKeyListener for a private unpublished constant
        /// in its API that is currently being retained for legacy reasons.
        const SELECTING = 0x800;

        /// This mask is used to check whether one of the ALT meta keys is pressed.
        ///
        /// See also [AndroidKey::AltLeft] and [AndroidKey::AltRight]
        const ALT_ON = 0x02;

        /// This mask is used to check whether the left ALT meta key is pressed.
        ///
        /// See also [AndroidKey::AltLeft]
        const ALT_LEFT_ON = 0x10;

        /// This mask is used to check whether the right the ALT meta key is pressed.
        ///
        /// See also [AndroidKey::AltRight]
        const ALT_RIGHT_ON = 0x20;

        /// This mask is used to check whether one of the SHIFT meta keys is pressed.
        ///
        /// See also [AndroidKey::ShiftLeft] and [AndroidKey::ShiftRight]
        const SHIFT_ON = 0x1;

        /// This mask is used to check whether the left SHIFT meta key is pressed.
        ///
        /// See also [AndroidKey::ShiftLeft]
        const SHIFT_LEFT_ON = 0x40;

        /// This mask is used to check whether the right SHIFT meta key is pressed.
        ///
        /// See also [AndroidKey::ShiftRight]
        const SHIFT_RIGHT_ON = 0x80;

        /// This mask is used to check whether the SYM meta key is pressed.
        const SYM_ON = 0x4;

        /// This mask is used to check whether the FUNCTION meta key is pressed.
        const FUNCTION_ON = 0x8;

        /// This mask is used to check whether one of the CTRL meta keys is pressed.
        ///
        /// See also [AndroidKey::CtrlLeft] and [AndroidKey::CtrlRight]
        const CTRL_ON = 0x1000;

        /// This mask is used to check whether the left CTRL meta key is pressed.
        ///
        /// See also [AndroidKey::CtrlLeft]
        const CTRL_LEFT_ON = 0x2000;

        /// This mask is used to check whether the right CTRL meta key is pressed.
        ///
        /// See also [AndroidKey::CtrlRight]
        const CTRL_RIGHT_ON = 0x4000;

        /// This mask is used to check whether one of the META meta keys is pressed.
        ///
        /// See also [AndroidKey::MetaLeft] and [AndroidKey::MetaRight].
        const META_ON = 0x10000;

        /// This mask is used to check whether the left META meta key is pressed.
        ///
        /// See also [AndroidKey::MetaLeft]
        const META_LEFT_ON = 0x20000;

        /// This mask is used to check whether the right META meta key is pressed.
        ///
        /// See also [AndroidKey::MetaRight]
        const META_RIGHT_ON = 0x40000;

        /// This mask is used to check whether the CAPS LOCK meta key is on.
        ///
        /// See also [AndroidKey::CapsLock]
        const CAPS_LOCK_ON = 0x100000;

        /// This mask is used to check whether the NUM LOCK meta key is on.
        ///
        /// See also [AndroidKey::NumLock]
        const NUM_LOCK_ON = 0x200000;

        /// This mask is used to check whether the SCROLL LOCK meta key is on.
        ///
        /// See also [AndroidKey::ScrollLock]
        const SCROLL_LOCK_ON = 0x400000;
    }
}


bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct AndroidKeyFlag: u32 {

     /// This mask is set if the key event was generated by a software keyboard.
    const SOFT_KEYBOARD = 0x2;

     /// This mask is set if we don't want the key event to cause us to leave
     /// touch mode.
    const KEEP_TOUCH_MODE = 0x4;

     /// This mask is set if an event was known to come from a trusted part
     /// of the system.  That is, the event is known to come from the user,
     /// and could not have been spoofed by a third party component.
    const FROM_SYSTEM = 0x8;

     /// This mask is used for compatibility, to identify enter keys that are
     /// coming from an IME whose enter key has been auto-labelled "next" or
     /// "done".  This allows TextView to dispatch these as normal enter keys
     /// for old applications, but still do the appropriate action when
     /// receiving them.
    const EDITOR_ACTION = 0x10;

     /// When associated with up key events, this indicates that the key press
     /// has been canceled.  Typically this is used with virtual touch screen
     /// keys, where the user can slide from the virtual key area on to the
     /// display: in that case, the application will receive a canceled up
     /// event and should not perform the action normally associated with the
     /// key.  Note that for this to work, the application can not perform an
     /// action for a key until it receives an up or the long press timeout has
     /// expired.
    const CANCELED = 0x20;

     /// This key event was generated by a virtual (on-screen) hard key area.
     /// Typically this is an area of the touchscreen, outside of the regular
     /// display, dedicated to "hardware" buttons.
    const VIRTUAL_HARD_KEY = 0x40;

     /// This flag is set for the first key repeat that occurs after the
     /// long press timeout.
    const LONG_PRESS = 0x80;

     /// Set when a key event has [AndroidKeyFlag::CANCELED] set because a long
     /// press action was executed while it was down.
    const CANCELED_LONG_PRESS = 0x100;

     /// Set for ACTION_UP when this event's key value is still being
     /// tracked from its initial down.  That is, somebody requested that tracking
     /// started on the key down and a long press has not caused
     /// the tracking to be canceled.
    const TRACKING = 0x200;

     /// Set when a key event has been synthesized to implement default behavior
     /// for an event that the application did not handle.
     /// Fallback key events are generated by unhandled trackball motions
     /// (to emulate a directional keypad) and by certain unhandled key presses
     /// that are declared in the key map (such as special function numeric keypad
     /// keys when numlock is off).
    const FALLBACK = 0x400;

     /// Signifies that the key is being predispatched.
    const PREDISPATCH = 0x20000000;

     /// Private control to determine when an app is tracking a key sequence.
    const START_TRACKING = 0x40000000;

     /// Private flag that indicates when the system has detected that this key event
     /// may be inconsistent with respect to the sequence of previously delivered key events,
     /// such as when a key up event is sent but the key was not down.
    const TAINTED = 0x80000000;
    }
}

