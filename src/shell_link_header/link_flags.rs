use crate::shared;

#[derive(Default, Debug, PartialEq)]
pub struct LinkFlags {
    pub has_link_target_idlist: bool,
    pub has_link_info: bool,
    pub has_name: bool,
    pub has_relative_path: bool,
    pub has_working_dir: bool,
    pub has_arguments: bool,
    pub has_icon_location: bool,
    pub is_unicode: bool,
    pub force_no_link_info: bool,
    pub has_exp_string: bool,
    pub run_in_separate_process: bool,
    pub unused1: bool,
    pub has_darwin_id: bool,
    pub run_as_user: bool,
    pub has_exp_icon: bool,
    pub no_pidl_alias: bool,
    pub unused2: bool,
    pub run_with_shim_layer: bool,
    pub force_no_link_track: bool,
    pub enable_target_metadata: bool,
    pub disable_link_path_tracking: bool,
    pub disable_known_folder_tracking: bool,
    pub disable_known_folder_alias: bool,
    pub allow_link_to_link: bool,
    pub unalias_on_save: bool,
    pub prefer_environment_path: bool,
    pub keep_local_idlist_for_unctarget: bool,
}

impl LinkFlags {
    pub fn from_bytes(data: &[u8; 4]) -> Self {
        let mut link_flag = Self::default();

        let reversed_byte = shared::reverse_byte(data[0]);
        link_flag.has_link_target_idlist = (reversed_byte >> 7 & 1) == 1;
        link_flag.has_link_info = (reversed_byte >> 6 & 1) == 1;
        link_flag.has_name = (reversed_byte >> 5 & 1) == 1;
        link_flag.has_relative_path = (reversed_byte >> 4 & 1) == 1;
        link_flag.has_working_dir = (reversed_byte >> 3 & 1) == 1;
        link_flag.has_arguments = (reversed_byte >> 2 & 1) == 1;
        link_flag.has_icon_location = (reversed_byte >> 1 & 1) == 1;
        link_flag.is_unicode = (reversed_byte & 1) == 1;
        // ---------------------------------
        let reversed_byte = shared::reverse_byte(data[1]);
        link_flag.force_no_link_info = (reversed_byte >> 7 & 1) == 1;
        link_flag.has_exp_string = (reversed_byte >> 6 & 1) == 1;
        link_flag.run_in_separate_process = (reversed_byte >> 5 & 1) == 1;
        link_flag.unused1 = (reversed_byte >> 4 & 1) == 1;
        link_flag.has_darwin_id = (reversed_byte >> 3 & 1) == 1;
        link_flag.run_as_user = (reversed_byte >> 2 & 1) == 1;
        link_flag.has_exp_icon = (reversed_byte >> 1 & 1) == 1;
        link_flag.no_pidl_alias = (reversed_byte & 1) == 1;
        // ---------------------------------
        let reversed_byte = shared::reverse_byte(data[2]);
        link_flag.unused2 = (data[2] >> 7 & 1) == 1;
        link_flag.run_with_shim_layer = (reversed_byte >> 6 & 1) == 1;
        link_flag.force_no_link_track = (reversed_byte >> 5 & 1) == 1;
        link_flag.enable_target_metadata = (reversed_byte >> 4 & 1) == 1;
        link_flag.disable_link_path_tracking = (reversed_byte >> 3 & 1) == 1;
        link_flag.disable_known_folder_tracking = (reversed_byte >> 2 & 1) == 1;
        link_flag.disable_known_folder_alias = (reversed_byte >> 1 & 1) == 1;
        link_flag.allow_link_to_link = (reversed_byte & 1) == 1;
        // ---------------------------------
        let reversed_byte = shared::reverse_byte(data[3]);
        link_flag.unalias_on_save = (reversed_byte >> 7 & 1) == 1;
        link_flag.prefer_environment_path = (reversed_byte >> 6 & 1) == 1;
        link_flag.keep_local_idlist_for_unctarget = (reversed_byte >> 5 & 1) == 1;

        link_flag
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        let mut flag_bytes = [0u8; 4];

        // OPTIMIZE
        if self.has_link_target_idlist {
            flag_bytes[0] |= 0b1000_0000;
        }
        if self.has_link_info {
            flag_bytes[0] |= 0b0100_0000;
        }
        if self.has_name {
            flag_bytes[0] |= 0b0010_0000;
        }
        if self.has_relative_path {
            flag_bytes[0] |= 0b0001_0000;
        }
        if self.has_working_dir {
            flag_bytes[0] |= 0b0000_1000;
        }
        if self.has_arguments {
            flag_bytes[0] |= 0b0000_0100;
        }
        if self.has_icon_location {
            flag_bytes[0] |= 0b0000_0010;
        }
        if self.is_unicode {
            flag_bytes[0] |= 0b0000_0001;
        }
        flag_bytes[0] = shared::reverse_byte(flag_bytes[0]);
        // ---------------------------------
        if self.force_no_link_info {
            flag_bytes[1] |= 0b1000_0000;
        };
        if self.has_exp_string {
            flag_bytes[1] |= 0b0100_0000;
        };
        if self.run_in_separate_process {
            flag_bytes[1] |= 0b0010_0000;
        };
        if self.unused1 {
            flag_bytes[1] |= 0b0001_0000;
        };
        if self.has_darwin_id {
            flag_bytes[1] |= 0b0000_1000;
        };
        if self.run_as_user {
            flag_bytes[1] |= 0b0000_0100;
        };
        if self.has_exp_icon {
            flag_bytes[1] |= 0b0000_0010;
        };
        if self.no_pidl_alias {
            flag_bytes[1] |= 0b0000_0001;
        };
        flag_bytes[1] = shared::reverse_byte(flag_bytes[1]);
        // ---------------------------------
        if self.unused1 {
            flag_bytes[2] |= 0b1000_0000;
        };
        if self.run_with_shim_layer {
            flag_bytes[2] |= 0b0100_0000;
        };
        if self.force_no_link_track {
            flag_bytes[2] |= 0b0010_0000;
        };
        if self.enable_target_metadata {
            flag_bytes[2] |= 0b0001_0000;
        };
        if self.disable_link_path_tracking {
            flag_bytes[2] |= 0b0000_1000;
        };
        if self.disable_known_folder_tracking {
            flag_bytes[2] |= 0b0000_0100;
        };
        if self.disable_known_folder_alias {
            flag_bytes[2] |= 0b0000_0010;
        };
        if self.allow_link_to_link {
            flag_bytes[2] |= 0b0000_0001;
        };
        flag_bytes[2] = shared::reverse_byte(flag_bytes[2]);
        // ---------------------------------
        if self.unalias_on_save {
            flag_bytes[3] |= 0b1000_0000;
        };
        if self.prefer_environment_path {
            flag_bytes[3] |= 0b0100_0000;
        };
        if self.keep_local_idlist_for_unctarget {
            flag_bytes[3] |= 0b0010_0000;
        };
        flag_bytes[3] = shared::reverse_byte(flag_bytes[3]);

        flag_bytes
    }

    pub fn print(&self) {
        println!("{:#?}", self);
    }
}

/*
let link_flags = LinkFlags {
    has_link_target_idlist: true,
    has_link_info: true,
    has_relative_path: true,
    has_working_dir: true,
    is_unicode: true,
    enable_target_metadata: true,
    ..Default::default()
};
let link_bytes: [u8; 4] = [0x9B, 0x00, 0x08, 0x00];
// dbg!(&link_flags);
println!(
    "0b{:08b} 0x{:02x} - 0b{:08b} 0x{:02x}",
    link_flags.to_bytes()[0],
    link_flags.to_bytes()[0],
    link_bytes[0],
    link_bytes[0],
);
println!(
    "0b{:08b} 0x{:02x} - 0b{:08b} 0x{:02x}",
    link_flags.to_bytes()[1],
    link_flags.to_bytes()[1],
    link_bytes[1],
    link_bytes[1],
);
println!(
    "0b{:08b} 0x{:02x} - 0b{:08b} 0x{:02x}",
    link_flags.to_bytes()[2],
    link_flags.to_bytes()[2],
    link_bytes[2],
    link_bytes[2],
);
println!(
    "0b{:08b} 0x{:02x} - 0b{:08b} 0x{:02x}",
    link_flags.to_bytes()[3],
    link_flags.to_bytes()[3],
    link_bytes[3],
    link_bytes[3],
);
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_flag_read() {
        let link_flag_bytes: [u8; 4] = [0x9B, 0x00, 0x08, 0x00];
        let expected_link_flag = LinkFlags {
            has_link_target_idlist: true,
            has_link_info: true,
            has_relative_path: true,
            has_working_dir: true,
            is_unicode: true,
            enable_target_metadata: true,
            ..Default::default()
        };

        assert_eq!(expected_link_flag, LinkFlags::from_bytes(&link_flag_bytes));
    }
    #[test]
    fn link_flag_to_bytes() {
        let link_flags = LinkFlags {
            has_link_target_idlist: true,
            has_link_info: true,
            has_relative_path: true,
            has_working_dir: true,
            is_unicode: true,
            enable_target_metadata: true,
            ..Default::default()
        };
        let expected_link_bytes: [u8; 4] = [0x9B, 0x00, 0x08, 0x00].to_vec().try_into().unwrap();
        assert_eq!(expected_link_bytes, link_flags.to_bytes());
    }
}
