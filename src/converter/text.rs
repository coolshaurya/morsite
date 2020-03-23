use crate::morse::{MorseString, MorseSymbol};
use maplit::hashmap;

pub struct Text(String);

impl From<Text> for MorseString {
    fn from(text: Text) -> Self {
        parse_text(&text.0)
    }
}

fn parse_text(text: &str) -> MorseString {
    text.to_lowercase()
        .split(' ')
        .map(|val| parse_word(val))
        .collect::<Vec<MorseString>>()
        .join(&MorseSymbol::InterWordSpace)
}

fn parse_word(word: &str) -> MorseString {
    let punc_chars = vec![
        '.', ',', '?', '\'', '!', '/', '(', ')', '&', ':', ';', '=', '+', '-', '_', '"', '@', '$',
    ];
    word.chars()
        .filter(|val| val.is_ascii_alphanumeric() || punc_chars.contains(val))
        .map(|val| parse_letter(val))
        .collect::<Vec<MorseString>>()
        .join(&MorseSymbol::InterLetterSpace)
}

fn parse_letter(letter: char) -> MorseString {
    use MorseSymbol::*;
    let hashmap = hashmap! {
        'a' => vec![Dot,InterElementSpace,Dash],
        'b' => vec![Dash,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dot],
        'c' => vec![Dash,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dot],
        'd' => vec![Dash,InterElementSpace,Dot,InterElementSpace,Dot],
        'e' => vec![Dot],
        'f' => vec![Dot,InterElementSpace,Dot,InterElementSpace,Dash,InterElementSpace,Dot],
        'g' => vec![Dash,InterElementSpace,Dash,InterElementSpace,Dot],
        'h' => vec![Dot,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dot],
        'i' => vec![Dot,InterElementSpace,Dot],
        'j' => vec![Dot,InterElementSpace,Dash,InterElementSpace,Dash,InterElementSpace,Dash],
        'k' => vec![Dash,InterElementSpace,Dot,InterElementSpace,Dash],
        'l' => vec![Dot,InterElementSpace,Dash,InterElementSpace,Dot,InterElementSpace,Dot],
        'm' => vec![Dash,InterElementSpace,Dash],
        'n' => vec![Dash,InterElementSpace,Dot],
        'o' => vec![Dash,InterElementSpace,Dash,InterElementSpace,Dash],
        'p' => vec![Dot,InterElementSpace,Dash,InterElementSpace,Dash,InterElementSpace,Dot],
        'q' => vec![Dash,InterElementSpace,Dash,InterElementSpace,Dot,InterElementSpace,Dash],
        'r' => vec![Dot,InterElementSpace,Dash,InterElementSpace,Dot],
        's' => vec![Dot,InterElementSpace,Dot,InterElementSpace,Dot],
        't' => vec![Dash],
        'u' => vec![Dot,InterElementSpace,Dot,InterElementSpace,Dash],
        'v' => vec![Dot,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dash],
        'w' => vec![Dot,InterElementSpace,Dash,InterElementSpace,Dash],
        'x' => vec![Dash,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dash],
        'y' => vec![Dash,InterElementSpace,Dot,InterElementSpace,Dash,InterElementSpace,Dash],
        'z' => vec![Dash,InterElementSpace,Dash,InterElementSpace,Dot,InterElementSpace,Dot],
        '0' => vec![Dash,InterElementSpace,Dash,InterElementSpace,Dash,InterElementSpace,Dash,InterElementSpace,Dash],
        '1' => vec![Dot,InterElementSpace,Dash,InterElementSpace,Dash,InterElementSpace,Dash,InterElementSpace,Dash],
        '2' => vec![Dot,InterElementSpace,Dot,InterElementSpace,Dash,InterElementSpace,Dash,InterElementSpace,Dash],
        '3' => vec![Dot,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dash,InterElementSpace,Dash],
        '4' => vec![Dot,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dash],
        '5' => vec![Dot,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dot],
        '6' => vec![Dash,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dot],
        '7' => vec![Dash,InterElementSpace,Dash,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dot],
        '8' => vec![Dash,InterElementSpace,Dash,InterElementSpace,Dash,InterElementSpace,Dot,InterElementSpace,Dot],
        '9' => vec![Dash,InterElementSpace,Dash,InterElementSpace,Dash,InterElementSpace,Dash,InterElementSpace,Dot],
        '.' => vec![Dot,InterElementSpace,Dash,InterElementSpace,Dot,InterElementSpace,Dash,InterElementSpace,Dot,InterElementSpace,Dash],
        ',' => vec![Dash,InterElementSpace,Dash,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dash,InterElementSpace,Dash],
        '?' => vec![Dot,InterElementSpace,Dot,InterElementSpace,Dash,InterElementSpace,Dash,InterElementSpace,Dot,InterElementSpace,Dot],
        '\'' => vec![Dot,InterElementSpace,Dash,InterElementSpace,Dash,InterElementSpace,Dash,InterElementSpace,Dash,InterElementSpace,Dot],
        '!' => vec![Dash,InterElementSpace,Dot,InterElementSpace,Dash,InterElementSpace,Dot,InterElementSpace,Dash,InterElementSpace,Dash],
        '/' => vec![Dash,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,InterElementSpace,Dash,InterElementSpace,Dot],
        '(' => vec![Dash,InterElementSpace,Dot,InterElementSpace,Dash,InterElementSpace,Dash,InterElementSpace,Dot],
        ')' => vec![Dash,InterElementSpace,Dot,InterElementSpace,Dash,InterElementSpace,Dash,InterElementSpace,Dot,InterElementSpace,Dash],
        '&' => vec![Dot,InterElementSpace,Dash,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dot],
        ':' => vec![Dash,InterElementSpace,Dash,InterElementSpace,Dash,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dot],
        ';' => vec![Dash,InterElementSpace,Dot,InterElementSpace,Dash,InterElementSpace,Dot,InterElementSpace,Dash,InterElementSpace,Dot],
        '=' => vec![Dash,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dash],
        '+' => vec![Dot,InterElementSpace,Dash,InterElementSpace,Dot,InterElementSpace,Dash,InterElementSpace,Dot],
        '-' => vec![Dash,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dash],
        '_' => vec![Dot,InterElementSpace,Dot,InterElementSpace,Dash,InterElementSpace,Dash,InterElementSpace,Dot,InterElementSpace,Dash],
        '$' => vec![Dot,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dash,InterElementSpace,Dot,InterElementSpace,Dot,InterElementSpace,Dash],
        '@' => vec![Dot,InterElementSpace,Dash,InterElementSpace,Dash,InterElementSpace,Dot,InterElementSpace,Dash,InterElementSpace,Dot],
    };

    if let Some(morse_string) = hashmap.get(&letter) {
        morse_string.to_vec()
    } else {
        panic!("invalid char: {}", letter);
    }
}
