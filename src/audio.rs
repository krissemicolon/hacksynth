use fundsp::{hacker::sine_hz, prelude::*};
use midir::MidiInputPort;

use crate::device;

pub struct Audio {
    midi: Option<MidiInputPort>,
    oscillators: Vec<Oscillator>,
}

impl Audio {
    pub fn new() -> Audio {
        let midi = device::get_midi_device();
        Audio {
            midi,
            oscillators: vec![Oscillator::new()],
        }
    }
}

pub struct Oscillator {
    detune: f64,
}

impl Oscillator {
    pub fn new() -> Oscillator {
        Oscillator { detune: 0.0 }
    }

    pub fn sound(freq: f64) -> An<Pipe<f64, Constant<U1, f64>, Sine<f64>>> {
        sine_hz(freq)
    }
}
