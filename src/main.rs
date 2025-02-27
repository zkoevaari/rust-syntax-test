/*
    Created by Zoltan Kovari <zkoevaari>, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

fn main() {
//~     println!("{}", char_literals());
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

        // These are all invalid
//~         ('',            b'',            0b_),
//~         ('\',           b'\',           0x__),
//~         (''',           b''',           0B1001)
//~         ('	',          b'	',          0O120),     // Tab
//~         ('\c',          b'\c',          0X63),
//~         ('\x',          b'\x',          6a3),
//~         ('\x9',         b'\x9',         0b2),
//~         ('\x80',        b'\xGG',        0o8),
//~         ('\xFF',        b'\x063',       0xG),
//~         ('\x063',       b'\u',          00x63),
//~         ('\u',          b'\u0063,       63u),
//~         ('\u0063,       b'\u{}',        63u0),
//~         ('\u{}',        b'\u{G}',       63u08),
//~         ('\u{G}',       b'\u{0063}',    63u_8),
//~         ('\u{+0063}',   b'\u{+0063}',   63u8_),
//~         ('\u{1234567}', b'\u{1234567}', 63u12),
    ];

    for (c, b, u) in &v {
        assert_eq!(*b, *u);
        assert_eq!(*c, char::from(*b));
    }

    let c = v.iter().map(|(c, _, _)| match c.is_whitespace() {
        true => ' ',
        false => *c,
    });

    String::from_iter(c)
}

#[derive(Clone, Debug)]
struct Message12 (
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
);
impl std::fmt::Display for Message12 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut msg = String::with_capacity(12);

        msg.push(char::from(u8::try_from(self.0).unwrap()));
        msg.push(char::from(u8::try_from(self.1).unwrap()));
        msg.push(char::from(u8::try_from(self.2).unwrap()));
        msg.push(char::from(u8::try_from(self.3).unwrap()));
        msg.push(char::from(u8::try_from(self.4).unwrap()));
        msg.push(char::from(u8::try_from(self.5).unwrap()));
        msg.push(char::from(u8::try_from(self.6).unwrap()));
        msg.push(char::from(u8::try_from(self.7).unwrap()));
        msg.push(char::from(u8::try_from(self.8).unwrap()));
        msg.push(char::from(u8::try_from(self.9).unwrap()));
        msg.push(char::from(u8::try_from(self.10).unwrap()));
        msg.push(char::from(u8::try_from(self.11).unwrap()));
//~         for i in 0..12 {
//~             msg.push(char::from(u8::try_from(self.i).unwrap()));
//~         }

        writeln!(fmt, "{msg}")
    }
}

fn integers() -> String {
    let msg = Message12(
        72u8,
        101u16,
        108u32,
        108u64,
        111u128,
        32usize,
        119i8,
        111i16,
        114i32,
        108i64,
        100i128,
        33isize,
    );
    format!("{msg}")
}
