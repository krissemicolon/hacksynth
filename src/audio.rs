use crate::oscillator::Oscillator;
use anyhow::bail;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Sample, SampleFormat, StreamConfig};
use crossbeam_queue::SegQueue;
use fundsp::hacker::var;
use fundsp::prelude::{An, Tag, Var};
use log::info;
use midi_msg::{ChannelVoiceMsg, MidiMsg};
use midir::{Ignore, MidiInput, MidiInputConnection};
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};

const PITCH_TAG: Tag = 1;
const FINISHED_TAG: Tag = PITCH_TAG + 1;
const RELEASE_TAG: Tag = FINISHED_TAG + 1;

pub fn run_midi(midi_queue: Arc<SegQueue<MidiMsg>>) -> anyhow::Result<MidiInputConnection<()>> {
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => {
            bail!("Could not detect a MIDI Input Device.")
        }
        _ => &in_ports[0],
    };

    info!("Opening MIDI connection");
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

pub fn setup_output(
    midi_out: Arc<SegQueue<MidiMsg>>,
    oscillators: Vec<Arc<RwLock<Oscillator>>>,
    filters: Vec<Arc<RwLock<(f64, f64)>>>,
) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    info!("Audio device: {:?}", device.name().expect("None"));
    let config = device.default_output_config().unwrap();
    match config.sample_format() {
        SampleFormat::F32 => {
            output_sound::<f32>(oscillators, filters, midi_out, device, config.into())
        }
        SampleFormat::I16 => {
            output_sound::<i16>(oscillators, filters, midi_out, device, config.into())
        }
        SampleFormat::U16 => {
            output_sound::<u16>(oscillators, filters, midi_out, device, config.into())
        }
    }
}

fn output_sound<T: Sample>(
    oscillators: Vec<Arc<RwLock<Oscillator>>>,
    filters: Vec<Arc<RwLock<(f64, f64)>>>,
    midi_out: Arc<SegQueue<MidiMsg>>,
    device: Device,
    config: StreamConfig,
) {
    let device = Arc::new(device);
    let config = Arc::new(config);
    std::thread::spawn(move || {
        let mut awaiting_release: VecDeque<An<Var<f64>>> = VecDeque::new();
        loop {
            if let Some(MidiMsg::ChannelVoice { channel: _, msg }) = midi_out.pop() {
                info!("Received {msg:?}");
                match msg {
                    ChannelVoiceMsg::NoteOff {
                        note: _,
                        velocity: _,
                    } => {
                        for oscillator in &oscillators {
                            oscillator
                                .read()
                                .unwrap()
                                .release_all(&mut awaiting_release);
                        }
                    }
                    ChannelVoiceMsg::NoteOn { note, velocity } => {
                        let releasing = var(RELEASE_TAG, 0.0);
                        awaiting_release.push_back(releasing.clone());
                        start_sound::<T>(
                            oscillators.clone(),
                            filters.clone(),
                            note,
                            velocity,
                            releasing,
                            device.clone(),
                            config.clone(),
                        );
                    }
                    _ => {}
                }
            }
        }
    });
}

fn start_sound<T: Sample>(
    oscillators: Vec<Arc<RwLock<Oscillator>>>,
    filters: Vec<Arc<RwLock<(f64, f64)>>>,
    note: u8,
    velocity: u8,
    releasing: An<Var<f64>>,
    device: Arc<Device>,
    config: Arc<StreamConfig>,
) {
    let finished = var(FINISHED_TAG, 0.0);
    let pitch_bend = var(PITCH_TAG, 1.0);

    let f1 = filters[0].read().unwrap().clone();
    let f2 = filters[1].read().unwrap().clone();

    let mut sound_osc1 = oscillators[0].read().unwrap().generate_note(
        note,
        velocity,
        releasing.clone(),
        finished.clone(),
        pitch_bend.clone(),
        f1,
    );
    let mut sound_osc2 = oscillators[1].read().unwrap().generate_note(
        note,
        velocity,
        releasing.clone(),
        finished.clone(),
        pitch_bend.clone(),
        f2,
    );

    let mut next_value = move || {
        let (l1, r1) = sound_osc1.get_stereo();
        let (l2, r2) = sound_osc2.get_stereo();
        (l1 + l2, r1 + r2)
    };

    let channels = config.channels as usize;
    std::thread::spawn(move || {
        let err_fn = |err| eprintln!("an error occurred on stream: {err}");
        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    write_data(data, channels, &mut next_value)
                },
                err_fn,
            )
            .unwrap();

        stream.play().unwrap();
        while finished.value() == 0.0 {}
    });
}

fn write_data<T: Sample>(
    output: &mut [T],
    channels: usize,
    next_sample: &mut dyn FnMut() -> (f64, f64),
) {
    for frame in output.chunks_mut(channels) {
        let sample = next_sample();
        let left: T = Sample::from::<f32>(&(sample.0 as f32));
        let right: T = Sample::from::<f32>(&(sample.1 as f32));

        for (channel, sample) in frame.iter_mut().enumerate() {
            *sample = if channel & 1 == 0 { left } else { right };
        }
    }
}
