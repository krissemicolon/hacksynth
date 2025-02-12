use crate::{
    audio,
    oscillator::{Oscillator, Waveform, ADSR},
    styling,
};
use crossbeam_queue::SegQueue;
use iced::{
    image, pick_list, Alignment, Column, Container, Element, Image, Length, PickList, Row, Sandbox,
    Text,
};
use iced_audio::{knob, FloatRange, FreqRange, Knob, Normal};
use log::info;
use midi_msg::MidiMsg;
use midir::MidiInputConnection;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub enum Message {
    DetuneOsc1(Normal),
    WaveformOsc1Selected(Waveform),
    AttackOsc1(Normal),
    DecayOsc1(Normal),
    SustainOsc1(Normal),
    ReleaseOsc1(Normal),
    DetuneOsc2(Normal),
    WaveformOsc2Selected(Waveform),
    AttackOsc2(Normal),
    DecayOsc2(Normal),
    SustainOsc2(Normal),
    ReleaseOsc2(Normal),
    CutoffF1(Normal),
    ResonanceF1(Normal),
    CutoffF2(Normal),
    ResonanceF2(Normal),
}

pub struct App {
    _midi_msgs: Arc<SegQueue<MidiMsg>>,
    _connection: Option<MidiInputConnection<()>>,
    osc1: Arc<RwLock<Oscillator>>,
    osc2: Arc<RwLock<Oscillator>>,
    f1: Arc<RwLock<(f64, f64)>>,
    f2: Arc<RwLock<(f64, f64)>>,

    // ui from here on out
    detune_range: FloatRange,
    adsr_range: FloatRange,
    freq_range: FreqRange,
    resonance_range: FloatRange,

    // osc1
    osc1_detune_state: knob::State,
    osc1_detune_label: String,
    osc1_waveform_state: pick_list::State<Waveform>,
    osc1_waveform_selected: Option<Waveform>,
    osc1_waveform_label: String,
    osc1_attack_state: knob::State,
    osc1_attack_label: String,
    osc1_decay_state: knob::State,
    osc1_decay_label: String,
    osc1_sustain_state: knob::State,
    osc1_sustain_label: String,
    osc1_release_state: knob::State,
    osc1_release_label: String,

    // osc2
    osc2_detune_state: knob::State,
    osc2_detune_label: String,
    osc2_waveform_state: pick_list::State<Waveform>,
    osc2_waveform_selected: Option<Waveform>,
    osc2_waveform_label: String,
    osc2_attack_state: knob::State,
    osc2_attack_label: String,
    osc2_decay_state: knob::State,
    osc2_decay_label: String,
    osc2_sustain_state: knob::State,
    osc2_sustain_label: String,
    osc2_release_state: knob::State,
    osc2_release_label: String,

    // f1
    f1_cutoff_state: knob::State,
    f1_cutoff_label: String,
    f1_resonance_state: knob::State,
    f1_resonance_label: String,

    // f2
    f2_cutoff_state: knob::State,
    f2_cutoff_label: String,
    f2_resonance_state: knob::State,
    f2_resonance_label: String,
}

impl Sandbox for App {
    type Message = Message;

    fn title(&self) -> String {
        "Hacksynth".to_string()
    }

    fn new() -> App {
        let osc1 = Arc::new(RwLock::new(Oscillator::new(
            crate::oscillator::Waveform::Square,
            ADSR(0.11, 0.14, 0.47, 0.63),
            0.0,
        )));
        let osc2 = Arc::new(RwLock::new(Oscillator::new(
            crate::oscillator::Waveform::Sine,
            ADSR(0.11, 0.14, 0.47, 0.63),
            0.0,
        )));

        let f1 = Arc::new(RwLock::new((20000.0, 1.0)));
        let f2 = Arc::new(RwLock::new((20000.0, 1.0)));

        let _midi_msgs = Arc::new(SegQueue::new());
        // This has to be retained to ensure the connection is not dropped
        let _connection = audio::run_midi(_midi_msgs.clone()).ok();
        audio::setup_output(
            _midi_msgs.clone(),
            vec![osc1.clone(), osc2.clone()],
            vec![f1.clone(), f2.clone()],
        );

        let detune_range = FloatRange::new(-100.0, 100.0);
        let adsr_range = FloatRange::new(0.0, 1.0);
        let freq_range = FreqRange::default();
        let resonance_range = FloatRange::new(0.0, 100.0);

        // osc1 state
        let osc1_detune_state =
            knob::State::new(detune_range.normal_param(osc1.read().unwrap().detune, 0.0));
        let osc1_detune_label = format!("Detune\n{} Hz", osc1.read().unwrap().detune);
        let osc1_waveform_state = pick_list::State::default();
        let osc1_waveform_selected = Some(osc1.read().unwrap().waveform);
        let osc1_waveform_label = format!("Waveform");
        let osc1_attack_state =
            knob::State::new(adsr_range.normal_param(osc1.read().unwrap().adsr.0 as f32, 0.0));
        let osc1_attack_label = format!("Attack\n{} s", osc1.read().unwrap().adsr.0 as f32);
        let osc1_decay_state =
            knob::State::new(adsr_range.normal_param(osc1.read().unwrap().adsr.1 as f32, 0.0));
        let osc1_decay_label = format!("Decay\n{} s", osc1.read().unwrap().adsr.1 as f32);
        let osc1_sustain_state =
            knob::State::new(adsr_range.normal_param(osc1.read().unwrap().adsr.2 as f32, 0.0));
        let osc1_sustain_label = format!("Sustain\n{} s", osc1.read().unwrap().adsr.2 as f32);
        let osc1_release_state =
            knob::State::new(adsr_range.normal_param(osc1.read().unwrap().adsr.3 as f32, 0.0));
        let osc1_release_label = format!("Release\n{} s", osc1.read().unwrap().adsr.3 as f32);

        // osc2 state
        let osc2_detune_state =
            knob::State::new(detune_range.normal_param(osc2.read().unwrap().detune, 0.0));
        let osc2_detune_label = format!("Detune\n{} Hz", osc2.read().unwrap().detune);
        let osc2_waveform_state = pick_list::State::default();
        let osc2_waveform_selected = Some(osc2.read().unwrap().waveform);
        let osc2_waveform_label = format!("Waveform");
        let osc2_attack_state =
            knob::State::new(adsr_range.normal_param(osc2.read().unwrap().adsr.0 as f32, 0.0));
        let osc2_attack_label = format!("Attack\n{} s", osc2.read().unwrap().adsr.0 as f32);
        let osc2_decay_state =
            knob::State::new(adsr_range.normal_param(osc2.read().unwrap().adsr.1 as f32, 0.0));
        let osc2_decay_label = format!("Decay\n{} s", osc2.read().unwrap().adsr.1 as f32);
        let osc2_sustain_state =
            knob::State::new(adsr_range.normal_param(osc2.read().unwrap().adsr.2 as f32, 0.0));
        let osc2_sustain_label = format!("Sustain\n{} s", osc2.read().unwrap().adsr.2 as f32);
        let osc2_release_state =
            knob::State::new(adsr_range.normal_param(osc2.read().unwrap().adsr.3 as f32, 0.0));
        let osc2_release_label = format!("Release\n{} s", osc2.read().unwrap().adsr.3 as f32);

        // f1 state
        let f1_cutoff_state = knob::State::new(freq_range.default_normal_param());
        let f1_cutoff_label = format!("Cutoff\n 20000.00 Hz");
        let f1_resonance_state = knob::State::new(resonance_range.default_normal_param());
        let f1_resonance_label = format!("Resonance\n 0.00 %");

        // f2 state
        let f2_cutoff_state = knob::State::new(freq_range.default_normal_param());
        let f2_cutoff_label = format!("Cutoff\n 20000.00 Hz");
        let f2_resonance_state = knob::State::new(resonance_range.default_normal_param());
        let f2_resonance_label = format!("Resonance\n 0.00 %");

        App {
            _midi_msgs,
            _connection,
            osc1,
            osc2,
            f1,
            f2,

            // ui from here on out
            detune_range,
            adsr_range,
            freq_range,
            resonance_range,

            // osc1 state
            osc1_detune_state,
            osc1_detune_label,
            osc1_waveform_state,
            osc1_waveform_selected,
            osc1_waveform_label,
            osc1_attack_state,
            osc1_attack_label,
            osc1_decay_state,
            osc1_decay_label,
            osc1_sustain_state,
            osc1_sustain_label,
            osc1_release_state,
            osc1_release_label,

            // osc2 state
            osc2_detune_state,
            osc2_detune_label,
            osc2_waveform_state,
            osc2_waveform_selected,
            osc2_waveform_label,
            osc2_attack_state,
            osc2_attack_label,
            osc2_decay_state,
            osc2_decay_label,
            osc2_sustain_state,
            osc2_sustain_label,
            osc2_release_state,
            osc2_release_label,

            // f1 state
            f1_cutoff_state,
            f1_cutoff_label,
            f1_resonance_state,
            f1_resonance_label,

            // f2 state
            f2_cutoff_state,
            f2_cutoff_label,
            f2_resonance_state,
            f2_resonance_label,
        }
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::DetuneOsc1(normal) => {
                let value = self.detune_range.unmap_to_value(normal);
                self.osc1.write().unwrap().detune = value;
                self.osc1_detune_label = format!("Detune\n{:+.1} Hz", value);
                info!("detune osc1: {value} Hz")
            }
            Message::WaveformOsc1Selected(waveform) => {
                self.osc1_waveform_selected = Some(waveform);
                self.osc1.write().unwrap().waveform = waveform;
            }
            Message::AttackOsc1(normal) => {
                let value = self.adsr_range.unmap_to_value(normal);
                self.osc1.write().unwrap().adsr.0 = value as f64;
                self.osc1_attack_label = format!("Attack\n{:.2} s", value);
                info!("attack osc1: {value} s")
            }
            Message::DecayOsc1(normal) => {
                let value = self.adsr_range.unmap_to_value(normal);
                self.osc1.write().unwrap().adsr.1 = value as f64;
                self.osc1_decay_label = format!("Decay\n{:.2} s", value);
                info!("decay osc1: {value} s")
            }
            Message::SustainOsc1(normal) => {
                let value = self.adsr_range.unmap_to_value(normal);
                self.osc1.write().unwrap().adsr.2 = value as f64;
                self.osc1_sustain_label = format!("Sustain\n{:.2} s", value);
                info!("sustain osc1: {value} s")
            }
            Message::ReleaseOsc1(normal) => {
                let value = self.adsr_range.unmap_to_value(normal);
                self.osc1.write().unwrap().adsr.3 = value as f64;
                self.osc1_release_label = format!("Release\n{:.2} s", value);
                info!("release osc1: {value} s")
            }
            Message::DetuneOsc2(normal) => {
                let value = self.detune_range.unmap_to_value(normal);
                self.osc2.write().unwrap().detune = value;
                self.osc2_detune_label = format!("Detune\n{:+.1} Hz", value);
                info!("detune osc2: {value} Hz")
            }
            Message::WaveformOsc2Selected(waveform) => {
                self.osc2_waveform_selected = Some(waveform);
                self.osc2.write().unwrap().waveform = waveform;
            }
            Message::AttackOsc2(normal) => {
                let value = self.adsr_range.unmap_to_value(normal);
                self.osc2.write().unwrap().adsr.0 = value as f64;
                self.osc2_attack_label = format!("Attack\n{:.2} s", value);
                info!("attack osc2: {value} s")
            }
            Message::DecayOsc2(normal) => {
                let value = self.adsr_range.unmap_to_value(normal);
                self.osc2.write().unwrap().adsr.1 = value as f64;
                self.osc2_decay_label = format!("Decay\n{:.2} s", value);
                info!("decay osc2: {value} s")
            }
            Message::SustainOsc2(normal) => {
                let value = self.adsr_range.unmap_to_value(normal);
                self.osc2.write().unwrap().adsr.2 = value as f64;
                self.osc2_sustain_label = format!("Sustain\n{:.2} s", value);
                info!("sustain osc2: {value} s")
            }
            Message::ReleaseOsc2(normal) => {
                let value = self.adsr_range.unmap_to_value(normal);
                self.osc2.write().unwrap().adsr.3 = value as f64;
                self.osc2_release_label = format!("Release\n{:.2} s", value);
                info!("release osc2: {value} s")
            }
            Message::CutoffF1(normal) => {
                let value = self.freq_range.unmap_to_value(normal);
                self.f1.write().unwrap().0 = value as f64;
                self.f1_cutoff_label = format!("Cutoff\n{:.2} Hz", value);
                info!("cutoff f1: {value} Hz")
            }
            Message::ResonanceF1(normal) => {
                let value = self.resonance_range.unmap_to_value(normal);
                self.f1.write().unwrap().1 = value as f64;
                self.f1_resonance_label = format!("Resonance\n{:.2} %", value);
                info!("resonance f1: {value} %")
            }
            Message::CutoffF2(normal) => {
                let value = self.freq_range.unmap_to_value(normal);
                self.f2.write().unwrap().0 = value as f64;
                self.f2_cutoff_label = format!("Cutoff\n{:.2} Hz", value);
                info!("cutoff f2: {value} Hz")
            }
            Message::ResonanceF2(normal) => {
                let value = self.resonance_range.unmap_to_value(normal);
                self.f2.write().unwrap().1 = value as f64;
                self.f2_resonance_label = format!("Resonance\n{:.2} %", value);
                info!("resonance f2: {value} %")
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let osc1_detune = Knob::new(
            &mut self.osc1_detune_state,
            Message::DetuneOsc1,
            || None,
            || None,
        );

        let osc1_waveform = PickList::new(
            &mut self.osc1_waveform_state,
            &Waveform::ALL[..],
            self.osc1_waveform_selected,
            Message::WaveformOsc1Selected,
        );

        let osc1_attack = Knob::new(
            &mut self.osc1_attack_state,
            Message::AttackOsc1,
            || None,
            || None,
        );

        let osc1_decay = Knob::new(
            &mut self.osc1_decay_state,
            Message::DecayOsc1,
            || None,
            || None,
        );

        let osc1_sustain = Knob::new(
            &mut self.osc1_sustain_state,
            Message::SustainOsc1,
            || None,
            || None,
        );

        let osc1_release = Knob::new(
            &mut self.osc1_release_state,
            Message::ReleaseOsc1,
            || None,
            || None,
        );

        let osc1 = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .spacing(5)
                .padding(5)
                .push(Text::new("Oscillator 1").size(12))
                .push(
                    Row::new()
                        .spacing(8)
                        .width(Length::Fill)
                        .align_items(Alignment::Center)
                        .push(
                            Column::new()
                                .push(Text::new(&self.osc1_detune_label).size(12))
                                .push(osc1_detune),
                        )
                        .push(
                            Column::new()
                                .push(Text::new(&self.osc1_waveform_label).size(12))
                                .push(osc1_waveform),
                        ),
                )
                .push(
                    Row::new()
                        .spacing(8)
                        .width(Length::Fill)
                        .align_items(Alignment::Center)
                        .push(
                            Column::new()
                                .push(Text::new(&self.osc1_attack_label).size(12))
                                .push(osc1_attack),
                        )
                        .push(
                            Column::new()
                                .push(Text::new(&self.osc1_decay_label).size(12))
                                .push(osc1_decay),
                        ),
                )
                .push(
                    Row::new()
                        .spacing(8)
                        .width(Length::Fill)
                        .align_items(Alignment::Center)
                        .push(
                            Column::new()
                                .push(Text::new(&self.osc1_sustain_label).size(12))
                                .push(osc1_sustain),
                        )
                        .push(
                            Column::new()
                                .push(Text::new(&self.osc1_release_label).size(12))
                                .push(osc1_release),
                        ),
                ),
        )
        .style(styling::GroupContainer)
        .width(Length::Fill);

        let osc2_detune = Knob::new(
            &mut self.osc2_detune_state,
            Message::DetuneOsc2,
            || None,
            || None,
        );

        let osc2_waveform = PickList::new(
            &mut self.osc2_waveform_state,
            &Waveform::ALL[..],
            self.osc2_waveform_selected,
            Message::WaveformOsc2Selected,
        );

        let osc2_attack = Knob::new(
            &mut self.osc2_attack_state,
            Message::AttackOsc2,
            || None,
            || None,
        );

        let osc2_decay = Knob::new(
            &mut self.osc2_decay_state,
            Message::DecayOsc2,
            || None,
            || None,
        );

        let osc2_sustain = Knob::new(
            &mut self.osc2_sustain_state,
            Message::SustainOsc2,
            || None,
            || None,
        );

        let osc2_release = Knob::new(
            &mut self.osc2_release_state,
            Message::ReleaseOsc2,
            || None,
            || None,
        );

        let osc2 = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .spacing(5)
                .padding(5)
                .push(Text::new("Oscillator 2").size(12))
                .push(
                    Row::new()
                        .spacing(8)
                        .width(Length::Fill)
                        .align_items(Alignment::Center)
                        .push(
                            Column::new()
                                .push(Text::new(&self.osc2_detune_label).size(12))
                                .push(osc2_detune),
                        )
                        .push(
                            Column::new()
                                .push(Text::new(&self.osc2_waveform_label).size(12))
                                .push(osc2_waveform),
                        ),
                )
                .push(
                    Row::new()
                        .spacing(8)
                        .width(Length::Fill)
                        .align_items(Alignment::Center)
                        .push(
                            Column::new()
                                .push(Text::new(&self.osc2_attack_label).size(12))
                                .push(osc2_attack),
                        )
                        .push(
                            Column::new()
                                .push(Text::new(&self.osc2_decay_label).size(12))
                                .push(osc2_decay),
                        ),
                )
                .push(
                    Row::new()
                        .spacing(8)
                        .width(Length::Fill)
                        .align_items(Alignment::Center)
                        .push(
                            Column::new()
                                .push(Text::new(&self.osc2_sustain_label).size(12))
                                .push(osc2_sustain),
                        )
                        .push(
                            Column::new()
                                .push(Text::new(&self.osc2_release_label).size(12))
                                .push(osc2_release),
                        ),
                ),
        )
        .style(styling::GroupContainer)
        .width(Length::Fill);

        let oscillators_container = Container::new(Container::new(
            Row::new()
                .align_items(Alignment::Start)
                .push(Image::new(image::Handle::from_path(format!(
                    "{}/assets/oscillators_text.png",
                    env!("CARGO_MANIFEST_DIR")
                ))))
                .push(
                    Column::new()
                        .spacing(20)
                        .padding(20)
                        .align_items(Alignment::Start)
                        .push(osc1)
                        .push(osc2),
                ),
        ))
        .align_x(iced::alignment::Horizontal::Left)
        .width(Length::Units(214))
        .height(Length::Units(465))
        .max_width(214)
        .max_height(465)
        .style(styling::OscillatorsContainer);

        let f1_cutoff = Knob::new(
            &mut self.f1_cutoff_state,
            Message::CutoffF1,
            || None,
            || None,
        );

        let f1_resonance = Knob::new(
            &mut self.f1_resonance_state,
            Message::ResonanceF1,
            || None,
            || None,
        );

        let f1 = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .spacing(5)
                .padding(5)
                .push(Text::new("Filter 1").size(12))
                .push(
                    Row::new()
                        .spacing(8)
                        .width(Length::Fill)
                        .align_items(Alignment::Center)
                        .push(
                            Column::new()
                                .push(Text::new(&self.f1_cutoff_label).size(12))
                                .push(f1_cutoff),
                        )
                        .push(
                            Column::new()
                                .push(Text::new(&self.f1_resonance_label).size(12))
                                .push(f1_resonance),
                        ),
                ),
        )
        .style(styling::GroupContainer)
        .width(Length::Fill);

        let f2_cutoff = Knob::new(
            &mut self.f2_cutoff_state,
            Message::CutoffF2,
            || None,
            || None,
        );

        let f2_resonance = Knob::new(
            &mut self.f2_resonance_state,
            Message::ResonanceF2,
            || None,
            || None,
        );

        let f2 = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .spacing(5)
                .padding(5)
                .push(Text::new("Filter 2").size(12))
                .push(
                    Row::new()
                        .spacing(8)
                        .width(Length::Fill)
                        .align_items(Alignment::Center)
                        .push(
                            Column::new()
                                .push(Text::new(&self.f2_cutoff_label).size(12))
                                .push(f2_cutoff),
                        )
                        .push(
                            Column::new()
                                .push(Text::new(&self.f2_resonance_label).size(12))
                                .push(f2_resonance),
                        ),
                ),
        )
        .style(styling::GroupContainer)
        .width(Length::Fill);

        let filters_container = Container::new(Container::new(
            Row::new()
                .align_items(Alignment::Center)
                .push(Image::new(image::Handle::from_path(format!(
                    "{}/assets/filters_text.png",
                    env!("CARGO_MANIFEST_DIR")
                ))))
                .push(
                    Column::new()
                        .spacing(20)
                        .padding(20)
                        .align_items(Alignment::Start)
                        .push(f1)
                        .push(f2),
                ),
        ))
        .align_x(iced::alignment::Horizontal::Left)
        .width(Length::Units(214))
        .height(Length::Units(465))
        .max_width(214)
        .max_height(465)
        .style(styling::FiltersContainer);

        let effects_container = Container::new(Container::new(
            Row::new()
                .align_items(Alignment::End)
                .push(Image::new(image::Handle::from_path(format!(
                    "{}/assets/effects_text.png",
                    env!("CARGO_MANIFEST_DIR")
                ))))
                .push(
                    Column::new()
                        .spacing(20)
                        .padding(20)
                        .align_items(Alignment::Start),
                ),
        ))
        .align_x(iced::alignment::Horizontal::Left)
        .style(styling::EffectsContainer)
        .width(Length::Units(214))
        .height(Length::Units(465))
        .max_width(214)
        .max_height(465)
        .style(styling::EffectsContainer);

        // Push widgets into the iced DOM
        let content: Element<_> = Row::new()
            .spacing(4)
            .align_items(Alignment::Start)
            .push(oscillators_container)
            .push(filters_container)
            .push(effects_container)
            .into();

        Container::new(content)
            // .width(Length::Fill)
            // .height(Length::Fill)
            .max_width(650)
            .max_height(465)
            .center_x()
            .center_y()
            .into()
    }
}
