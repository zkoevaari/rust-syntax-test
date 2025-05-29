/*
    Created by Zoltan Kovari <zkoevaari>, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

/*!
    Goal of this source file is to exercise most language features, so it can be used in the
    development of syntax highlighters.

    It is intentionally silly.
*/

use std::collections::VecDeque;
use std::io::Write;
use std::time::SystemTime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chars = char_literals();
    let mut chars2 = substr_until_nul(&chars).to_string();
    tab_to_space(&mut chars2);
    print!("{chars2}");

    let raws = raw_strings();
    assert_eq!(unescape(&raws), chars);

    let string = strings();
    println!("{}", prettify(string));

    println!("{}", integers());
    println!("{}", floating_points());
    println!("{}", ranges());

    misc_keywords(&mut std::io::stdout());

    Ok(())
}

///
/// Literals
///

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
        ('\"',              b'\"',      0b10_0010u8), //" Shouldn't need this doublequote
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

    String::from_iter(v.iter().map(|(c, _, _)| c))
}

fn raw_strings() -> String {
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

    raw_s.replace('#', "o")
}

fn strings() -> &'static str {
    let normal_str = "\u{9}|  \x7c T\"\' |   |    /\
    '\\  A\r\n\t\x7C--| |\u{2d}  |   |   \u{28}   ) V
    \u{07C}  | \u{007C}__ |\u{0005F}_ |_\u{00_00_5F}  \\_/  #\0"; //" Shouldn't need this doublequote

    let byte_str = b"\x09|  \x7c T\"\' |   |    /\
    '\\  A\r\n\t\x7C--| |\x2D  |   |   \x28   ) V
    \x7C  | \x7C__ |\x5F_ |_\x5F  \\_/  #\0";

    let c_str = c"\u{9}|  \x7c T\"\' |   |    /\
    '\\  A\r\n\t\x7C--| |\u{2d}  |   |   \u{28}   ) V
    \u{07C}  | \u{007C}__ |\u{0005F}_ |_\u{00_00_5F}  \\_/  #"; //" Shouldn't need this doublequote

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

    // /*
    let multi = "Line #1
    // This is not a comment! \\
    Line #3";
    let raw_multi = r#"Line #1
    // This is not a comment! \\
    Line #3"#;
    assert_ne!(raw_multi, multi);
    assert_eq!(unescape(raw_multi), multi);
    // */

    without_nul
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
    string_try_chars![
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
    ]
}

fn floating_points() -> String {
//~     let a = (97. as f16).into(); // TODO: use these when f16 and f128 stabilize
    let a = (97. as f32).into();
    let e = 101f16.into();
    let i = (105.0 as f32).into();
    let o = 111.0f32.into();
//~     let u = f128::round(117_.0) as f64;
    let u = f64::round(117_.0);
    let sp = 32_f128 as f64;

    let floats = [
        34.0_,      69.0_f64,   109e0,
        112E0,      o,          119_e0,
        e,          114_E0,     i,
        110e_0,     103E_0,     sp,
        e,          118e0_,     e,
        114E0_,     121e_0_,    o,
        110E_0_,    e,          sp,
        116_e_0_,   o,          sp,
        98_E_0_,    u,          i,
        108e0f64,   100E0f64,   sp,
        114e0_f64,  e,          108E0_f64,
        i,          a,          98e+0,
        108E+0,     e,          sp,
        a,          110e-0,     100E-0,
        sp,         e,          102e+0f64,
        102E+0f64,  i,          99e-0f64,
        i,          e,          110E-0f64,
        116e+_0,    sp,         115E+_0,
        o,          102e-_0,    116E-_0,
        119e+0_f64, a,          114E+0_f64,
        e,          46e-0_f64,  34E-0_f64,

        // These are invalid
//~         _1.0,       1._,        1._0,
//~         1.f64,      1.0f12,     1.0f64,
//~         1.0u64,     1._f64,     1._0f64,
//~         1.0f_64,    1.0f64_,    0x1.0,
//~         1.0x0,      1e,         1E,
//~         1ef64,      1Ef64,      1e0f12,
//~         1E0f12,     1e+,        1E+,
//~         1e-,        1E-,        1e+f64,
//~         1E+f64,     1e-f64,     1E-f64,
//~         1e_+0,      1E_+0,      1e_-0,
//~         1E_-0,      .1,         .1f64,
    ];

    String::from_utf8(floats.iter().map(|f| *f as u8).collect()).unwrap()
}

fn ranges() -> String {
    let (mut h, m) = get_time();

    let prefix = match m {
        ..4 => "It is exactly",
        57.. => {
            h += 1;
            "It is almost"
        }
        ..=18 => "It is a little past",
        42u8..57 => {
            h += 1;
            "It is approaching"
        }
        27_..=33 => {
            h += 1;
            "It is halfway to"
        }

        // These are invalid
//~         1..= | 1... | 1...2 => "",

        _ => "It is about half past",
    };

    let tod = match h as f32 {
        22.0.. | ..3. => "at night",
        ..=11f32 => "in the morning",
        12e0..13.0 => "at midday",
        19.0_..=2_1_f32 => "in the evening",
        _ => "in the afternoon",
    };

    let h12 = match h {
        13.. => h - 12,
        _ => h,
    };

    // /*
    fn floor() -> i32 { 42 }
    let _range = 23..floor(); // Note: this is a range and not a function call on a float
    // */

    format!("{prefix} {h12} o'clock universal, {tod}.")
}

//
// Misc. keywords
//

trait Song {
    fn sing(&mut self, out: &mut impl Write) -> bool;
}
struct NinetyNineBoB {
    bottles: i8,
}
impl NinetyNineBoB {
    pub(crate) const QTY: i8 = 99;

    fn new() -> Self {
        Self {
            bottles: Self::QTY,
        }
    }

    fn chorus(&self) -> String {
        format!(
            "{} bottle{} of beer",
            match self.bottles {
                0 => String::from("No more"),
                _ => self.bottles.to_string(),
            },
            match self.bottles {
                1 => "",
                _ => "s",
            }
        )
    }
}
impl Song for NinetyNineBoB {
    fn sing(&mut self, out: &mut impl Write) -> bool {
        writeln!(out, "\n{} on the wall, {},", self.chorus(), self.chorus()).unwrap();

        self.bottles -= 1;
        let ret = match self.bottles {
            1.. => false,
            _ => raw_true(),
        };

        writeln!(
            out,
            "{}, {} on the wall.",
            match self.bottles {
                0.. => "Take one down and pass it around",
                _ => {
                    self.bottles = Self::QTY;
                    "Go to the store and buy some more"
                }
            },
            self.chorus(),
        ).unwrap();

        ret
    }
}

type IoResult<T> = std::io::Result<T>;

struct EllipsizedSinger<'a, T>
where T: Write
{
    buf: VecDeque<String>,
    writer: &'a mut T,
}
impl<'a, T: Write> EllipsizedSinger<'a, T> {
    fn new(writer: &'a mut T) -> Self {
        Self {
            buf: VecDeque::new(),
            writer,
        }
    }
}
impl<T: Write> Write for EllipsizedSinger<'_, T> {
    fn write(&mut self, b: &[u8]) -> IoResult<usize> {
        let chunk = String::from(str::from_utf8(b).map_err(std::io::Error::other)?);
        if self.buf.is_empty() || self.buf.back().unwrap().ends_with('\n') {
            self.buf.push_back(chunk);
        } else {
            self.buf.back_mut().unwrap().push_str(&chunk);
        }
        Ok(b.len())
    }

    fn flush(&mut self) -> IoResult<()> {
        if self.buf.len() <= 17 {
            for s in self.buf.iter() {
                write!(self.writer, "{s}")?;
            }
        } else {
            for s in self.buf.iter().take(7) {
                write!(self.writer, "{s}")?;
            }
            writeln!(self.writer, "{}", ellipsis())?;
            for s in self.buf.iter().rev().take(9).rev() {
                write!(self.writer, "{s}")?;
            }
        }
        self.buf.clear();
        Ok(())
    }
}

fn ellipsis() -> &'static str {
    let pin = std::pin::pin!(prepare_mark(async || "[...]"));
    let mut ctx = std::task::Context::from_waker(std::task::Waker::noop());
    if let std::task::Poll::Ready(v) = Future::poll(pin, &mut ctx) {
        v
    } else {
        panic!("unexpected Poll::Pending");
    }
}

async fn prepare_mark<'a>(f: impl AsyncFn() -> &'a str) -> &'a str {
    f().await
}

fn misc_keywords(mut out: impl Write) {
    writeln!(out, "\nLet's sing a song!").unwrap();
    let mut singer_value = EllipsizedSinger::new(&mut out);
    let ref mut singer = singer_value;
    let mut song = NinetyNineBoB::new();
    let mut n: u8 = 0;
    'outer: loop {
        let mut m: u8 = 0;
        while !song.sing(singer) {
            m += 1;
            if m >= 2 && n >= 1 {
                break 'outer;
            }
        }
        n += 1;
        singer.flush().unwrap();
        continue;
    }
    singer.flush().unwrap();
    writeln!(out, "\n{}", ellipsis()).unwrap();
}

//
// Helper functions
//

fn substr_until_nul(slice: &str) -> &str {
    match slice.chars().position(move |c| c == '\0') {
        Some(i) => &slice[0..i],
        None => slice,
    }
}

fn remove_tabs(s: &str) -> String {
    s.replace('\t', "").replace("\n    ", "\n")
}

fn replace_ascii(s: &mut str, p: u8, r: u8) {
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
    replace_ascii(s, b'\t', b' ');
}

fn prettify(s: &str) -> String {
    let mut n = remove_tabs(s).replace('A', "\u{039b}");
    replace_ascii(&mut n, b'#', b'o');
    n
}

fn unescape(s: &str) -> String {
    enum State {
        None,
        Escape,
        Ascii(Option<u8>),
        UnicodeMaybe,
        Unicode(Vec<u8>),
    }

    let mut out = String::new();
    let mut buf = String::new();
    let mut state = State::None;

    for c in s.chars() {
        state = match state {
            State::None => match c {
                '\\' => {
                    buf.push(c);
                    State::Escape
                }
                _ => {
                    out.push(c);
                    State::None
                }
            },
            State::Escape => match c {
                'x' => {
                    buf.push(c);
                    State::Ascii(None)
                }
                'u' => {
                    buf.push(c);
                    State::UnicodeMaybe
                }
                'n' => {
                    out.push('\n');
                    buf.clear();
                    State::None
                }
                'r' => {
                    out.push('\r');
                    buf.clear();
                    State::None
                }
                't' => {
                    out.push('\t');
                    buf.clear();
                    State::None
                }
                '0' => {
                    out.push('\0');
                    buf.clear();
                    State::None
                }
                '\'' | '"' | '\\' => {
                    out.push(c);
                    buf.clear();
                    State::None
                }
                _ => {
                    buf.push(c);
                    out.push_str(&buf);
                    buf.clear();
                    State::None
                }
            },
            State::Ascii(None) => match c {
                _ if c.is_ascii_digit() && c != '8' && c != '9' => {
                    buf.push(c);
                    State::Ascii(Some(c.to_digit(16).unwrap().try_into().unwrap()))
                }
                _ => {
                    buf.push(c);
                    out.push_str(&buf);
                    buf.clear();
                    State::None
                }
            },
            State::Ascii(Some(u)) => match c {
                _ if c.is_ascii_hexdigit() => {
                    out.push(
                        char::from_u32(((u as u32) << 4) + c.to_digit(16).unwrap()).unwrap()
                    );
                    buf.clear();
                    State::None
                }
                _ => {
                    buf.push(c);
                    out.push_str(&buf);
                    buf.clear();
                    State::None
                }
            },
            State::UnicodeMaybe => {
                buf.push(c);
                match c {
                    '{' => State::Unicode(Vec::new()),
                    _ => {
                        out.push_str(&buf);
                        buf.clear();
                        State::None
                    }
                }
            }
            State::Unicode(mut v) => {
                buf.push(c);
                match c {
                    '}' => {
                        if v.is_empty() || v.len() > 6 {
                            out.push_str(&buf);
                        } else {
                            match parse_unicode(&v) {
                                Some(p) => out.push(p),
                                _ => out.push_str(&buf),
                            }
                        }
                        buf.clear();
                        State::None
                    }
                    '_' => State::Unicode(v),
                    _ if c.is_ascii_hexdigit() => {
                        v.push(c.to_digit(16).unwrap().try_into().unwrap());
                        State::Unicode(v)
                    }
                    _ => {
                        out.push_str(&buf);
                        buf.clear();
                        State::None
                    }
                }
            }
        };
    }

    if !matches!(state, State::None) {
        out.push_str(&buf);
    }

    out
}
fn parse_unicode(digits: &[u8]) -> Option<char> {
    if digits.is_empty() || digits.len() > 8 {
        return None;
    }

    let mut acc: u32 = 0;
    for (i, u) in digits.iter().rev().enumerate() {
        acc += (*u as u32) << (i * 4);
    }

    char::from_u32(acc)
}

fn get_time() -> (u8, u8) {
    let epoch_secs = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let day_secs = epoch_secs % (24 * 60 * 60);
    let hour = day_secs / (60 * 60);
    let min = (day_secs - (hour * 60 * 60)) / 60;
    (hour.try_into().unwrap(), min.try_into().unwrap())
}

extern "Rust" fn raw_true() -> bool {
    union MyBool {
        b: bool,
        _u: u8,
    }
    let mut maybe = std::mem::MaybeUninit::<MyBool>::uninit();
    unsafe {
        let p = &raw mut (*maybe.as_mut_ptr()).b;
        p.write(true);
    }
    // I know this is cheating, but it would be a pain to bring in extern functions
    // just to demo the `safe` keyword.
    // At least this way we can see how a _weak keyword_ works.
    let safe = unsafe { maybe.assume_init().b };
    safe
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
        let art1 = ref1;
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
            std::ffi::CStr::from_bytes_until_nul(art4.as_bytes())
                .unwrap()
                .to_str()
                .unwrap()
        );
    }

    #[test]
    fn test_parse_unicode() {
        let tests = [
            ('\t', vec![9]),
            ('Z', vec![5, 0xA]),
            ('\u{1b5}', vec![1, 0xB, 5]),
            ('\u{2124}', vec![2, 1, 2, 4]),
            ('\u{1FBC5}', vec![1, 0xF, 0xB, 0xC, 5]),
        ];

        for (r, a) in tests {
            assert_eq!(parse_unicode(&a), Some(r));
        }
    }
}

// Let's put comment highlighting to the test

/// This is a doc comment...
//// ...but this is not

/* Similarly, */
/** this is a doc comment... */
/*** ...but this is not */

mod inner {
    //! Remember: bang marks inner, i.e. applies to the parent
    //!! Makes no difference...
    /*! ...even if there are... */
    /*!! ...more than one exclamation marks */
}

mod nest {
    /* Can we /* nest */ block comments? */
    /*   /* */  /*! */  /** */  */
    /*!  /* */  /*! */  /** */  */
    /**  /* */  /*! */  /** */  */

    /*
    /// How about...
    */
    /**
    // ...mixing them?
    */

    mod dummy1 {}
}

mod empty {
    //!
    /*!*/
    //
    ///
    ////
    /**/
    /***/

    mod dummy2 {}
}
