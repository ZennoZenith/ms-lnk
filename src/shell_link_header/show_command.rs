const SW_SHOWNORMAL: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
const SW_SHOWMAXIMIZED: [u8; 4] = [0x03, 0x00, 0x00, 0x00];
const SW_SHOWMINNOACTIVE: [u8; 4] = [0x07, 0x00, 0x00, 0x00];

#[derive(Default, Debug, PartialEq)]
pub enum ShowCommand {
    #[default]
    SwShownormal,
    SwShowmaximized,
    SwShowminnoactive,
}

impl ShowCommand {
    pub fn from_bytes(data: &[u8; 4]) -> Self {
        match *data {
            SW_SHOWNORMAL => Self::SwShownormal,
            SW_SHOWMAXIMIZED => Self::SwShowmaximized,
            SW_SHOWMINNOACTIVE => Self::SwShowminnoactive,
            _ => Self::default(),
        }
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        match self {
            Self::SwShownormal => SW_SHOWNORMAL,
            Self::SwShowmaximized => SW_SHOWMAXIMIZED,
            Self::SwShowminnoactive => SW_SHOWMINNOACTIVE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_command_read_from_bytes() {
        let show_command_bytes: [u8; 4] = [0x03, 0x00, 0x00, 0x00];
        assert_eq!(
            ShowCommand::from_bytes(&show_command_bytes),
            ShowCommand::SwShowmaximized
        );
    }

    #[test]
    fn show_command_read_from_any_bytes() {
        let show_command_bytes: [u8; 4] = [0x43, 0x20, 0x21, 0xa0];
        assert_eq!(
            ShowCommand::from_bytes(&show_command_bytes),
            ShowCommand::SwShownormal
        );
    }

    #[test]
    fn show_command_to_bytes() {
        assert_eq!(
            ShowCommand::SwShowminnoactive.to_bytes(),
            SW_SHOWMINNOACTIVE
        );
    }

    #[test]
    fn show_command_default_to_bytes() {
        assert_eq!(ShowCommand::default().to_bytes(), SW_SHOWNORMAL);
    }
}
