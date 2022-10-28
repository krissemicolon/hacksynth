use midir::{MidiInput, MidiInputPorts};

pub fn get_midi_devices() -> Option<(MidiInput, MidiInputPorts)> {
    let midi_in: MidiInput = match MidiInput::new("midir reading input") {
        Ok(midi_in) => midi_in,
        Err(_) => return None,
    };
    let midi_in_ports = midi_in.ports();

    for port in &midi_in_ports {
        println!("midi devices: {}", midi_in.port_name(&port).unwrap());
    }

    Some((midi_in, midi_in_ports))
}
