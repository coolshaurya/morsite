use crate::morse::{MorseString, MorseSymbol};
use rodio::source::{SineWave, Zero};
use rodio::Source;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct MorseSound {
    morse: MorseString,
    freq: u32,
    dot_duration: Duration,
}

type SourceSend = dyn Source<Item = f32> + Send;

impl MorseSound {
    pub fn new(morse: MorseString, freq: u32, dot_duration: Duration) -> Self {
        MorseSound {
            morse,
            freq,
            dot_duration,
        }
    }
    pub fn with_defaults(morse: MorseString) -> Self {
        Self::from_paris_wpm(morse, 1000, 20)
    }
    pub fn from_paris_wpm(morse: MorseString, freq: u32, paris_wpm: u64) -> Self {
        let dot_duration = Duration::from_nanos((1200 * 1000 * 1000) / paris_wpm);
        MorseSound {
            morse,
            freq,
            dot_duration,
        }
    }
    pub fn from_codex_wpm(morse: MorseString, freq: u32, codex_wpm: u64) -> Self {
        let dot_duration = Duration::from_nanos((1000 * 1000 * 1000) / codex_wpm);
        MorseSound {
            morse,
            freq,
            dot_duration,
        }
    }

    pub fn duration(&self) -> Duration {
        use MorseSymbol::*;
        let units: u32 = self
            .morse
            .iter()
            .map(|val| match val {
                Dot | InterElementSpace => 1,
                Dash | InterLetterSpace => 3,
                InterWordSpace => 7,
            })
            .sum();
        self.dot_duration * units
    }

    pub fn source(self) -> Box<SourceSend> {
        let freq = self.freq;
        let dot_duration = self.dot_duration;
        Box::new(rodio::source::from_iter(
            self.morse
                .into_iter()
                .map(move |val| gen_source_from_symbol(val, freq, dot_duration)),
        ))
    }
}

fn gen_source_from_symbol(
    symbol: MorseSymbol,
    freq: u32,
    dot_duration: Duration,
) -> Box<SourceSend> {
    use MorseSymbol::*;

    let beep = SineWave::new(freq);
    let silent: Zero<f32> = Zero::new(1, 48_000);

    match symbol {
        Dot => Box::new(beep.clone().take_duration(dot_duration)) as Box<SourceSend>,
        Dash => Box::new(beep.clone().take_duration(dot_duration * 3)) as Box<SourceSend>,
        InterElementSpace => {
            Box::new(silent.clone().take_duration(dot_duration)) as Box<SourceSend>
        }
        InterLetterSpace => {
            Box::new(silent.clone().take_duration(dot_duration * 3)) as Box<SourceSend>
        }
        InterWordSpace => {
            Box::new(silent.clone().take_duration(dot_duration * 7)) as Box<SourceSend>
        }
    }
}
