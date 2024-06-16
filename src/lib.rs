pub use shell_link_header::{file_attribute_flags::FileAttributesFlags, link_flags::LinkFlags};

pub mod file;
pub mod shared;
pub mod shell_link_header;

pub fn const_bytes() -> [u8; 20] {
    const HEADER_SIZE: [u8; 4] = [0x4c, 0x00, 0x00, 0x00];
    const LINK_CLSID: [u8; 16] = [
        0x01, 0x14, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ];

    // OPTIMIZE:
    let whole: [u8; 20] = {
        let mut whole: [u8; 20] = [0; 20];
        let (one, two) = whole.split_at_mut(HEADER_SIZE.len());
        one.copy_from_slice(&HEADER_SIZE);
        two.copy_from_slice(&LINK_CLSID);
        whole
    };
    whole
}

#[derive(Default, Debug, PartialEq)]
pub struct Lnk {
    link_flags: LinkFlags,
    file_attrubute_flags: FileAttributesFlags,
}

impl Lnk {
    // pub fn new() -> Self {
    //     Self {}
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn example_test() {
//         assert_eq!();
//     }
// }
