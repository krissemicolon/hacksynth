use midir::{MidiInput, MidiInputPort};

pub fn get_midi_device<'a>() -> Option<MidiInputPort> {
    let midi_in: MidiInput = match MidiInput::new("midir reading input") {
        Ok(midi_in) => midi_in,
        Err(_) => return None,
    };
    let midi_in_ports = midi_in.ports();

    for port in &midi_in_ports {
        println!("midi devices: {}", midi_in.port_name(&port).unwrap());
    }

    if midi_in_ports.len() == 0 {
        return None;
    }

    Some(midi_in_ports[0].clone())
}
