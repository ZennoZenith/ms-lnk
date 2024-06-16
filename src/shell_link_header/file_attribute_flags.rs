use crate::shared;

#[derive(Default, Debug, PartialEq)]
pub struct FileAttributesFlags {
    pub file_attribute_readonly: bool,
    pub file_attribute_hidden: bool,
    pub file_attribute_system: bool,
    pub reserved1: bool,
    pub file_attribute_directory: bool,
    pub file_attribute_archive: bool,
    pub reserved2: bool,
    pub file_attribute_normal: bool,
    pub file_attribute_temporary: bool,
    pub file_attribute_sparse_file: bool,
    pub file_attribute_reparse_point: bool,
    pub file_attribute_compressed: bool,
    pub file_attribute_offline: bool,
    pub file_attribute_not_content_indexed: bool,
    pub file_attribute_encrypted: bool,
}
impl FileAttributesFlags {
    fn default() -> Self {
        FileAttributesFlags {
            file_attribute_readonly: false,
            file_attribute_hidden: false,
            file_attribute_system: false,
            reserved1: false,
            file_attribute_directory: false,
            file_attribute_archive: false,
            reserved2: false,
            file_attribute_normal: false,
            file_attribute_temporary: false,
            file_attribute_sparse_file: false,
            file_attribute_reparse_point: false,
            file_attribute_compressed: false,
            file_attribute_offline: false,
            file_attribute_not_content_indexed: false,
            file_attribute_encrypted: false,
        }
    }
    pub fn read(data: &[u8; 4]) -> Self {
        let mut file_attribute_flag = Self::default();

        let reversed_byte = shared::reverse_byte(data[0]);
        file_attribute_flag.file_attribute_readonly = (reversed_byte >> 7 & 1) == 1;
        file_attribute_flag.file_attribute_hidden = (reversed_byte >> 6 & 1) == 1;
        file_attribute_flag.file_attribute_system = (reversed_byte >> 5 & 1) == 1;
        file_attribute_flag.reserved1 = (reversed_byte >> 4 & 1) == 1;
        file_attribute_flag.file_attribute_directory = (reversed_byte >> 3 & 1) == 1;
        file_attribute_flag.file_attribute_archive = (reversed_byte >> 2 & 1) == 1;
        file_attribute_flag.reserved2 = (reversed_byte >> 1 & 1) == 1;
        file_attribute_flag.file_attribute_normal = (reversed_byte & 1) == 1;

        // --------------------------------
        let reversed_byte = shared::reverse_byte(data[1]);
        file_attribute_flag.file_attribute_temporary = (reversed_byte >> 7 & 1) == 1;
        file_attribute_flag.file_attribute_sparse_file = (reversed_byte >> 6 & 1) == 1;
        file_attribute_flag.file_attribute_reparse_point = (reversed_byte >> 5 & 1) == 1;
        file_attribute_flag.file_attribute_compressed = (reversed_byte >> 4 & 1) == 1;
        file_attribute_flag.file_attribute_offline = (reversed_byte >> 3 & 1) == 1;
        file_attribute_flag.file_attribute_not_content_indexed = (reversed_byte >> 2 & 1) == 1;
        file_attribute_flag.file_attribute_encrypted = (reversed_byte >> 1 & 1) == 1;

        file_attribute_flag
    }
    pub fn to_bytes(&self) -> [u8; 4] {
        let mut flag_bytes = [0u8; 4];

        // OPTIMIZE
        // if self.has_link_target_idlist {
        //     flag_bytes[0] |= 0b1000_0000;
        // }
        if self.file_attribute_readonly {
            flag_bytes[0] |= 0b1000_0000;
        }
        if self.file_attribute_hidden {
            flag_bytes[0] |= 0b0100_0000;
        }
        if self.file_attribute_system {
            flag_bytes[0] |= 0b0010_0000;
        }
        if self.reserved1 {
            flag_bytes[0] |= 0b0001_0000;
        }
        if self.file_attribute_directory {
            flag_bytes[0] |= 0b0000_1000;
        }
        if self.file_attribute_archive {
            flag_bytes[0] |= 0b0000_0100;
        }
        if self.reserved2 {
            flag_bytes[0] |= 0b0000_0010;
        }
        if self.file_attribute_normal {
            flag_bytes[0] |= 0b0000_0001;
        }
        flag_bytes[0] = shared::reverse_byte(flag_bytes[0]);
        // ---------------------------------
        if self.file_attribute_temporary {
            flag_bytes[1] |= 0b1000_0000;
        }
        if self.file_attribute_sparse_file {
            flag_bytes[1] |= 0b0100_0000;
        }
        if self.file_attribute_reparse_point {
            flag_bytes[1] |= 0b0010_0000;
        }
        if self.file_attribute_compressed {
            flag_bytes[1] |= 0b0001_0000;
        }
        if self.file_attribute_offline {
            flag_bytes[1] |= 0b0000_1000;
        }
        if self.file_attribute_not_content_indexed {
            flag_bytes[1] |= 0b0000_0100;
        }
        if self.file_attribute_encrypted {
            flag_bytes[1] |= 0b0000_0010;
        }

        flag_bytes[1] = shared::reverse_byte(flag_bytes[1]);

        flag_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_attributes_flag_read() {
        let file_attributes_flag_bytes: [u8; 4] = [0x20, 0x00, 0x00, 0x00];
        let expected_file_attributes_flag = FileAttributesFlags {
            file_attribute_archive: true,
            ..Default::default()
        };

        assert_eq!(
            expected_file_attributes_flag,
            FileAttributesFlags::read(&file_attributes_flag_bytes)
        );
    }
    #[test]
    fn file_attribute_flag_to_bytes() {
        let file_attribute_flags = FileAttributesFlags {
            file_attribute_archive: true,
            ..Default::default()
        };
        let expected_file_attribute_bytes: [u8; 4] =
            [0x20, 0x00, 0x00, 0x00].to_vec().try_into().unwrap();
        assert_eq!(
            expected_file_attribute_bytes,
            file_attribute_flags.to_bytes()
        );
    }
}
