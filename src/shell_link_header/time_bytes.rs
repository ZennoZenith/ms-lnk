/// 8 bytes
#[derive(Default, Debug, PartialEq)]
pub struct CreationTime {
    pub dw_low_date_time: u32,
    pub dw_high_date_time: u32,
}

impl CreationTime {
    fn default() -> Self {
        Self {
            dw_low_date_time: 0,
            dw_high_date_time: 0,
        }
    }

    pub fn read(_data: &[u8; 8]) -> Self {
        let time_bytes: CreationTime = Self::default();
        time_bytes
    }

    pub fn to_bytes(&self) -> [u8; 8] {
        let time_bytes = [0u8; 8];
        time_bytes
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn time_byte_read() {}
// }
