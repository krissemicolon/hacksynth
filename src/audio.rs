use midir::{MidiInput, MidiInputPort, MidiInputPorts};

use crate::device;

pub struct Audio<'a> {
    midi: Option<(MidiInput, MidiInputPorts)>,
    selected_midi: &'a MidiInputPort,
}

impl <'a> Audio <'a> {
    pub fn new() -> Audio<'a> {
        let midi = device::get_midi_devices();
        let selected_midi = match midi {
            Some((midi_input, ports)) => ports.get(0).unwrap(),
            None       => panic!("No MIDI Device found")
        };

        Audio {
            midi,
            selected_midi,
        }
    }
}
