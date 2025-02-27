/*
    Created by Zoltan Kovari <zkoevaari>, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

fn main() {
    println!("{}", char_literals());
}

fn char_literals() -> String {
    let mut ch = vec![
        '"',
        'L',
        '\x65',
        '\u{74}',
        '\'',
        '\u{073}',
        '\u{9}',
        '\u{0067}',
        '\u{0006F}',
        '\u{00_00_21}',
        '\t',
        '\\',
        '/',
        '\r',
        '\n',
        '\0',

        // These are invalid
//~         '
//~         ',
//~         '',
//~         '\',
//~         ''',
//~         '	', // Tab
//~         '\c',
//~         '\x',
//~         '\x9',
//~         '\xAA',
//~         '\x80',
//~         '\x063',
//~         '\u',
//~         '\u0063,
//~         '\u{}',
//~         '\u{G}',
//~         '\u{+0063}',
//~         '\u{1234567}',
    ];

    for i in [6, 10] {
        ch[i] = ' ';
    }
    for (i0, i1) in [(8, 12), (0, 14)] {
        ch.insert(i1, ch[i0]);
    }
    let ch = ch.iter().filter(|c| !c.is_control());

    String::from_iter(ch)
}
