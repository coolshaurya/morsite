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
        Self::new(morse, 1000, Duration::from_millis(60)) // A dot duration of 60ms is equal to 20 wpm on the PARIS standard
    }

    pub fn freq(&self) -> u32 {
        self.freq
    }

    pub fn dot_duration(&self) -> Duration {
        self.dot_duration
    }

    pub fn set_freq(&mut self, freq: u32) {
        self.freq = freq;
    }

    pub fn set_dot_duration(&mut self, dot_duration: Duration) {
        self.dot_duration = dot_duration;
    }

    pub fn set_paris_wpm(&mut self, paris_wpm: u64) {
        self.dot_duration = Duration::from_nanos((1200 * 1000 * 1000) / paris_wpm);
    }

    pub fn set_codex_wpm(&mut self, codex_wpm: u64) {
        self.dot_duration = Duration::from_nanos((1000 * 1000 * 1000) / codex_wpm);
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
