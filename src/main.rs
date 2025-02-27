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
        'F',
        '\x65',
        'e',
        '\u{6C}',
        '\u{069}',
        'n',
        '\'',
        '\u{9}',
        '"',
        '\u{00072}',
        'u',
        '\u{0073}',
        '\u{00_00_74}',
        'y',
        '\"', //" TODO remove
        '\t',
        '\\',
        'o',
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

    ch.iter_mut().for_each(|c| if c.is_whitespace() {
        *c = ' ';
    });

    String::from_iter(ch)
}
