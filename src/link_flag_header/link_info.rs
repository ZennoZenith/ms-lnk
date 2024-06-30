/// The LinkInfo structure specifies information necessary to resolve a link
/// target if it is not found in its original location. This includes information
/// about the volume that the target was stored on, the mapped drive letter, and a
/// Universal Naming Convention (UNC) form of the path if one existed when the link
/// was created. For more details about UNC paths

// LinkInfoSize,
// LinkInfoHeaderSize,
// LinkInfoFlags
// VolumeIDOffset
// LocalBasePathOffset
// CommonNetworkRelativeLinkOffset
// CommonPathSuffixOffset
// LocalBasePathOffsetUnicode (optional)
// CommonPathSuffixOffsetUnicode (optional)
// VolumeID (variable)
// LocalBasePath (variable)
// CommonNetworkRelativeLink (variable)
// CommonPathSuffix (variable)
// LocalBasePathUnicode (variable)
// CommonPathSuffixUnicode (variable)

#[derive(Debug, PartialEq)]
pub struct LinkInfoFlag {
    volume_idand_local_base_path: bool,
    common_network_relative_link_and_path_suffix: bool,
}

#[derive(Debug, PartialEq)]
pub struct LinkInfo {
    /**
    A 32-bit, unsigned integer that specifies the size, in bytes, of the LinkInfo structure. All offsets specified in this structure MUST be less than this value, and all strings contained in this structure MUST fit within the extent defined by this size.
    */
    link_info_size: u32,

    /**
    A 32-bit, unsigned integer that specifies the size, in bytes, of the LinkInfo header section
    Value is 0x0000001C if offset to the otpional fields are not specified
    Value >= 0x00000024 if ofset to the optional field are specified
    */
    link_info_header_size: u32,
    /**
    Flags that specify whether the VolumeID, LocalBasePath,
    LocalBasePathUnicode, and CommonNetworkRelativeLink fields are present in this
    structure.
    value is unsigned 32 bits
    */
    link_info_flags: LinkInfoFlag,
}

impl LinkInfo {
    // pub fn from_bytes(data: &[u8; 4]) -> Self {
    //     let mut file_size: u32 = 0;
    //     for x in data.iter().rev() {
    //         file_size <<= 8;
    //         file_size += *x as u32;
    //     }
    //     Self(file_size)
    // }

    // pub fn to_bytes(&self) -> [u8; 4] {
    //     // OPTIMIZE: use of array instead of vec
    //     let mut file_size_bytes = vec![];
    //     let mut file_size = self.0;
    //     for _ in 0..4 {
    //         file_size_bytes.push((file_size & 0b1111_1111) as u8);
    //         file_size >>= 8;
    //     }
    //     file_size_bytes.try_into().unwrap()
    // }
}
