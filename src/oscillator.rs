use std::{collections::VecDeque};

use fundsp::{
    hacker::{adsr_live, saw, sine, square, triangle},
    prelude::{midi_hz, An, AudioUnit64, Var},
};

#[derive(Clone)]
pub struct ADSR(pub f64, pub f64, pub f64, pub f64);

#[derive(Clone)]
pub enum Waveform {
    Sine,
    Triangle,
    Sawtooth,
    Square,
}

#[derive(Clone)]
pub struct Oscillator {
    pub waveform: Waveform,
    pub adsr: ADSR,
    pub detune: f32,
    pub sample_rate: u64,
}

impl Oscillator {
    pub fn new(waveform: Waveform, adsr: ADSR, detune: f32, sample_rate: u64) -> Oscillator {
        Oscillator {
            waveform,
            adsr,
            detune,
            sample_rate,
        }
    }

    pub fn default() -> Oscillator {
        Oscillator {
            waveform: Waveform::Sine,
            adsr: ADSR(0.1, 0.2, 0.4, 0.0),
            detune: 0.0,
            sample_rate: 44100,
        }
    }

    pub fn generate_note(
        &self,
        note: u8,
        velocity: u8,
        releasing: An<Var<f64>>,
        finished: An<Var<f64>>,
        pitch_bend: An<Var<f64>>,
    ) -> Box<dyn AudioUnit64> {
        let pitch = midi_hz(note as f64);
        let volume = velocity as f64 / 127.0;

        println!("== {} ==", self.detune);

        match &self.waveform {
            Waveform::Sine => Box::new(
                (pitch + self.detune as f64) * pitch_bend
                    >> sine()
                        * adsr_live(
                            self.adsr.0,
                            self.adsr.1,
                            self.adsr.2,
                            self.adsr.3,
                            releasing,
                            finished,
                        )
                        * volume
                        * 2.0,
            ),
            Waveform::Triangle => Box::new(
                (pitch + self.detune as f64) * pitch_bend
                    >> triangle()
                        * adsr_live(
                            self.adsr.0,
                            self.adsr.1,
                            self.adsr.2,
                            self.adsr.3,
                            releasing,
                            finished,
                        )
                        * volume
                        * 2.0,
            ),
            Waveform::Sawtooth => Box::new(
                (pitch + self.detune as f64) * pitch_bend
                    >> saw()
                        * adsr_live(
                            self.adsr.0,
                            self.adsr.1,
                            self.adsr.2,
                            self.adsr.3,
                            releasing,
                            finished,
                        )
                        * volume
                        * 2.0,
            ),
            Waveform::Square => Box::new(
                (pitch + self.detune as f64) * pitch_bend
                    >> square()
                        * adsr_live(
                            self.adsr.0,
                            self.adsr.1,
                            self.adsr.2,
                            self.adsr.3,
                            releasing,
                            finished,
                        )
                        * volume
                        * 2.0,
            ),
        }
    }

    pub fn release_all(&self, awaiting_release: &mut VecDeque<An<Var<f64>>>) {
        while let Some(m) = awaiting_release.pop_front() {
            m.set_value(1.0);
        }
    }
}
