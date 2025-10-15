use core::u8;

#[cfg(feature = "ux_numbering")]
use ux::u5;
pub static NUMBER: [(char, char); 10] = [
    ('\u{1BB0}', '0'),
    ('\u{1BB1}', '1'),
    ('\u{1BB2}', '2'),
    ('\u{1BB3}', '3'),
    ('\u{1BB4}', '4'),
    ('\u{1BB5}', '5'),
    ('\u{1BB6}', '6'),
    ('\u{1BB7}', '7'),
    ('\u{1BB8}', '8'),
    ('\u{1BB9}', '9')
];
pub static NUMBER_WITH_U8_EQUIV: [(char, u8); 10] = [
    (NUMBER[0].0, 0),
    (NUMBER[1].0, 1),
    (NUMBER[2].0, 2),
    (NUMBER[3].0, 3),
    (NUMBER[4].0, 4),
    (NUMBER[5].0, 5),
    (NUMBER[6].0, 6),
    (NUMBER[7].0, 7),
    (NUMBER[8].0, 8),
    (NUMBER[9].0, 9)
];
#[cfg(feature = "ux_numbering")]
pub static NUMBER_WITH_U5_EQUIV: [(char, u5); 10] = [
    (NUMBER[0].0, u5::new(0)),
    (NUMBER[1].0, u5::new(1)),
    (NUMBER[2].0, u5::new(2)),
    (NUMBER[3].0, u5::new(3)),
    (NUMBER[4].0, u5::new(4)),
    (NUMBER[5].0, u5::new(5)),
    (NUMBER[6].0, u5::new(6)),
    (NUMBER[7].0, u5::new(7)),
    (NUMBER[8].0, u5::new(8)),
    (NUMBER[9].0, u5::new(9))
];