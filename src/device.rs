use midir::{MidiInput, MidiInputPorts};

pub fn get_midi_devices() -> Option<(MidiInput, MidiInputPorts)> {
    let midi_in: MidiInput = match MidiInput::new("midir reading input") {
        Ok(midi_in) => midi_in,
        Err(_) => return None,
    };
    let midi_in_ports = midi_in.ports();

    Some((midi_in, midi_in_ports))
}
