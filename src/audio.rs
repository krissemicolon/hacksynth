use crate::device;
use crossbeam_queue::SegQueue;
use midi_msg::MidiMsg;
use midir::{MidiInput, MidiInputPort};
use std::sync::Arc;
use std::thread;

pub struct Audio {
    pub midi_queue: Arc<SegQueue<MidiMsg>>,
}

impl Audio {
    pub fn new() -> Audio {
        let midi_in = device::get_midi_device();
        let midi_queue = Arc::new(SegQueue::new());

        let audio = Audio {
            midi_queue: midi_queue.clone(),
        };

        test_output(midi_queue.clone());
        listen_for_midi(midi_in.unwrap(), midi_queue);

        audio
    }
}

pub fn listen_for_midi(midi_in: (MidiInputPort, MidiInput), midi_out: Arc<SegQueue<MidiMsg>>) {
    let midi_port = midi_in.0;
    let midi_input = midi_in.1;

    println!("Using {:?}", midi_input.port_name(&midi_port));

    // let midi_port_name = midi_input.port_name(&midi_port).unwrap();
    let connection = midi_input.connect(
        &midi_port,
        "midir-read-input",
        move |_stamp, message, _| {
            println!("LMAO");
            let (msg, _len) = MidiMsg::from_midi(message).unwrap();
            midi_out.push(msg);
        },
        (),
    );

    println!(
        "Connection: {}",
        if connection.is_ok() { "OK" } else { "NOK" }
    );
}

pub fn test_output(midi_out: Arc<SegQueue<MidiMsg>>) {
    // loop {
    //     if let Some(msg) = midi_out.pop() {
    //         println!("{:?}", msg);
    //     }
    // }
    // thread::spawn(move || loop {
    //     if let Some(MidiMsg::ChannelVoice { channel: _, msg }) = midi_out.pop() {
    //         println!("{:?}", msg);
    //     }
    // });
}
