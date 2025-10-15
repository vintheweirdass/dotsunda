#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::string::String;
use pest::{Parser, iterators::Pairs};
#[cfg(feature = "std")]
use std::string::String;
mod grammar;
use grammar::DotSundaParser;
pub use grammar::Rule;
pub mod datasets;
use datasets::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ParseError {
    PestRuleError(pest::error::Error<Rule>),
    IterateToNextPair,
}
impl From<pest::error::Error<Rule>> for ParseError {
    fn from(err: pest::error::Error<Rule>) -> Self {
        ParseError::PestRuleError(err)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParseOptionBuilder {
    /// Default: (`true`)
    keep_number_pipes: bool,
}
impl Default for ParseOptionBuilder {
    fn default() -> Self {
        return Self {
            keep_number_pipes: true,
        };
    }
}
impl ParseOptionBuilder {
    pub fn new() -> Self {
        return Self::default();
    }
    /// Default: (`true`)
    pub fn keep_number_pipes(mut self, value: bool) -> Self {
        self.keep_number_pipes = value;
        self
    }
}

pub fn parse_to_pest_pairs<'a>(input: &'a str) -> Result<Pairs<'a, Rule>, ParseError> {
    let mut pairs = DotSundaParser::parse(Rule::content, input)?;
    Ok(pairs
        .next()
        .ok_or(ParseError::IterateToNextPair)?
        .into_inner())
}

pub fn parse<'a>(input: &'a str, opt: &'a ParseOptionBuilder) -> Result<String, ParseError> {
    let pairs = parse_to_pest_pairs(input)?;
    let mut result = String::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::SundaContent => {
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::SundaNumbers => {
                            if opt.keep_number_pipes {
                                result.push('|');
                            }
                            for ch in inner.as_str().chars() {
                                if let Some(conv) = get_number_from_char(ch) {
                                    result.push(conv);
                                }
                            }
                            if opt.keep_number_pipes {
                                result.push('|');
                            }
                        }
                        Rule::SundaWord => {
                            for syll_pair in inner.into_inner() {
                                process_syllable(syll_pair, &mut result);
                            }
                        }
                        Rule::VisiblePlainContent => {
                            for content_part in inner.into_inner() {
                                match content_part.as_rule() {
                                    Rule::PlainChars => {
                                        result.push_str(content_part.as_str());
                                    }
                                    Rule::EscapedChar => {
                                        let s = content_part.as_str();
                                        match s {
                                            "'" | "\\" => {
                                                result.push_str(s);
                                            }
                                            _ => {
                                                result.push('\\');
                                                result.push_str(s);
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }

            Rule::OuterContent => {
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::OuterChars => {
                            result.push_str(inner.as_str());
                        }
                        Rule::EscapedChar => {
                            let s = inner.as_str();
                            match s {
                                "'" | "\\" => {
                                    result.push_str(s);
                                }
                                _ => {
                                    result.push('\\');
                                    result.push_str(s);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }

            _ => {}
        }
    }

    Ok(result)
}

fn process_syllable(syll_pair: pest::iterators::Pair<Rule>, result: &mut String) {
    let mut base_char: Option<char> = None;
    let mut combining: Option<char> = None;

    match syll_pair.as_rule() {
        Rule::NgalagenaWithRarangkenReplacer => {
            // Consonant ~ RarangkenReplacer
            for part in syll_pair.into_inner() {
                match part.as_rule() {
                    Rule::Consonant => {
                        base_char = get_consonant_from(part.as_str());
                    }
                    Rule::RarangkenReplacer => {
                        combining = get_rarangken_replacer_equiv(part.as_str());
                    }
                    _ => {}
                }
            }
        }
        Rule::NgalagenaWithRarangkenInfix => {
            // Consonant ~ RarangkenInfix ~ Spell_A
            for part in syll_pair.into_inner() {
                match part.as_rule() {
                    Rule::Consonant => {
                        base_char = get_consonant_from(part.as_str());
                    }
                    Rule::RarangkenInfix => {
                        let next = part.as_str().chars().next();
                        match next {
                            Some(ch) => {
                                combining = get_rarangken_infix_equiv(ch);
                            }
                            None => {
                                continue;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Rule::NgalagenaWithRarangkenSuffix => {
            // Consonant ~ Spell_A ~ RarangkenSuffix
            for part in syll_pair.into_inner() {
                match part.as_rule() {
                    Rule::Consonant => {
                        base_char = get_consonant_from(part.as_str());
                    }
                    Rule::RarangkenSuffix => {
                        combining = get_rarangken_suffix_equiv(part.as_str());
                    }
                    _ => {}
                }
            }
        }
        Rule::VocalWithRarangkenSuffix => {
            // Vocal ~ RarangkenSuffix
            for part in syll_pair.into_inner() {
                match part.as_rule() {
                    Rule::Vocal => {
                        let s = part.as_str();
                        let inp = if s == "^e" { "é" } else { s };
                        base_char = get_vocal(inp);
                    }
                    Rule::RarangkenSuffix => {
                        combining = get_rarangken_suffix_equiv(part.as_str());
                    }
                    _ => {}
                }
            }
        }
        Rule::Ngalagena => {
            // Consonant ~ Spell_A
            for part in syll_pair.into_inner() {
                if part.as_rule() == Rule::Consonant {
                    base_char = get_consonant_from(part.as_str());
                }
            }
        }
        Rule::NgalagenaWithRarangkenRemover => {
            for part in syll_pair.into_inner() {
                if part.as_rule() == Rule::Consonant {
                    base_char = get_consonant_from(part.as_str());
                    combining = get_rarangken_replacer_equiv("-")
                }
            }
        }
        Rule::Vocal => {
            let s = syll_pair.as_str();
            let inp = if s == "^e" { "é" } else { s };
            base_char = get_vocal(inp);
        }
        _ => {}
    }

    if let Some(b) = base_char {
        result.push(b);
        if let Some(r) = combining {
            result.push(r);
        }
    }
}

pub fn parse_with_default_opt<'a>(input: &'a str) -> Result<String, ParseError> {
    let opt = &ParseOptionBuilder::default();
    return parse(input, opt);
}
