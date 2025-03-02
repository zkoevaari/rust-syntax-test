/*
    Created by Zoltan Kovari <zkoevaari>, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

fn main() {
    let mut chars = char_literals();
    tab_to_space(&mut chars);
    print!("{chars}");

    let _raw1 = raw_strings_1();

    let string = prettify(strings());
    println!("{string}");

    let _raw2 = raw_strings_2();

    println!("{}", integers());
}

fn char_literals() -> String {
    let v = vec![
        ('F',               b'F',       70),
        ('\x65',            b'\x65',    0b0110_0101),
        ('e',               b'e',       0o145),
        ('\u{6C}',          b'\x6C',    0x6C),
        ('\u{069}',         b'\x69',    0x_69),
        ('n',               b'n',       0x6E_),
        ('\'',              b'\'',      0x0027),
        ('\u{9}',           b'\x09',    0x9),
        ('"',               b'"',       034),
        ('\u{00072}',       b'\x72',    0x72),
        ('u',               b'u',       117),
        ('\u{0073}',        b'\x73',    0x73),
        ('\u{00_00_74}',    b'\x74',    0x74),
        ('y',               b'y',       121u8),
        ('\"',              b'\"',      0b10_0010u8), //" TODO remove
        ('\t',              b'\t',      0o11u8),
        ('\\',              b'\\',      0x5Cu8),
        ('o',               b'o',       0x6fu8),
        ('/',               b'/',       0x2f_u8),
        ('\r',              b'\r',      0b1101_u8),
        ('\n',              b'\n',      10_u8),
        ('\0',              b'\0',      0),

        // These are invalid
//~         ('',            b'',            0b_),
//~         ('\',           b'\',           0x__),
//~         (''',           b''',           0B1001)
//~         ('	',          b'	',          0O120),     // Tab
//~         ('\c',          b'\c',          0X63),
//~         ('\x',          b'\x',          6a3),
//~         ('\x9',         b'\x9',         0b2),
//~         ('\x80',        b'\xGG',        0o8),
//~         ('\x7_F',       b'\x063',       0xG),
//~         ('\x063',       b'\u',          00x63),
//~         ('\u',          b'\u0063'       63u),
//~         ('\u0063',      b'\u{}',        63u0),
//~         ('\u{}',        b'\u{G}',       63u08),
//~         ('\u{G}',       b'\u{0063}',    63u_8),
//~         ('\u{+0063}',   b'\u{+0063}',   63u8_),
//~         ('\u{1234567}', b'\u{1234567}', 63u12),
    ];

    for (c, b, u) in &v {
        assert_eq!(*b, *u);
        assert_eq!(*c, char::from(*b));
    }

    let nts = String::from_iter(v.iter().map(|(c, _, _)| c));
    substr_until_nul(&nts).to_string()
}

fn raw_strings_1() -> String {
    let raw_s =  r#"F\x65e\u{6C}\u{069}n\'\u{9}"\u{00072}u\u{0073}\u{00_00_74}y\"\t\\#/\r\n\0"#;
    let raw_b = br#"F\x65e\u{6C}\u{069}n\'\u{9}"\u{00072}u\u{0073}\u{00_00_74}y\"\t\\#/\r\n\0"#;
    let raw_c = cr#"F\x65e\u{6C}\u{069}n\'\u{9}"\u{00072}u\u{0073}\u{00_00_74}y\"\t\\#/\r\n\0"#;

    assert_eq!(raw_s, std::str::from_utf8(raw_b).unwrap());
    assert_eq!(raw_s, raw_c.to_string_lossy());

    // These are invalid
//~     let i1 = r""#;
//~     let i2 = r#"#"##;
//~     let i3 = r##"##"###;
//~     let ib1 = br""#;
//~     let ib2 = br#"#"##;
//~     let ib3 = br##"##"###;
//~     let ic1 = cr""#;
//~     let ic2 = cr#"#"##;
//~     let ic3 = cr##"##"###;

    // These should not terminate
//~     let u1 = r""";
//~     let u2 = r#"#";
//~     let u3 = r##"##;
//~     let u4 = r##"";
//~     let u5 = r##""#;
//~     let u6 = r##"##"#;
//~     let ub1 = br""";
//~     let ub2 = br#"#";
//~     let ub3 = br##"##;
//~     let ub4 = br##"";
//~     let ub5 = br##""#;
//~     let ub6 = br##"##"#;
//~     let uc1 = cr""";
//~     let uc2 = cr#"#";
//~     let uc3 = cr##"##;
//~     let uc4 = cr##"";
//~     let uc5 = cr##""#;
//~     let uc6 = cr##"##"#;

    raw_s.to_string()
}

fn strings() -> &'static str {
    let normal_str = "\u{9}|  \x7c T\"\' |   |    /\
    '\\  A\r\n\t\x7C--| |\u{2d}  |   |   \u{28}   ) V
    \u{07C}  | \u{007C}__ |\u{0005F}_ |_\u{00_00_5F}  \\_/  #\0"; //" TODO remove

    let byte_str = b"\x09|  \x7c T\"\' |   |    /\
    '\\  A\r\n\t\x7C--| |\x2D  |   |   \x28   ) V
    \x7C  | \x7C__ |\x5F_ |_\x5F  \\_/  #\0";

    let c_str = c"\u{9}|  \x7c T\"\' |   |    /\
    '\\  A\r\n\t\x7C--| |\u{2d}  |   |   \u{28}   ) V
    \u{07C}  | \u{007C}__ |\u{0005F}_ |_\u{00_00_5F}  \\_/  #"; //" TODO remove

    assert_eq!(normal_str, std::str::from_utf8(byte_str).unwrap());
    assert_eq!(c_str, std::ffi::CStr::from_bytes_with_nul(byte_str).unwrap());

    let without_nul = substr_until_nul(normal_str);
    assert_eq!(without_nul, c_str.to_string_lossy());

    // These are invalid
//~     let i = "\c \x \x9 \x80 \x7_F \u \u0063 \u{} \u{G} \u{+0063} \u{1234567}";
//~     let ib = b"\c \x \x9 \xGG \u \u0063 \u{} \u{G} \u{0063} \u{+0063} \u{1234567}";
//~     let ic = c"\c \x \x9 \xGG \x7_F \u \u0063 \u{} \u{G} \u{+0063} \u{1234567}";
//~     let ic0 = c"\0 \x0 \x00 \u{0} \u{00} \u{000} \u{0000} \
//~         \u{0_0000} \u{00_0000} \u{000_0000}";

    let _multi_line_with_comment = "First line.
    // This is not a comment!
    Third line.";

    without_nul
}

fn raw_strings_2() -> String {
    let raw_str = r#"\u{9}|  \x7c T\"\' |   |    /\
    '\\  A\r\n\t\x7C--| |\u{2d}  |   |   \u{28}   ) V
    \u{07C}  | \u{007C}__ |\u{0005F}_ |_\u{00_00_5F}  \\_/  #\0"#;

//~     unescape(raw_str)
    raw_str.to_string()
}

macro_rules! string_try_chars {
    ($($e:expr),+ $(,)?) => {
        {
            let mut s = String::new();
            $(
                s.push(char::from(u8::try_from($e).unwrap_or(b'?')));
            )*
            s
        }
    };
}
fn integers() -> String {
    let msg = string_try_chars![
        72u8 as u8,
        101u16 as u16,
        108u32 as u32,
        108u64 as u64,
        111u128 as u128,
        32usize as usize,
        119i8 as i8,
        111i16 as i16,
        114i32 as i32,
        108i64 as i64,
        100i128 as i128,
        33isize as isize,
    ];
    format!("{msg}")
}

//
// Helper functions
//

fn substr_until_nul(slice: &str) -> &str {
    match slice.chars().position(|c| c == '\0') {
        Some(i) => &slice[0..i],
        None => slice,
    }
}

fn remove_tabs(s: &str) -> String {
    s.replace('\t', "").replace("\n    ", "\n")
}

fn swap_ascii(s: &mut str, p: u8, r: u8) {
    assert!(p <= 0x7F);
    assert!(r <= 0x7F);
    unsafe {
        for c in s.as_bytes_mut() {
            if *c == p {
                *c = r;
            }
        }
    }
}

fn tab_to_space(s: &mut str) {
    swap_ascii(s, b'\t', b' ');
}

fn prettify(s: &str) -> String {
    let mut n = remove_tabs(s)
        .replace('A', "\u{039b}");
    swap_ascii(&mut n, b'#', b'o');
    n
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substr_until_nul() {
        let ref1 = "Hello world!";
        let art1 = ref1.clone();
        assert_eq!(substr_until_nul(art1), ref1);

        let art2 = "Hello world!\0";
        assert_eq!(substr_until_nul(art2), ref1);

        let art3 = "Hello\0 world!";
        let ref3 = "Hello";
        assert_eq!(substr_until_nul(art3), ref3);

        let art4 = "Hello\0 world!\0";
        assert_eq!(substr_until_nul(art4), ref3);
        assert_eq!(
            substr_until_nul(art4),
            std::ffi::CStr::from_bytes_until_nul(art4.as_bytes()).unwrap().to_str().unwrap()
        );
    }
}
