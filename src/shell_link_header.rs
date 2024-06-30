pub mod file_attribute_flags;
pub mod file_size;
pub mod hotkey;
pub mod icon_index;
pub mod link_flags;
pub mod show_command;
pub mod time_bytes;

use crate::link_flag_header::{link_info::LinkInfo, link_target_id_list::LinkTargetIDList};
use file_attribute_flags::FileAttributesFlags;
use file_size::FileSize;
use hotkey::Hotkey;
use icon_index::IconIndex;
use link_flags::LinkFlags;
use show_command::ShowCommand;
use time_bytes::Filetime;

/*
SHELL_LINK_HEADER:
Size: 76 bytes
------------------------------------------------------------------
|     0-7b     |     8-15b     |     16-23b     |     24-31b     |
------------------------------------------------------------------
|                 <u32> HeaderSize == 0x0000004C                 |
------------------------------------------------------------------
|   <CSID> LinkCLSID == 00021401-0000-0000-C000-000000000046     |
|                            16 B                                |
------------------------------------------------------------------
|                     <flags> LinkFlags                          |
------------------------------------------------------------------
|                   <flags> FileAttributes                       |
------------------------------------------------------------------
|                  <FILETIME> CreationTime                       |
|                            16 B                                |
------------------------------------------------------------------
|                   <FILETIME> AccessTime                        |
|                            16 B                                |
------------------------------------------------------------------
|                   <FILETIME> WriteTime                         |
|                            16 B                                |
------------------------------------------------------------------
|                        <u32> FileSize                          |
------------------------------------------------------------------
|                        <u32> IconIndex                         |
------------------------------------------------------------------
|                        <u32> ShowCommand                       |
------------------------------------------------------------------
|   <HotKeyFlags> HotKey      |            Reserved1             |
------------------------------------------------------------------
|                        Reserved2                               |
------------------------------------------------------------------
|                        Reserved3                               |
------------------------------------------------------------------
*/

// TODO: Remove in release
#[allow(dead_code)]
// ----
pub struct ShellLinkHeader {
    // 20 const byte prefix
    link_flags: LinkFlags,
    file_attrubute_flags: FileAttributesFlags,
    creation_time: Filetime,
    access_time: Filetime,
    write_time: Filetime,
    file_size: FileSize,
    icon_index: IconIndex,
    show_command: ShowCommand,
    hot_key: Hotkey,
    // reserver1 - 2 bytes - must be zero
    // reserver2 - 4 bytes - must be zero
    // reserver3 - 4 bytes - must be zero
    link_target_id_list: Option<LinkTargetIDList>,
    link_info: Option<LinkInfo>,
}

impl TryFrom<&[u8]> for ShellLinkHeader {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // u16::from_ne_bytes()
        if value.len() < 76 {
            return Err("Parse error: Invalid prefix Bytes");
        }
        const PREFIX_BYTES: [u8; 20] = [
            0x4c, 0x0, 0x0, 0x0, 0x1, 0x14, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0xc0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x46,
        ];

        if value[..20] != PREFIX_BYTES {
            return Err("Parse error: Invalid prefix Bytes");
        }

        let link_flags = LinkFlags::from_bytes(&value[20..24].try_into().unwrap());

        println!("{}", link_flags);
        // print!("To be tested bytes:         ");
        // for x in value.iter().take(20) {
        //     print!("{:#2x} ", x);
        // }
        println!();
        // dbg!(value);

        Err("Parse error")
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn lnk_parse() {
//         ShellLinkHeader::try_from()
//     }
// }
