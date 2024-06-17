#![allow(unused)]
use chrono::prelude::*;
use std::fs::{self, FileTimes};
use std::time::{SystemTime, UNIX_EPOCH};

use ms_lnk::shared::{ldap_timestamp_to_unix, reverse_byte};
use ms_lnk::shell_link_header::time_bytes::Filetime;

fn main() {
    // chat_gpt();
    // test_read();

    // print!("Time bytes:         ");
    // let time_bytes: [u8; 8] = [0xD0, 0xE9, 0xEE, 0xF2, 0x15, 0x15, 0xC9, 0x01];
    // for x in time_bytes.iter() {
    //     print!("{:#2x} ", x);
    // }
    // println!();

    let timestamp = ldap_timestamp_to_unix(Filetime::default().get_ldap_time());

    let datetime = DateTime::from_timestamp(timestamp.0, 0).unwrap();

    // Create a normal DateTime from the NaiveDateTime

    // Format the datetime how you want
    let newdate = datetime.format("%Y-%m-%d %H:%M:%S");

    // Print the newly formatted date and time
    println!("new data: {}", newdate);
    println!("new data: {:?}", ldap_timestamp_to_unix(133295946020000000));
    let temp = Filetime::new_from_ldap(133295946020000000);
    println!("{:?}", temp);
    print!("Time bytes:         ");
    // let time_bytes: [u8; 8] = [0xD0, 0xE9, 0xEE, 0xF2, 0x15, 0x15, 0xC9, 0x01];
    // for x in time_bytes.iter() {
    for x in temp.to_bytes().iter() {
        print!("{:#2x} ", x);
    }
    println!();
}

fn chat_gpt() {
    // Get the current system time
    let current_time = SystemTime::now();
    // Get the duration since UNIX_EPOCH (January 1, 1970)
    let duration_since_epoch = current_time.duration_since(UNIX_EPOCH).unwrap();

    // Convert the duration to the number of 100-nanosecond intervals since January 1, 1601
    // 1 second = 10^9 nanoseconds
    // 1 nanosecond / 100 = 100-nanosecond intervals
    let intervals_since_1601 = duration_since_epoch.as_secs() * 10_000_000
        + u64::from(duration_since_epoch.subsec_nanos()) / 100;

    println!("Intervals since January 1, 1601: {}", intervals_since_1601);
    println!("In hex {:#16x}", intervals_since_1601);
}

fn test_read() {
    let data = fs::read("./temp/Everything.lnk").expect("Unable to read file");

    print!("Time bytes:         ");
    // let time_bytes: [u8; 8] = [0xD0, 0xE9, 0xEE, 0xF2, 0x15, 0x15, 0xC9, 0x01];
    for x in &data[28..36] {
        print!("{:#2x} ", x);
    }
    println!()
}
