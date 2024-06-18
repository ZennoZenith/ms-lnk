// TODO: This module is untested
const NOT_ASSIGNED: u8 = 0x00;
const KEY_0: u8 = 0x30;
const KEY_1: u8 = 0x31;
const KEY_2: u8 = 0x32;
const KEY_3: u8 = 0x33;
const KEY_4: u8 = 0x34;
const KEY_5: u8 = 0x35;
const KEY_6: u8 = 0x36;
const KEY_7: u8 = 0x37;
const KEY_8: u8 = 0x38;
const KEY_9: u8 = 0x39;
const KEY_A: u8 = 0x41;
const KEY_B: u8 = 0x42;
const KEY_C: u8 = 0x43;
const KEY_D: u8 = 0x44;
const KEY_E: u8 = 0x45;
const KEY_F: u8 = 0x46;
const KEY_G: u8 = 0x47;
const KEY_H: u8 = 0x48;
const KEY_I: u8 = 0x49;
const KEY_J: u8 = 0x4A;
const KEY_K: u8 = 0x4B;
const KEY_L: u8 = 0x4C;
const KEY_M: u8 = 0x4D;
const KEY_N: u8 = 0x4E;
const KEY_O: u8 = 0x4F;
const KEY_P: u8 = 0x50;
const KEY_Q: u8 = 0x51;
const KEY_R: u8 = 0x52;
const KEY_S: u8 = 0x53;
const KEY_T: u8 = 0x54;
const KEY_U: u8 = 0x55;
const KEY_V: u8 = 0x56;
const KEY_W: u8 = 0x57;
const KEY_X: u8 = 0x58;
const KEY_Y: u8 = 0x59;
const KEY_Z: u8 = 0x5A;
const KEY_F1: u8 = 0x70;
const KEY_F2: u8 = 0x71;
const KEY_F3: u8 = 0x72;
const KEY_F4: u8 = 0x73;
const KEY_F5: u8 = 0x74;
const KEY_F6: u8 = 0x75;
const KEY_F7: u8 = 0x76;
const KEY_F8: u8 = 0x77;
const KEY_F9: u8 = 0x78;
const KEY_F10: u8 = 0x79;
const KEY_F11: u8 = 0x7A;
const KEY_F12: u8 = 0x7B;
const KEY_F13: u8 = 0x7C;
const KEY_F14: u8 = 0x7D;
const KEY_F15: u8 = 0x7E;
const KEY_F16: u8 = 0x7F;
const KEY_F17: u8 = 0x80;
const KEY_F18: u8 = 0x81;
const KEY_F19: u8 = 0x82;
const KEY_F20: u8 = 0x83;
const KEY_F21: u8 = 0x84;
const KEY_F22: u8 = 0x85;
const KEY_F23: u8 = 0x86;
const KEY_F24: u8 = 0x87;
const KEY_NUM_LOCK: u8 = 0x90;
const KEY_SCROLL_LOCK: u8 = 0x91;

#[derive(Default, Debug, PartialEq)]
pub enum HotkeyLowByte {
    #[default]
    NotAssigned,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    KeyF1,
    KeyF2,
    KeyF3,
    KeyF4,
    KeyF5,
    KeyF6,
    KeyF7,
    KeyF8,
    KeyF9,
    KeyF10,
    KeyF11,
    KeyF12,
    KeyF13,
    KeyF14,
    KeyF15,
    KeyF16,
    KeyF17,
    KeyF18,
    KeyF19,
    KeyF20,
    KeyF21,
    KeyF22,
    KeyF23,
    KeyF24,
    KeyNumLock,
    KeyScrollLock,
}

impl HotkeyLowByte {
    pub fn to_byte(&self) -> u8 {
        match self {
            HotkeyLowByte::NotAssigned => NOT_ASSIGNED,
            HotkeyLowByte::Key0 => KEY_0,
            HotkeyLowByte::Key1 => KEY_1,
            HotkeyLowByte::Key2 => KEY_2,
            HotkeyLowByte::Key3 => KEY_3,
            HotkeyLowByte::Key4 => KEY_4,
            HotkeyLowByte::Key5 => KEY_5,
            HotkeyLowByte::Key6 => KEY_6,
            HotkeyLowByte::Key7 => KEY_7,
            HotkeyLowByte::Key8 => KEY_8,
            HotkeyLowByte::Key9 => KEY_9,
            HotkeyLowByte::KeyA => KEY_A,
            HotkeyLowByte::KeyB => KEY_B,
            HotkeyLowByte::KeyC => KEY_C,
            HotkeyLowByte::KeyD => KEY_D,
            HotkeyLowByte::KeyE => KEY_E,
            HotkeyLowByte::KeyF => KEY_F,
            HotkeyLowByte::KeyG => KEY_G,
            HotkeyLowByte::KeyH => KEY_H,
            HotkeyLowByte::KeyI => KEY_I,
            HotkeyLowByte::KeyJ => KEY_J,
            HotkeyLowByte::KeyK => KEY_K,
            HotkeyLowByte::KeyL => KEY_L,
            HotkeyLowByte::KeyM => KEY_M,
            HotkeyLowByte::KeyN => KEY_N,
            HotkeyLowByte::KeyO => KEY_O,
            HotkeyLowByte::KeyP => KEY_P,
            HotkeyLowByte::KeyQ => KEY_Q,
            HotkeyLowByte::KeyR => KEY_R,
            HotkeyLowByte::KeyS => KEY_S,
            HotkeyLowByte::KeyT => KEY_T,
            HotkeyLowByte::KeyU => KEY_U,
            HotkeyLowByte::KeyV => KEY_V,
            HotkeyLowByte::KeyW => KEY_W,
            HotkeyLowByte::KeyX => KEY_X,
            HotkeyLowByte::KeyY => KEY_Y,
            HotkeyLowByte::KeyZ => KEY_Z,
            HotkeyLowByte::KeyF1 => KEY_F1,
            HotkeyLowByte::KeyF2 => KEY_F2,
            HotkeyLowByte::KeyF3 => KEY_F3,
            HotkeyLowByte::KeyF4 => KEY_F4,
            HotkeyLowByte::KeyF5 => KEY_F5,
            HotkeyLowByte::KeyF6 => KEY_F6,
            HotkeyLowByte::KeyF7 => KEY_F7,
            HotkeyLowByte::KeyF8 => KEY_F8,
            HotkeyLowByte::KeyF9 => KEY_F9,
            HotkeyLowByte::KeyF10 => KEY_F10,
            HotkeyLowByte::KeyF11 => KEY_F11,
            HotkeyLowByte::KeyF12 => KEY_F12,
            HotkeyLowByte::KeyF13 => KEY_F13,
            HotkeyLowByte::KeyF14 => KEY_F14,
            HotkeyLowByte::KeyF15 => KEY_F15,
            HotkeyLowByte::KeyF16 => KEY_F16,
            HotkeyLowByte::KeyF17 => KEY_F17,
            HotkeyLowByte::KeyF18 => KEY_F18,
            HotkeyLowByte::KeyF19 => KEY_F19,
            HotkeyLowByte::KeyF20 => KEY_F20,
            HotkeyLowByte::KeyF21 => KEY_F21,
            HotkeyLowByte::KeyF22 => KEY_F22,
            HotkeyLowByte::KeyF23 => KEY_F23,
            HotkeyLowByte::KeyF24 => KEY_F24,
            HotkeyLowByte::KeyNumLock => KEY_NUM_LOCK,
            HotkeyLowByte::KeyScrollLock => KEY_SCROLL_LOCK,
        }
    }

    // TODO: Define custom ERROR
    pub fn from_byte(data: u8) -> Result<Self, String> {
        match data {
            NOT_ASSIGNED => Ok(Self::NotAssigned),
            KEY_0 => Ok(Self::Key0),
            KEY_1 => Ok(Self::Key1),
            KEY_2 => Ok(Self::Key2),
            KEY_3 => Ok(Self::Key3),
            KEY_4 => Ok(Self::Key4),
            KEY_5 => Ok(Self::Key5),
            KEY_6 => Ok(Self::Key6),
            KEY_7 => Ok(Self::Key7),
            KEY_8 => Ok(Self::Key8),
            KEY_9 => Ok(Self::Key9),
            KEY_A => Ok(Self::KeyA),
            KEY_B => Ok(Self::KeyB),
            KEY_C => Ok(Self::KeyC),
            KEY_D => Ok(Self::KeyD),
            KEY_E => Ok(Self::KeyE),
            KEY_F => Ok(Self::KeyF),
            KEY_G => Ok(Self::KeyG),
            KEY_H => Ok(Self::KeyH),
            KEY_I => Ok(Self::KeyI),
            KEY_J => Ok(Self::KeyJ),
            KEY_K => Ok(Self::KeyK),
            KEY_L => Ok(Self::KeyL),
            KEY_M => Ok(Self::KeyM),
            KEY_N => Ok(Self::KeyN),
            KEY_O => Ok(Self::KeyO),
            KEY_P => Ok(Self::KeyP),
            KEY_Q => Ok(Self::KeyQ),
            KEY_R => Ok(Self::KeyR),
            KEY_S => Ok(Self::KeyS),
            KEY_T => Ok(Self::KeyT),
            KEY_U => Ok(Self::KeyU),
            KEY_V => Ok(Self::KeyV),
            KEY_W => Ok(Self::KeyW),
            KEY_X => Ok(Self::KeyX),
            KEY_Y => Ok(Self::KeyY),
            KEY_Z => Ok(Self::KeyZ),
            KEY_F1 => Ok(Self::KeyF1),
            KEY_F2 => Ok(Self::KeyF2),
            KEY_F3 => Ok(Self::KeyF3),
            KEY_F4 => Ok(Self::KeyF4),
            KEY_F5 => Ok(Self::KeyF5),
            KEY_F6 => Ok(Self::KeyF6),
            KEY_F7 => Ok(Self::KeyF7),
            KEY_F8 => Ok(Self::KeyF8),
            KEY_F9 => Ok(Self::KeyF9),
            KEY_F10 => Ok(Self::KeyF10),
            KEY_F11 => Ok(Self::KeyF11),
            KEY_F12 => Ok(Self::KeyF12),
            KEY_F13 => Ok(Self::KeyF13),
            KEY_F14 => Ok(Self::KeyF14),
            KEY_F15 => Ok(Self::KeyF15),
            KEY_F16 => Ok(Self::KeyF16),
            KEY_F17 => Ok(Self::KeyF17),
            KEY_F18 => Ok(Self::KeyF18),
            KEY_F19 => Ok(Self::KeyF19),
            KEY_F20 => Ok(Self::KeyF20),
            KEY_F21 => Ok(Self::KeyF21),
            KEY_F22 => Ok(Self::KeyF22),
            KEY_F23 => Ok(Self::KeyF23),
            KEY_F24 => Ok(Self::KeyF24),
            KEY_NUM_LOCK => Ok(Self::KeyNumLock),
            KEY_SCROLL_LOCK => Ok(Self::KeyScrollLock),
            _ => Err("Invalid Hotkey Low Byte".to_string()),
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub enum HotkeyHighByte {
    #[default]
    NoModifier,
    KeyShift,
    KeyControl,
    KeyAlt,
    KeyControlShift,
    KeyAltShift,
    KeyControlAlt,
    KeyControlAltShift,
}

impl HotkeyHighByte {
    pub fn from_byte(data: u8) -> Result<Self, String> {
        match data {
            0x00 => Ok(HotkeyHighByte::NoModifier),
            0x01 => Ok(HotkeyHighByte::KeyShift),
            0x02 => Ok(HotkeyHighByte::KeyControl),
            0x04 => Ok(HotkeyHighByte::KeyAlt),
            0x03 => Ok(HotkeyHighByte::KeyControlShift),
            0x05 => Ok(HotkeyHighByte::KeyAltShift),
            0x06 => Ok(HotkeyHighByte::KeyControlAlt),
            0x07 => Ok(HotkeyHighByte::KeyControlAltShift),
            _ => Err("Invalid Hotkey High Byte".to_string()),
        }
    }
    pub fn to_byte(&self) -> u8 {
        match self {
            HotkeyHighByte::NoModifier => 0x00,
            HotkeyHighByte::KeyShift => 0x01,
            HotkeyHighByte::KeyControl => 0x02,
            HotkeyHighByte::KeyAlt => 0x04,
            HotkeyHighByte::KeyControlShift => 0x03,
            HotkeyHighByte::KeyAltShift => 0x05,
            HotkeyHighByte::KeyControlAlt => 0x06,
            HotkeyHighByte::KeyControlAltShift => 0x07,
        }
    }
}

// 2 bytes
#[derive(Default, Debug, PartialEq)]
pub struct Hotkey {
    low_byte: HotkeyLowByte,
    high_byte: HotkeyHighByte,
}

impl Hotkey {
    // TODO:
    pub fn from_bytes(_data: &[u8; 2]) -> Self {
        Hotkey::default()
    }
    // TODO:
    pub fn to_bytes(&self) -> [u8; 2] {
        [0, 0]
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test() {}
// }
