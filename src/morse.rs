#[derive(Debug, Clone, Copy)]
pub enum MorseSymbol {
    Dot,
    Dash,
    InterElementSpace,
    InterLetterSpace,
    InterWordSpace,
}

pub type MorseString = Vec<MorseSymbol>;

impl From<MorseSymbol> for MorseString {
    fn from(morse_char: MorseSymbol) -> Self {
        vec![morse_char]
    }
}
