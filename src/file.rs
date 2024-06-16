use crate::shared::{const_prefix, hex_to_str, reverse_hex, str_to_hex};
use std::fs;

fn write_u8_hex(data: &Vec<u8>) {
    fs::write("./temp/test.hex", data).expect("Unable to write file");

    for d in data {
        print!("{:02x}, ", d);
    }
    println!();
    // for d in &mut_data {
    //     print!("{:02x}, ", d);
    // }
    // println!("{:?}", mut_data);
    println!();
    println!("Written data");
}

pub fn shell_link_header_write() {
    let mut shell_link_header: Vec<u8> = Vec::new();

    const HEADER: &str = "0000_004c";
    const LINK_CLSID: &str = "00021401_00000000_C0000000_00000046";
    shell_link_header.extend_from_slice(&reverse_hex(&str_to_hex(HEADER), 8));
    shell_link_header.extend_from_slice(&reverse_hex(&str_to_hex(&LINK_CLSID[0..8]), 8));
    shell_link_header.extend_from_slice(&str_to_hex(&LINK_CLSID[8..]));

    write_u8_hex(&shell_link_header);
}

pub fn shell_link_header_read() {
    let data = fs::read("/etc/hosts").expect("Unable to read file");
    println!("{}", data.len());
}

pub fn test_read() {
    // let data = fs::read("./temp/read_test.hex").expect("Unable to read file");
    let data = fs::read_to_string("./temp/read_test.hex").expect("Unable to read file");
    let data = data.replace(' ', "");
    let data = data.replace('\n', "").to_lowercase();

    let data_in_hex = str_to_hex(&data);
    let reconvert_to_str = hex_to_str(&data_in_hex);
    println!("{}", &data[..26]);
    println!("{}", &reconvert_to_str[..26]);
    println!("{}", hex_to_str(&const_prefix()));

    println!("{}", data == reconvert_to_str);
    // println!("{:?}", input);
    let _const_prefix_len = const_prefix().len();
    // print!("{:?}", &data[0..4]);
    // println!("{:?}", &data[..20]);
    // println!("{:?}", &data[..8]);
    println!("{:?}", &data[8..32 + 8]);
    // println!("{:?}", &data_in_hex[..8]);
    // let _link_flag = LinkFlags::read(&data_in_hex[const_prefix_len..(const_prefix_len + 4)]);
    // link_flag.print();
}
