const TIME_FROM_LDAP_START_TO_UNIX_START_IN_SEC: u64 = 11_644_473_600;
const FACTOR_TO_CONVERT_LDAP_TO_UNIX_SECONDS: u64 = 10_000_000;

/// returns (seconds, nano seconds)
pub fn ldap_timestamp_to_unix(ldap: u64) -> (i64, u64) {
    (
        ((ldap / FACTOR_TO_CONVERT_LDAP_TO_UNIX_SECONDS)
            - TIME_FROM_LDAP_START_TO_UNIX_START_IN_SEC) as i64,
        (ldap % FACTOR_TO_CONVERT_LDAP_TO_UNIX_SECONDS) * 100,
    )
}

pub fn unix_to_ldap_timestamp(unix_seconds: i64, unix_nanoseconds: u64) -> u64 {
    // multiply by 10 000 000 to get 100 nano seconds
    ((unix_seconds as u64 + TIME_FROM_LDAP_START_TO_UNIX_START_IN_SEC)
        * FACTOR_TO_CONVERT_LDAP_TO_UNIX_SECONDS)
        + (unix_nanoseconds / 100)
}

pub fn str_to_hex(input: &str) -> Vec<u8> {
    let mut hex_vec: Vec<u8> = Vec::new();
    let input = input.replace('_', "");

    // FIXME: add padding for odd input length
    // OPTIMIZE: add another map to convert string to hex
    let chunks: Vec<&str> = input
        .as_bytes()
        .chunks(2)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect();
    for chunk in &chunks {
        hex_vec.push(u8::from_str_radix(chunk, 16).unwrap());
    }
    hex_vec
}

pub fn hex_to_str(input: &[u8]) -> String {
    // OPTIMIZE:
    let mut hex_str = String::new();
    for ele in input {
        hex_str.push_str(format!("{:x}", ele).as_str());
    }
    hex_str
}

pub fn reverse_hex(data: &[u8], chunk_size: usize) -> Vec<u8> {
    let mut mut_data: Vec<u8> = Vec::new();
    if data.is_empty() {
        return mut_data;
    }

    // HACK: reminder chunk will be removed
    let mut i = 0;
    while i < data.len() {
        let end = (i + chunk_size).min(data.len());
        let mut temp = data[i..end].to_owned();
        temp.reverse();
        mut_data.extend(temp);
        i += chunk_size;
    }
    mut_data
}

pub fn const_prefix() -> Vec<u8> {
    let mut shell_link_header: Vec<u8> = Vec::new();

    const HEADER: &str = "0000_004c";
    const LINK_CLSID: &str = "00021401_00000000_C0000000_00000046";
    shell_link_header.extend_from_slice(&reverse_hex(&str_to_hex(HEADER), 8));
    shell_link_header.extend_from_slice(&reverse_hex(&str_to_hex(&LINK_CLSID[0..8]), 8));
    shell_link_header.extend_from_slice(&str_to_hex(&LINK_CLSID[8..]));
    shell_link_header
}

/// First the left four bits are swapped with the right four bits.
/// Then all adjacent pairs are swapped and then all adjacent single bits.
/// This results in a reversed order.
pub fn reverse_byte(byte: u8) -> u8 {
    let mut byte = (byte & 0xF0) >> 4 | (byte & 0x0F) << 4;
    byte = (byte & 0xCC) >> 2 | (byte & 0x33) << 2;
    byte = (byte & 0xAA) >> 1 | (byte & 0x55) << 1;
    byte
}
// pub fn reverse_byte(byte: u8) -> u8 {
//     let mut reversed_byte = 0;
//     for i in 0..8 {
//         if (byte & (1 << i)) != 0 {
//             reversed_byte |= 1 << (7 - i);
//         }
//     }
//     reversed_byte
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn link_flags_test() {

    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    #[test]
    fn reverse_byte_test() {
        assert_eq!(0b1001_1101u8, reverse_byte(0b1011_1001));
    }
}
