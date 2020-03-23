use morsite::morse::{MorseString, MorseSymbol};
use morsite::reciter::MorseSound;
use rodio::{default_output_device, play_raw};
use std::thread::sleep;

fn main() {
    let device = default_output_device().unwrap();
    let sound = MorseSound::new_with_defaults(MorseString::from(MorseSymbol::Dash));

    let dur = sound.duration();
    play_raw(&device, sound.source());
    sleep(dur);
}
