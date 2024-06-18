pub mod file_attribute_flags;
pub mod file_size;
pub mod link_flags;
pub mod time_bytes;

use file_attribute_flags::FileAttributesFlags;
use file_size::FileSize;
use link_flags::LinkFlags;
use time_bytes::Filetime;

#[derive(Default, Debug, PartialEq)]
pub struct ShellLinkHeader {
    link_flags: LinkFlags,
    file_attrubute_flags: FileAttributesFlags,
    creation_time: Filetime,
    access_time: Filetime,
    write_time: Filetime,
    file_size: FileSize,
}

impl ShellLinkHeader {
    // pub fn new() -> Self {
    //     Self {}
    // }
}
