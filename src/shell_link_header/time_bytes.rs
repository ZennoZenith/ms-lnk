/// 8 bytes
/*
9e53b8d011bfda01
Windows Filetime (UTC)	2024-06-15 10:50:19.7941150 Z
Windows Filetime	2024-06-15 16:20:19.7941150 +05:30

009185e8f48fd901
LDAP timestamp: 133295946020000000 -> 0x1D98FF4E8859100
Windows Filetime (UTC)	2023-05-26 17:10:02.0000000 Z
2023-05-26 22:40:02.0000000 +05:30

00 91 85 e8 f4 8f d9 01
01 D9 8F F4 E8 85 91 00


D0E9EEF21515C901
Windows Filetime (UTC)	2008-09-12 20:27:17.1010000 Z
2008-09-13 01:57:17.1010000 +05:30
*/
use std::time::SystemTime;

use crate::shared::{ldap_timestamp_to_unix, unix_to_ldap_timestamp};

#[derive(Debug, PartialEq)]
pub struct Filetime {
    ldap_timestamp: u64,
}

impl Default for Filetime {
    fn default() -> Self {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => {
                println!("n.seconds: {}", n.as_secs());
                Self {
                    ldap_timestamp: unix_to_ldap_timestamp(n.as_secs() as i64, 0),
                }
            }
            // SystemTime before UNIX EPOCH!
            Err(_) => {
                println!("n.seconds: ERROR");
                Self {
                    ldap_timestamp: unix_to_ldap_timestamp(0, 0),
                }
            }
        }
    }
}

impl Filetime {
    pub fn new_from_ldap(ldap_timestamp: u64) -> Self {
        Self { ldap_timestamp }
    }

    pub fn new_from_unix(unix_seconds: i64, unix_nanoseconds: u64) -> Self {
        Self {
            ldap_timestamp: unix_to_ldap_timestamp(unix_seconds, unix_nanoseconds),
        }
    }

    // OPTIMIZE
    pub fn read(data: &[u8; 8]) -> Self {
        let mut reversed_bytes = data.to_owned();
        reversed_bytes.reverse();
        let mut ldap_timestamp: u64 = 0;

        for x in reversed_bytes.iter() {
            ldap_timestamp <<= 8;
            ldap_timestamp += *x as u64;
        }
        println!();
        Self { ldap_timestamp }
    }

    pub fn to_bytes(&self) -> [u8; 8] {
        let mut time_bytes = vec![];
        let mut ldap_timestamp = self.ldap_timestamp;
        for _ in 0..8 {
            time_bytes.push((ldap_timestamp & 0b1111_1111) as u8);
            ldap_timestamp >>= 8;
        }
        time_bytes.try_into().unwrap()
    }

    pub fn get_unix_time(&self) -> (i64, u64) {
        ldap_timestamp_to_unix(self.ldap_timestamp)
    }

    pub fn get_ldap_time(&self) -> u64 {
        self.ldap_timestamp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 009185e8f48fd901
    // LDAP timestamp: 133295946020000000 -> 0x1D98FF4E8859100
    #[test]
    fn time_byte_read_from_bytes() {
        let time_bytes: [u8; 8] = [0x00, 0x91, 0x85, 0xe8, 0xf4, 0x8f, 0xd9, 0x01];

        assert_eq!(
            Filetime::read(&time_bytes).get_ldap_time(),
            133295946020000000
        );
    }

    #[test]
    fn ldap_time_to_bytes() {
        let time_bytes: [u8; 8] = [0x00, 0x91, 0x85, 0xe8, 0xf4, 0x8f, 0xd9, 0x01];
        let ldap_timestamp: u64 = 133295946020000000;
        assert_eq!(
            Filetime::new_from_ldap(ldap_timestamp).to_bytes(),
            time_bytes
        );
    }
}
