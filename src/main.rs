#![allow(unused)]
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::prelude::*;
use chrono::TimeDelta;

fn main() {
    let dt = Utc.with_ymd_and_hms(2008, 9, 12, 20, 27, 17).unwrap(); // `2008-09-12T20:27:17Z`

    dbg!(dt);
    chat_gpt();
    // 9/12/08, 8:27:17PM
    // 3D 0E 12 AE A7 C8 9D

    // D0 E9 EE F2 15 15 C9 01
    let time_bytes: [u8; 8] = [0xD0, 0xE9, 0xEE, 0xF2, 0x15, 0x15, 0xC9, 0x01];
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
