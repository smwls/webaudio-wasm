// mod utils;

// // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// // allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, OscillatorType};

enum Token {
    Note,
    Name(String),
    Operation(String),


}

fn tokenize(input: &str) -> Result<Vec<Token>, String>{

}

struct Osc {
    osc_node: web_sys::OscillatorNode,
    gain_node: web_sys::GainNode,
    freq: f32,
    on: bool
}

type MidiNote = ;
type Offset = 
struct Note {
    midi: MidiNote,
    offset: 
}
// enum NoteName {
//     C,
//     D,
//     E,
//     F,
//     G,
//     A,
//     B,
// }

// enum Accidental {
//     Flat,
//     HalfFlat,
//     Natural,
//     HalfSharp,
//     Sharp
// }

// enum Offset {
//     Note(f32),
//     OctaveUp,
//     OctaveDown
// }

// // impl Accidental {
// //     fn offset(&self) -> f32 {
// //         match &self {
// //             Sharp => 1f32,
// //             Flat => -1f32,
// //             HalfSharp => 0.5f32,
// //             HalfFlat => -0.5f32
// //         }
// //     }
// // }

// struct Note {
//     note: NoteName,
//     octave: Octave,
//     accidental: Accidental
// }

impl Note {
    fn toFreq(&self) -> f32 {
        let accidental_offset = match self.accidental {
            Accidental::Flat => -1f32,
            Accidental::HalfFlat => -0.5f32,
            Accidental::Natural => 0f32,
            Accidental::HalfSharp => 0.5f32,
            Accidental::Sharp => 1f32
        };

        let note_offset = match self.note {
            NoteName::C => 0f32,
            NoteName::D => 2f32,
            NoteName::E => 4f32,
            NoteName::F => 5f32,
            NoteName::G => 7f32,
            NoteName::A => 9f32,
            NoteName::B => 11f32
        };
        
        let octave_offset = match self.octave {
            Octave::Zero => -4f32,
            Octave::One => -3f32,
            Octave::Two => -2f32,
            Octave::Three => -1f32,
            Octave::Four => 0f32,
            Octave::Five => 1f32,
            Octave::Six => 2f32,
            Octave::Seven => 3f32,
            Octave::Eight => 4f32,
            Octave::Nine => 5f32,
        }
        let offset = note_offset + 12f32*octave_offset + accidental_offset;
        2f32.powf(offset/12f32)*440f32
    }
}

fn detect_octave(freq: f32) -> Option(Octave) {
    let oct = 4f32 + (freq/440f32).log(2f32);
    if oct < 0f32 {
        None
    } else if oct < 1f32 {
        Some(Octave::Zero)
    } else if oct < 2f32 {
        Some(Octave::One)
    } else if oct < 3f32 {
        Some(Octave::Two)
    } else if oct < 4f32 {
        Some(Octave::Three)
    } else if oct < 5f32 {
        Some(Octave::Four)
    } else if oct < 6f32 {
        Some(Octave::Five)
    } else if oct < 7f32 {
        Some(Octave::Six)
    } else if oct < 8f32 {
        Some(Octave::Seven)
    } else {
        Some(Octave::Eight)
    }
}

struct ParseState {
    unparsed: String,
    base_octave: Octave,
    notes: Vec<Note>,
    last_named_note: Note,
    last_offset: Offset 
}

impl ParseState {
    fn parseOctave(&self) -> Option<ParseState> {

        let octave = match self.unparsed.get(0..1)
                .and_then(|ch| {
                    ch.parse::<i32>()
                }) {
                Some(p) => 
            }
            .ok()
            .and_then(detect_octave)
            

        // let octave = match self.unparsed.get(0..1) {
        //     Some(ch) => match ch.parse::<i32>() {
        //         Ok(c) if c >= 0 => Some(detectOctave(c as f32)),
        //         otherwise => None
        //     }
        //     None => None
        // };
        match octave {
            Some(o) => Some(ParseState { base_octave: o, unparsed: self.unparsed.get(1..), ..*self}),
            None => None
        }        
    }

    // fn parseNoteOffset(&self) -> Option<ParseState> {
    //     let offset = self.unparsed.get(0..2)
    //         .and_then(|ch| {
    //             if ch == "/" {
    //                 offset = Some(Offset::OctaveUp)
    //             } else if ch == '\' {
    //                 offset = Some(Offset::OctaveDown)
    //             } else {
    //                 offset = None
    //             }
    //         })
    //         .ok()
    //     match offset {
    //         Some(o) => Some(ParseState { last_offset: o, ..*self }),
    //         None => None
    //     }        
    // }

    fn parseOctaveOffset(&self) -> Option<ParseState> {
        let offset = self.unparsed.get(0..1)
            .and_then(|ch| {
                match ch {
                    "/" => Some(Offset::OctaveUp),
                    "\\" => Some(Offset::OctaveDown),
                    _ => None
                }
            });
            match offset {
            Some(o) => Some(ParseState { 
                base_octave: self.base_octave + match o {
                    Offset::OctaveUp => 1,
                    Offset::OctaveDown => -1,
                },
                last_offset: o, 
                unparsed: self.unparsed.get(1..).unwrap_or(String::from("")),
                ..*self }),
            None => None
        }
    }

    fn parseNoteToFreq(&self) -> Option<ParseState> {
        let to_parse = self.unparsed.get(0..1);

        //i know NOTHING about Rust, oh my god!! what the hell am i supposed to do here?

    }
}

/// Converts a midi note to frequency
///
/// A midi note is an integer, generally in the range of 21 to 108
// pub fn midi_to_freq(note: u8) -> f32 {
//     27.5 * 2f32.powf((note as f32 - 21.0) / 12.0)
// }

#[wasm_bindgen]
pub struct FmOsc {
    ctx: AudioContext,
    /// The primary oscillator.  This will be the fundamental frequency
    primary: web_sys::OscillatorNode,

    /// Overall gain (volume) control
    gain: web_sys::GainNode,

    /// Amount of frequency modulation
    fm_gain: web_sys::GainNode,

    /// The oscillator that will modulate the primary oscillator's frequency
    fm_osc: web_sys::OscillatorNode,

    /// The ratio between the primary frequency and the fm_osc frequency.
    ///
    /// Generally fractional values like 1/2 or 1/4 sound best
    fm_freq_ratio: f32,

    fm_gain_ratio: f32,
}



impl Drop for FmOsc {
    fn drop(&mut self) {
        let _ = self.ctx.close();
    }
}

#[wasm_bindgen]
impl FmOsc {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<FmOsc, JsValue> {
        let ctx = web_sys::AudioContext::new()?;

        // Create our web audio objects.
        let primary = ctx.create_oscillator()?;
        let fm_osc = ctx.create_oscillator()?;
        let gain = ctx.create_gain()?;
        let fm_gain = ctx.create_gain()?;

        // Some initial settings:
        primary.set_type(OscillatorType::Sine);
        primary.frequency().set_value(440.0); // A4 note
        gain.gain().set_value(0.0); // starts muted
        fm_gain.gain().set_value(0.0); // no initial frequency modulation
        fm_osc.set_type(OscillatorType::Sine);
        fm_osc.frequency().set_value(0.0);

        // Connect the nodes up!

        // The primary oscillator is routed through the gain node, so that
        // it can control the overall output volume.
        primary.connect_with_audio_node(&gain)?;

        // Then connect the gain node to the AudioContext destination (aka
        // your speakers).
        gain.connect_with_audio_node(&ctx.destination())?;

        // The FM oscillator is connected to its own gain node, so it can
        // control the amount of modulation.
        fm_osc.connect_with_audio_node(&fm_gain)?;

        // Connect the FM oscillator to the frequency parameter of the main
        // oscillator, so that the FM node can modulate its frequency.
        fm_gain.connect_with_audio_param(&primary.frequency())?;

        // Start the oscillators!
        primary.start()?;
        fm_osc.start()?;

        Ok(FmOsc {
            ctx,
            primary,
            gain,
            fm_gain,
            fm_osc,
            fm_freq_ratio: 0.0,
            fm_gain_ratio: 0.0,
        })
    }

    /// Sets the gain for this oscillator, between 0.0 and 1.0.
    #[wasm_bindgen]
    pub fn set_gain(&self, mut gain: f32) {
        if gain > 1.0 {
            gain = 1.0;
        }
        if gain < 0.0 {
            gain = 0.0;
        }
        self.gain.gain().set_value(gain);
    }

    #[wasm_bindgen]
    pub fn set_primary_frequency(&self, freq: f32) {
        self.primary.frequency().set_value(freq);

        // The frequency of the FM oscillator depends on the frequency of the
        // primary oscillator, so we update the frequency of both in this method.
        self.fm_osc.frequency().set_value(self.fm_freq_ratio * freq);
        self.fm_gain.gain().set_value(self.fm_gain_ratio * freq);
    }

    #[wasm_bindgen]
    pub fn set_note(&self, note: u8) {
        let freq = midi_to_freq(note);
        self.set_primary_frequency(freq);
    }

    /// This should be between 0 and 1, though higher values are accepted.
    #[wasm_bindgen]
    pub fn set_fm_amount(&mut self, amt: f32) {
        self.fm_gain_ratio = amt;

        self.fm_gain
            .gain()
            .set_value(self.fm_gain_ratio * self.primary.frequency().value());
    }

    /// This should be between 0 and 1, though higher values are accepted.
    #[wasm_bindgen]
    pub fn set_fm_frequency(&mut self, amt: f32) {
        self.fm_freq_ratio = amt;
        self.fm_osc
            .frequency()
            .set_value(self.fm_freq_ratio * self.primary.frequency().value());
    }
}