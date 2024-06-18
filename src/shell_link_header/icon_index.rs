// 4 bytes
// REVIEW: this module is not tested
/// A 32-bit signed integer that specifies the index of an icon within a given icon location.
#[derive(Default, Debug, PartialEq)]
pub struct IconIndex(u32);
impl IconIndex {
    pub fn new(file_size: u32) -> Self {
        Self(file_size)
    }

    // OPTIMIZE
    pub fn from_bytes(data: &[u8; 4]) -> Self {
        let mut file_size: u32 = 0;
        for x in data.iter().rev() {
            file_size <<= 8;
            file_size += *x as u32;
        }
        Self(file_size)
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        // OPTIMIZE: use of array instead of vec
        let mut file_size_bytes = vec![];
        let mut file_size = self.0;
        for _ in 0..4 {
            file_size_bytes.push((file_size & 0b1111_1111) as u8);
            file_size >>= 8;
        }
        file_size_bytes.try_into().unwrap()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn icon_index_read_from_bytes() {
//         let file_size_bytes: [u8; 4] = [0x08, 0x90, 0x22, 0x00];
//         let file_size: u32 = 2_265_096;
//         assert_eq!(IconIndex::from_bytes(&file_size_bytes).0, file_size);
//     }

//     #[test]
//     fn icon_index_to_bytes() {
//         let file_size_bytes: [u8; 4] = [0x08, 0x90, 0x22, 0x00];
//         let file_size: u32 = 2_265_096;
//         assert_eq!(IconIndex(file_size).to_bytes(), file_size_bytes);
//     }
// }
