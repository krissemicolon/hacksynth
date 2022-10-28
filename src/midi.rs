use anyhow::bail;
use crossbeam_queue::SegQueue;
use midi_msg::MidiMsg;
use midir::{Ignore, MidiInput, MidiInputConnection};
use std::sync::Arc;

pub fn run(midi_queue: Arc<SegQueue<MidiMsg>>) -> anyhow::Result<MidiInputConnection<()>> {
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => bail!("Could not detect a MIDI Input Device."),
        _ => &in_ports[0],
    };

    println!("\nOpening connection");
    // let in_port_name = midi_in.port_name(in_port)?;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let connection = midi_in
        .connect(
            in_port,
            "midir-read-input",
            move |_, message, _| {
                let (msg, _) = MidiMsg::from_midi(message).unwrap();
                midi_queue.push(msg);
            },
            (),
        )
        .unwrap();

    Ok(connection)
}
