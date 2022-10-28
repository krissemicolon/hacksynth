use midir::MidiInputPort;

use crate::device;

pub struct Audio {
    midi: Option<MidiInputPort>,
}

impl Audio {
    pub fn new() -> Audio {
        let midi = device::get_midi_device();
        Audio { midi }
    }
}
