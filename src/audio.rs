use crossbeam_queue::SegQueue;
use fundsp::{hacker::sine_hz, prelude::*};
use midi_msg::MidiMsg;
use std::sync::Arc;

pub struct Audio {
    oscillators: Vec<Oscillator>,
}

impl Audio {
    pub fn new() -> Audio {
        Audio {
            oscillators: vec![Oscillator::new()],
        }
    }
}

pub fn test_output(midi_out: Arc<SegQueue<MidiMsg>>) {
    loop {
        if let Some(MidiMsg::ChannelVoice { channel: _, msg }) = midi_out.pop() {
            println!("{:?}", msg);
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
