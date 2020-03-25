use morsite::{MorseRepr, MorseSound, MorseString, Text};
use rodio::{default_output_device, play_raw};
use std::thread::sleep;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "morsite", about = "plays some morse code")]
struct Args {
    /// The string that is input to the program
    input: String,

    //freq: u32,
    //paris_wpm: u32,
    //codex_wpm: u32,
    //dot_duration: u64,
    /// Make input use morse representation that uses -
    /// * fullstops[.] for dots
    /// * underscores[_] for dashes
    /// * concatenation for the space between two dots or dashes
    /// * spaces[ ] for the space between letters
    /// * slashes[/] for the space between words
    #[structopt(short, long)]
    repr: bool,

    #[structopt(short, long)]
    print: bool,
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

    if !args.mute {
        let sound = MorseSound::with_defaults(morse);
        play_morse(sound);
    }

    Ok(())
}

fn play_morse(morse_sound: MorseSound) {
    let device = default_output_device().unwrap();
    let dur = morse_sound.duration();
    play_raw(&device, morse_sound.source());
    sleep(dur);
}
