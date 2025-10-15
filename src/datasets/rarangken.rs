pub static RARANGKEN_INFIX:[(char, char); 3] = [
    ('\u{1BA1}', 'y'),
    ('\u{1BA2}', 'r'),
    ('\u{1BA3}', 'l')
];
pub static RARANGKEN_SUFFIX:[(char, &'static str); 3] = [
    ('\u{1B80}', "ng"),
    ('\u{1B81}', "r"),
    ('\u{1B82}', "h")
];
pub static RARANGKEN_REPLACER:[(char, &'static str); 7] = [
    ('\u{1BAA}', "-"),
    ('\u{1BA4}', "i"),
    ('\u{1BA8}', "e"),
    ('\u{1BA9}', "eu"),
    ('\u{1BA5}', "u"),
    ('\u{1BA6}', "é"),
    ('\u{1BA7}', "o")
];