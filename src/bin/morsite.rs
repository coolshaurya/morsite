use morsite::{MorseRepr, MorseSound, MorseString, Text};
use rodio::{default_output_device, play_raw};
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;

/// Plays some morse code. Morse is represented using "." for
/// dots, "_" for dashes, concatenation for the space between
/// elements, " " (space) for the space between letters and, "/" for
/// the space between words. e.g - "be nice" will be represented
/// as "_... . / _. .. _... .". The spaces around the "/" are
/// merely for readibility.
///       
#[derive(Debug, StructOpt)]
#[structopt(name = "morsite")]
struct Args {
    /// The text that is played
    input: String,

    /// The frequency at which to play the sound
    #[structopt(long, short)]
    freq: Option<u32>,

    /// The wpm according to the PARIS standard
    #[structopt(long, short = "w")]
    paris_wpm: Option<u64>,

    /// The wpm according to the CODEX standard
    #[structopt(long, short = "x")]
    codex_wpm: Option<u64>,

    /// The duration of a single dot in milliseconds
    #[structopt(long, short)]
    dot_duration: Option<u64>,

    /// Use morse representation for input
    #[structopt(short, long)]
    repr: bool,

    /// Print morse representation
    #[structopt(short, long)]
    print: bool,

    /// Don't play any audio. Use in combination with --print to convert text to morse
    #[structopt(short, long)]
    mute: bool,
}

fn main() -> Result<(), String> {
    let args = Args::from_args();
    let morse: MorseString = if args.repr {
        MorseRepr::new(args.input).into()
    } else {
        Text::new(args.input).into()
    };

    if args.print {
        println!("{}", MorseRepr::from(morse.clone()));
    }

    if args.mute {
        return Ok(());
    }

    let mut sound = MorseSound::with_defaults(morse);

    if args.freq.is_some() {
        sound.set_freq(args.freq.unwrap());
    }

    match (args.dot_duration, args.paris_wpm, args.codex_wpm) {
        (Some(dot_duration), _, _) => {
            sound.set_dot_duration(Duration::from_millis(dot_duration));
        }
        (_, Some(_), Some(_)) => {
            return Err(String::from(
                "Please only specify codex_wpm OR paris_wpm, not both",
            ));
        }
        (_, Some(paris_wpm), None) => sound.set_paris_wpm(paris_wpm),
        (_, None, Some(codex_wpm)) => sound.set_codex_wpm(codex_wpm),
        _ => {}
    };

    play_morse(sound);

    Ok(())
}

fn play_morse(morse_sound: MorseSound) {
    let device = default_output_device().unwrap();
    let dur = morse_sound.duration();
    play_raw(&device, morse_sound.source());
    sleep(dur);
}
