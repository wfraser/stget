use std::io;
use base32;

#[cfg(unix)]
use libc;

#[allow(clippy::many_single_char_names)]
pub fn device_id_from_hash(hash: &[u8]) -> String {
    let s = base32::encode(
        base32::Alphabet::RFC4648 { padding: false },
        hash);

    let a = syncthing_luhn(&s[0..13]);
    let b = syncthing_luhn(&s[13..26]);
    let c = syncthing_luhn(&s[26..39]);
    let d = syncthing_luhn(&s[39..52]);

    format!("{}-{}{}-{}-{}{}-{}-{}{}-{}-{}{}",
            &s[ 0.. 7], &s[ 7..13], a,
            &s[13..20], &s[20..26], b,
            &s[26..33], &s[33..39], c,
            &s[39..46], &s[46..52], d)
}

#[test]
fn test_device_id() {
    let hash = vec![
        0x48, 0xcb, 0xde, 0xc7, 0xb0, 0x82, 0x43, 0x7a,
        0x42, 0x0f, 0x95, 0x4b, 0x33, 0x94, 0x40, 0xaf,
        0xbe, 0xd9, 0x55, 0x9f, 0x46, 0x94, 0x10, 0x7d,
        0xfc, 0x61, 0x9c, 0x04, 0x44, 0xa0, 0xda, 0x38];

    assert_eq!("JDF55R5-QQJBXUN-QQPSVFT-HFCAV6J-7NSVM7I-2KBA7PI-4MGOAIR-FA3I4AH",
               device_id_from_hash(&hash));
}

pub fn hash_from_device_id(device_id: &str) -> Vec<u8> {
    // xxxxxxx-xxxxxxA-xxxxxxx-xxxxxxB-xxxxxxx-xxxxxxC-xxxxxxx-xxxxxxD
    // 000000000011111111112222222222333333333344444444445555555555666
    // 012345678901234567890123456789012345678901234567890123456789012
    // JDF55R5-QQJBXUN-QQPSVFT-HFCAV6J-7NSVM7I-2KBA7PI-4MGOAIR-FA3I4AH

    let mut base32_input = String::new();
    base32_input.push_str(&device_id[0..7]);
    base32_input.push_str(&device_id[8..14]);
    base32_input.push_str(&device_id[16..23]);
    base32_input.push_str(&device_id[24..30]);
    base32_input.push_str(&device_id[32..39]);
    base32_input.push_str(&device_id[40..46]);
    base32_input.push_str(&device_id[48..55]);
    base32_input.push_str(&device_id[56..62]);
    base32::decode(base32::Alphabet::RFC4648 { padding: false }, &base32_input).unwrap()
}

#[test]
fn test_reverse_device_id() {
    let hash = vec![
        0x48, 0xcb, 0xde, 0xc7, 0xb0, 0x82, 0x43, 0x7a,
        0x42, 0x0f, 0x95, 0x4b, 0x33, 0x94, 0x40, 0xaf,
        0xbe, 0xd9, 0x55, 0x9f, 0x46, 0x94, 0x10, 0x7d,
        0xfc, 0x61, 0x9c, 0x04, 0x44, 0xa0, 0xda, 0x38];

    assert_eq!(&hash, &hash_from_device_id("JDF55R5-QQJBXUN-QQPSVFT-HFCAV6J-7NSVM7I-2KBA7PI-4MGOAIR-FA3I4AH"));
}

/// This is similar to Luhn mod 32, except with some bugs that are in the Syncthing implementation:
/// the initial factor is 1 instead of 2, and it reads the input forwards instead of in reverse.
fn syncthing_luhn(group: &str) -> char {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

    let mut factor = 1;
    let mut sum = 0;
    let n = ALPHABET.len();

    for c in group.chars() {
        let codepoint = ALPHABET.iter().position(|x| *x as char == c).unwrap();
        let addend = factor * codepoint;
        sum += addend / n + addend % n;
        factor = if factor == 2 { 1 } else { 2 };
    }

    let remainder = sum % n;
    let check_codepoint = (n - remainder) % n;
    ALPHABET[check_codepoint] as char
}

#[cfg(unix)]
pub fn get_hostname() -> io::Result<String> {
    extern "C" {
        fn gethostname(name: *mut u8, size: libc::size_t) -> libc::c_int;
    }

    let mut buf = [0u8; 256];
    let result = unsafe { gethostname(buf.as_mut_ptr(), 255) };
    if -1 == result {
        return Err(io::Error::last_os_error());
    }

    let len = buf.iter().position(|c| *c == 0).unwrap_or(255);
    Ok(String::from_utf8_lossy(&buf[0 .. len]).into_owned())
}

#[cfg(windows)]
pub fn get_hostname() -> io::Result<String> {
    extern "C" {
        fn GetComputerNameW(lpBuffer: *mut u16, lpnSize: *mut u32) -> i32;
    }

    let mut buf = [0u16; 16]; // 16 == MAX_COMPUTERNAME_LENGTH + 1
    let mut len = 16u32;
    let result = unsafe { GetComputerNameW(buf.as_mut_ptr(), &mut len) };
    if 0 == result {
        return Err(io::Error::last_os_error());
    }

    Ok(String::from_utf16_lossy(&buf[0 .. len as usize]))
}
