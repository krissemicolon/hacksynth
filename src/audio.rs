use midir::{MidiInput, MidiInputPorts};

use crate::device;

pub struct Audio {
    midi: Option<(MidiInput, MidiInputPorts)>,
}

impl Audio {
    pub fn new() -> Audio {
        Audio {
            midi: device::get_midi_devices(),
        }
    }
}
