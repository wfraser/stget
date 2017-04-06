use base32;

#[allow(many_single_char_names)]
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

/// This is similar to Luhn mod 32, except with some bugs that are in the Syncthing implementation:
/// the initial factor is 1 instead of 2, and it reads the input forwards instead of in reverse.
fn syncthing_luhn(group: &str) -> char {
    const ALPHABET: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

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