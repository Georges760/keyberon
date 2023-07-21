//! Key code definitions.

#[allow(missing_docs)]
/// Define a key code according to the HID specification.  Their names
/// correspond to the american QWERTY layout.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum KeyCode {
    /// The "no" key, a placeholder to express nothing.
    No = 0x00,
    /// Error if too much keys are pressed at the same time.
    ErrorRollOver,
    /// The POST fail error.
    PostFail,
    /// An undefined error occured.
    ErrorUndefined,
    /// `a` and `A`.
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M, // 0x10
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    /// `1` and `!`.
    Kb1,
    /// `2` and `@`.
    Kb2,
    /// `3` and `#`.
    Kb3, // 0x20
    /// `4` and `$`.
    Kb4,
    /// `5` and `%`.
    Kb5,
    /// `6` and `^`.
    Kb6,
    /// `7` and `&`.
    Kb7,
    /// `8` and `*`.
    Kb8,
    /// `9` and `(`.
    Kb9,
    /// `0` and `)`.
    Kb0,
    Enter,
    Escape,
    BSpace,
    Tab,
    Space,
    /// `-` and `_`.
    Minus,
    /// `=` and `+`.
    Equal,
    /// `[` and `{`.
    LBracket,
    /// `]` and `}`.
    RBracket, // 0x30
    /// `\` and `|`.
    Bslash,
    /// Non-US `#` and `~` (Typically near the Enter key).
    NonUsHash,
    /// `;` and `:`.
    SColon,
    /// `'` and `"`.
    Quote,
    // How to have ` as code?
    /// \` and `~`.
    Grave,
    /// `,` and `<`.
    Comma,
    /// `.` and `>`.
    Dot,
    /// `/` and `?`.
    Slash,
    CapsLock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7, // 0x40
    F8,
    F9,
    F10,
    F11,
    F12,
    PScreen,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PgUp,
    Delete,
    End,
    PgDown,
    Right,
    Left, // 0x50
    Down,
    Up,
    NumLock,
    /// Keypad `/`
    KpSlash,
    /// Keypad `*`
    KpAsterisk,
    /// Keypad `-`.
    KpMinus,
    /// Keypad `+`.
    KpPlus,
    /// Keypad enter.
    KpEnter,
    /// Keypad 1.
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8, // 0x60
    Kp9,
    Kp0,
    KpDot,
    /// Non-US `\` and `|` (Typically near the Left-Shift key)
    NonUsBslash,
    Application, // 0x65
    /// not a key, used for errors
    Power,
    /// Keypad `=`.
    KpEqual,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21, // 0x70
    F22,
    F23,
    F24,
    Execute,
    Help,
    Menu,
    Select,
    Stop,
    Again,
    Undo,
    Cut,
    Copy,
    Paste,
    Find,
    Mute,
    VolUp, // 0x80
    VolDown,
    /// Deprecated.
    LockingCapsLock,
    /// Deprecated.
    LockingNumLock,
    /// Deprecated.
    LockingScrollLock,
    /// Keypad `,`, also used for the brazilian keypad period (.) key.
    KpComma,
    /// Used on AS/400 keyboard
    KpEqualSign,
    Intl1,
    Intl2,
    Intl3,
    Intl4,
    Intl5,
    Intl6,
    Intl7,
    Intl8,
    Intl9,
    Lang1, // 0x90
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    Lang6,
    Lang7,
    Lang8,
    Lang9,
    AltErase,
    SysReq,
    Cancel,
    Clear,
    Prior,
    Return,
    Separator,
    Out, // 0xA0
    Oper,
    ClearAgain,
    CrSel,
    ExSel,

    // According to QMK, 0xA5-0xDF are not usable on modern keyboards

    // Modifiers
    /// Left Control.
    LCtrl = 0xE0,
    /// Left Shift.
    LShift,
    /// Left Alt.
    LAlt,
    /// Left GUI (the Windows key).
    LGui,
    /// Right Control.
    RCtrl,
    /// Right Shift.
    RShift,
    /// Right Alt (or Alt Gr).
    RAlt,
    /// Right GUI (the Windows key).
    RGui, // 0xE7

    // Unofficial
    MediaPlayPause = 0xE8,
    MediaStopCD,
    MediaPreviousSong,
    MediaNextSong,
    MediaEjectCD,
    MediaVolUp,
    MediaVolDown,
    MediaMute,
    MediaWWW, // 0xF0
    MediaBack,
    MediaForward,
    MediaStop,
    MediaFind,
    MediaScrollUp,
    MediaScrollDown,
    MediaEdit,
    MediaSleep,
    MediaCoffee,
    MediaRefresh,
    MediaCalc, // 0xFB
}

impl KeyCode {
    /// Returns `true` if the key code corresponds to a modifier (sent
    /// separately on the USB HID report).
    pub fn is_modifier(self) -> bool {
        KeyCode::LCtrl <= self && self <= KeyCode::RGui
    }

    /// Returns the byte with the bit corresponding to the USB HID
    /// modifier bitfield set.
    pub fn as_modifier_bit(self) -> u8 {
        if self.is_modifier() {
            1 << (self as u8 - KeyCode::LCtrl as u8)
        } else {
            0
        }
    }

    /// View this Key Code as a printable text
    pub fn as_str(&self) -> &str {
        match self {
            KeyCode::A => "A",
            KeyCode::B => "B",
            KeyCode::C => "C",
            KeyCode::D => "D",
            KeyCode::E => "E",
            KeyCode::F => "F",
            KeyCode::G => "G",
            KeyCode::H => "H",
            KeyCode::I => "I",
            KeyCode::J => "J",
            KeyCode::K => "K",
            KeyCode::L => "L",
            KeyCode::M => "M",
            KeyCode::N => "N",
            KeyCode::O => "O",
            KeyCode::P => "P",
            KeyCode::Q => "Q",
            KeyCode::R => "R",
            KeyCode::S => "S",
            KeyCode::T => "T",
            KeyCode::U => "U",
            KeyCode::V => "V",
            KeyCode::W => "W",
            KeyCode::X => "X",
            KeyCode::Y => "Y",
            KeyCode::Z => "Z",
            KeyCode::Kb1 => "1", // `1` and `!`
            KeyCode::Kb2 => "2", // `2` and `@`
            KeyCode::Kb3 => "3", // `3` and `#`
            KeyCode::Kb4 => "4", // `4` and `$`
            KeyCode::Kb5 => "5", // `5` and `%`
            KeyCode::Kb6 => "6", // `6` and `^`
            KeyCode::Kb7 => "7", // `7` and `&`
            KeyCode::Kb8 => "8", // `8` and `*`
            KeyCode::Kb9 => "9", // `9` and `(`
            KeyCode::Kb0 => "0", // `0` and `)`
            KeyCode::Enter => "↵",
            KeyCode::Escape => "␛",
            KeyCode::BSpace => "⌫",
            KeyCode::Tab => "⭾",
            KeyCode::Space => "␠",
            KeyCode::Minus => "-",     // `-` and `_`
            KeyCode::Equal => "=",     // `=` and `+`
            KeyCode::LBracket => "[",  // `[` and `{`
            KeyCode::RBracket => "]",  // `]` and `}`
            KeyCode::Bslash => "\\",   // `\` and `|`
            KeyCode::NonUsHash => "#", // Non-US `#` and `~` (Typically near the Enter key)
            KeyCode::SColon => ";",    // `;` and `:`
            KeyCode::Quote => "'",     // `'` and `"`
            KeyCode::Grave => "`",     // \` and `~`
            KeyCode::Comma => ",",     // `,` and `<`
            KeyCode::Dot => ".",       // `.` and `>`
            KeyCode::Slash => "/",     // `/` and `?`
            KeyCode::CapsLock => "CapsLock",
            KeyCode::F1 => "F1",
            KeyCode::F2 => "F2",
            KeyCode::F3 => "F3",
            KeyCode::F4 => "F4",
            KeyCode::F5 => "F5",
            KeyCode::F6 => "F6",
            KeyCode::F7 => "F7",
            KeyCode::F8 => "F8",
            KeyCode::F9 => "F9",
            KeyCode::F10 => "F10",
            KeyCode::F11 => "F11",
            KeyCode::F12 => "F12",
            KeyCode::PScreen => "PScreen",
            KeyCode::ScrollLock => "ScrollLock",
            KeyCode::Pause => "Pause",
            KeyCode::Insert => "Insert",
            KeyCode::Home => "↖",
            KeyCode::PgUp => "⇞",
            KeyCode::Delete => "␡",
            KeyCode::End => "End",
            KeyCode::PgDown => "⇟",
            KeyCode::Right => "▶",
            KeyCode::Left => "◀",
            KeyCode::Down => "▼",
            KeyCode::Up => "▲",
            KeyCode::NumLock => "NumLock",
            KeyCode::KpSlash => "/",
            KeyCode::KpAsterisk => "*",
            KeyCode::KpMinus => "-",
            KeyCode::KpPlus => "+",
            KeyCode::KpEnter => "↵",
            KeyCode::Kp1 => "1",
            KeyCode::Kp2 => "2",
            KeyCode::Kp3 => "3",
            KeyCode::Kp4 => "4",
            KeyCode::Kp5 => "5",
            KeyCode::Kp6 => "6",
            KeyCode::Kp7 => "7",
            KeyCode::Kp8 => "8",
            KeyCode::Kp9 => "9",
            KeyCode::Kp0 => "0",
            KeyCode::KpDot => "",
            KeyCode::NonUsBslash => "\\",
            KeyCode::Application => "≣",
            KeyCode::KpEqual => "=",
            KeyCode::F13 => "F13",
            KeyCode::F14 => "F14",
            KeyCode::F15 => "F15",
            KeyCode::F16 => "F16",
            KeyCode::F17 => "F17",
            KeyCode::F18 => "F18",
            KeyCode::F19 => "F19",
            KeyCode::F20 => "F20",
            KeyCode::F21 => "F21",
            KeyCode::F22 => "F22",
            KeyCode::F23 => "F23",
            KeyCode::F24 => "F24",
            KeyCode::Execute => "Execute",
            KeyCode::Help => "Help",
            KeyCode::Menu => "Menu",
            KeyCode::Select => "Select",
            KeyCode::Stop => "Stop",
            KeyCode::Again => "Again",
            KeyCode::Undo => "Undo",
            KeyCode::Cut => "Cut",
            KeyCode::Copy => "Copy",
            KeyCode::Paste => "Paste",
            KeyCode::Find => "Find",
            KeyCode::Mute => "🔇",
            KeyCode::VolUp => "🔉",
            KeyCode::VolDown => "🔊",
            KeyCode::KpComma => ",",
            KeyCode::KpEqualSign => "=",
            KeyCode::Intl1 => "Intl1",
            KeyCode::Intl2 => "Intl2",
            KeyCode::Intl3 => "Intl3",
            KeyCode::Intl4 => "Intl4",
            KeyCode::Intl5 => "Intl5",
            KeyCode::Intl6 => "Intl6",
            KeyCode::Intl7 => "Intl7",
            KeyCode::Intl8 => "Intl8",
            KeyCode::Intl9 => "Intl9",
            KeyCode::Lang1 => "Lang1",
            KeyCode::Lang2 => "Lang2",
            KeyCode::Lang3 => "Lang3",
            KeyCode::Lang4 => "Lang4",
            KeyCode::Lang5 => "Lang5",
            KeyCode::Lang6 => "Lang6",
            KeyCode::Lang7 => "Lang7",
            KeyCode::Lang8 => "Lang8",
            KeyCode::Lang9 => "Lang9",
            KeyCode::AltErase => "AltErase",
            KeyCode::SysReq => "SysReq",
            KeyCode::Cancel => "Cancel",
            KeyCode::Clear => "Clear",
            KeyCode::Prior => "Prior",
            KeyCode::Return => "Return",
            KeyCode::Separator => "Separator",
            KeyCode::Out => "Out",
            KeyCode::Oper => "Oper",
            KeyCode::ClearAgain => "ClearAgain",
            KeyCode::CrSel => "CrSel",
            KeyCode::ExSel => "ExSel",
            KeyCode::LCtrl => "LCtrl",
            KeyCode::LShift => "LShift",
            KeyCode::LAlt => "LAlt",
            KeyCode::LGui => "LGui",
            KeyCode::RCtrl => "RCtrl",
            KeyCode::RShift => "RShift",
            KeyCode::RAlt => "RAlt",
            KeyCode::RGui => "RGui",
            KeyCode::MediaPlayPause => "⏯",
            KeyCode::MediaStopCD => "⏹",
            KeyCode::MediaPreviousSong => "⏮",
            KeyCode::MediaNextSong => "⏭",
            KeyCode::MediaEjectCD => "⏏",
            KeyCode::MediaVolUp => "🔊",
            KeyCode::MediaVolDown => "🔉",
            KeyCode::MediaMute => "",
            KeyCode::MediaWWW => "ℹ",
            KeyCode::MediaBack => "⏪",
            KeyCode::MediaForward => "⏩",
            KeyCode::MediaStop => "⏹",
            KeyCode::MediaFind => "",
            KeyCode::MediaScrollUp => "",
            KeyCode::MediaScrollDown => "",
            KeyCode::MediaEdit => "",
            KeyCode::MediaSleep => "",
            KeyCode::MediaCoffee => "",
            KeyCode::MediaRefresh => "🔄",
            KeyCode::MediaCalc => "",
            _ => "",
        }
    }
}

/// A standard keyboard USB HID report.
///
/// It can handle any modifier and 6 keys.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct KbHidReport([u8; 8]);

impl core::iter::FromIterator<KeyCode> for KbHidReport {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = KeyCode>,
    {
        let mut res = Self::default();
        for kc in iter {
            res.pressed(kc);
        }
        res
    }
}

impl KbHidReport {
    /// Returns the byte slice corresponding to the report.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Add the given key code to the report. If the report is full,
    /// it will be set to `ErrorRollOver`.
    pub fn pressed(&mut self, kc: KeyCode) {
        use KeyCode::*;
        match kc {
            No => (),
            ErrorRollOver | PostFail | ErrorUndefined => self.set_all(kc),
            kc if kc.is_modifier() => self.0[0] |= kc.as_modifier_bit(),
            _ => self.0[2..]
                .iter_mut()
                .find(|c| **c == 0)
                .map(|c| *c = kc as u8)
                .unwrap_or_else(|| self.set_all(ErrorRollOver)),
        }
    }
    fn set_all(&mut self, kc: KeyCode) {
        for c in &mut self.0[2..] {
            *c = kc as u8;
        }
    }
}
