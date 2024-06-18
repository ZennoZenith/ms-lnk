pub mod file_attribute_flags;
pub mod file_size;
pub mod hotkey;
pub mod icon_index;
pub mod link_flags;
pub mod show_command;
pub mod time_bytes;

use file_attribute_flags::FileAttributesFlags;
use file_size::FileSize;
use hotkey::Hotkey;
use icon_index::IconIndex;
use link_flags::LinkFlags;
use show_command::ShowCommand;
use time_bytes::Filetime;

#[derive(Default, Debug, PartialEq)]
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
}

impl ShellLinkHeader {
    // pub fn new() -> Self {
    //     Self {}
    // }
}
