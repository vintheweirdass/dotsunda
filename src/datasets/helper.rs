use crate::datasets::{CONSONANT, NUMBER, RARANGKEN_INFIX, RARANGKEN_REPLACER, RARANGKEN_SUFFIX, VOCAL};
#[cfg(feature="ux_numbering")]
use crate::datasets::NUMBER_WITH_U5_EQUIV;
#[cfg(feature="ux_numbering")]
use ux::u5;
#[cfg(not(feature = "ux_numbering"))]
use crate::datasets::NUMBER_WITH_U8_EQUIV;
pub fn get_number(num: impl Into<usize>) -> Option<char> {
    #[cfg(feature="ux_numbering")]
    {
        let u5_num: u5 = match u5::try_from(num.into()) {
            Ok(val) => val,
            Err(_) => {
                return None;
            }
        };
        for (e, k) in NUMBER_WITH_U5_EQUIV.iter() {
            if *k == u5_num {
                return Some(*e);
            }
        }
    }
    #[cfg(not(feature="ux_numbering"))]
    {
        let u8_num: u8 = match u8::try_from(num.into()) {
            Ok(val) => val,
            Err(_) => {
                return None;
            }
        };
        for (e, k) in NUMBER_WITH_U8_EQUIV.iter() {
            if *k == u8_num {
                return Some(*e);
            }
        }
    }
    None
}
pub fn get_number_from_char(ch: char) -> Option<char> {
    let ch_lower = match ch.to_lowercase().next() {
        Some(v) => v,
        None => {
            return None
        },
    };
    for (e, k) in NUMBER.iter() {
        if *k == ch_lower {
            return Some(*e);
        }
    }
    return None;
}
pub fn get_consonant_from<'a>(c: &'a str) -> Option<char> {
    let ch_lower = c.to_lowercase();
    let ch_lower_str = ch_lower.as_str();
    for (e, k) in CONSONANT.iter() {
        if *k == ch_lower_str {
            return Some(*e);
        }
    }
    return None;
}
pub fn get_vocal<'a>(v: &'a str) -> Option<char> {
    let ch_lower = v.to_lowercase();
    let ch_lower_str = ch_lower.as_str();
    for (e, k) in VOCAL.iter() {
        if *k == ch_lower_str {
            return Some(*e);
        }
    }
    return None;
}
pub fn get_rarangken_replacer_equiv<'a>(v: &'a str) -> Option<char> {
    let ch_lower = v.to_lowercase();
    let ch_lower_str = ch_lower.as_str();
    for (e, k) in RARANGKEN_REPLACER.iter() {
        if *k == ch_lower_str {
            return Some(*e);
        }
    }
    return None;
}
pub fn get_rarangken_suffix_equiv<'a>(v: &'a str) -> Option<char> {
    let ch_lower = v.to_lowercase();
    let ch_lower_str = ch_lower.as_str();
    for (e, k) in RARANGKEN_SUFFIX.iter() {
        if *k == ch_lower_str {
            return Some(*e);
        }
    }
    return None;
}
pub fn get_rarangken_infix_equiv(v: char) -> Option<char> {
    let ch_lower = match v.to_lowercase().next() {
        Some(v) => v,
        None => {
            return None
        },
    };
    for (e, k) in RARANGKEN_INFIX.iter() {
        if *k == ch_lower {
            return Some(*e);
        }
    }
    return None;
}


