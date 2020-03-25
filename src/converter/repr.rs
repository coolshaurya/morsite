use crate::morse::{MorseString, MorseSymbol};
use std::fmt;

#[derive(Debug, Clone)]
pub struct MorseRepr(String);

impl MorseRepr {
    pub fn new(string: String) -> Self {
        MorseRepr(string)
    }
}

impl fmt::Display for MorseRepr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<MorseString> for MorseRepr {
    fn from(morse_string: MorseString) -> Self {
        MorseRepr(
            morse_string
                .iter()
                .map(|morse_symbol| match morse_symbol {
                    MorseSymbol::Dot => ".",
                    MorseSymbol::Dash => "_",
                    MorseSymbol::InterElementSpace => "",
                    MorseSymbol::InterLetterSpace => " ",
                    MorseSymbol::InterWordSpace => " / ",
                })
                .flat_map(|val| val.chars())
                .collect(),
        )
    }
}

impl From<MorseRepr> for MorseString {
    fn from(morse_repr: MorseRepr) -> Self {
        parse_text(&morse_repr.0)
    }
}

fn parse_text(text: &str) -> MorseString {
    let valid_chars = vec!['.', '_', ' ', '/'];
    text.trim()
        .chars()
        .filter(|val| valid_chars.contains(val))
        .collect::<String>()
        .split('/')
        .map(|word| parse_word(word))
        .collect::<Vec<MorseString>>()
        .join(&MorseSymbol::InterWordSpace)
}

fn parse_word(word: &str) -> MorseString {
    word.trim()
        .split(' ')
        .map(|val| parse_letter(val))
        .collect::<Vec<MorseString>>()
        .join(&MorseSymbol::InterLetterSpace)
}

fn parse_letter(letter: &str) -> MorseString {
    letter
        .chars()
        .map(|val| match val {
            '.' => [MorseSymbol::Dot],
            '_' => [MorseSymbol::Dash],
            _ => panic!("invalid char: {}", val),
        })
        .collect::<Vec<[MorseSymbol; 1]>>()
        .join(&MorseSymbol::InterElementSpace)
}
