use iced::{
    image, Alignment, Column, Container, Element, Image, Length, Row, Sandbox, Text,
};
use iced_audio::{knob, FloatRange, FreqRange, Knob, Normal};
use crate::styling;

#[derive(Debug, Clone)]
pub enum Message {
    DetuneOsc1(Normal),
    AttackOsc1(Normal),
    DecayOsc1(Normal),
    SustainOsc1(Normal),
    ReleaseOsc1(Normal),
    DetuneOsc2(Normal),
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
    detune_range: FloatRange,
    adsr_range: FloatRange,
    freq_range: FreqRange,
    resonance_range: FloatRange,

    // osc1
    osc1_detune_state: knob::State,
    osc1_detune_label: String,
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
        let detune_range = FloatRange::new(-100.0, 100.0);
        let adsr_range = FloatRange::new(0.0, 2000.0);
        let freq_range = FreqRange::default();
        let resonance_range = FloatRange::new(0.0, 100.0);

        App {
            detune_range,
            adsr_range,
            freq_range,
            resonance_range,

            // osc1 state
            osc1_detune_state: knob::State::new(detune_range.default_normal_param()),
            osc1_detune_label: "Detune\n0 Hz".to_string(),
            osc1_attack_state: knob::State::new(adsr_range.default_normal_param()),
            osc1_attack_label: "Attack\n0 ms".to_string(),
            osc1_decay_state: knob::State::new(adsr_range.default_normal_param()),
            osc1_decay_label: "Decay\n0 ms".to_string(),
            osc1_sustain_state: knob::State::new(adsr_range.default_normal_param()),
            osc1_sustain_label: "Sustain\n0 ms".to_string(),
            osc1_release_state: knob::State::new(adsr_range.default_normal_param()),
            osc1_release_label: "Release\n0 ms".to_string(),

            // osc2 state
            osc2_detune_state: knob::State::new(detune_range.default_normal_param()),
            osc2_detune_label: "Detune\n0 Hz".to_string(),
            osc2_attack_state: knob::State::new(adsr_range.default_normal_param()),
            osc2_attack_label: "Attack\n0 ms".to_string(),
            osc2_decay_state: knob::State::new(adsr_range.default_normal_param()),
            osc2_decay_label: "Decay\n0 ms".to_string(),
            osc2_sustain_state: knob::State::new(adsr_range.default_normal_param()),
            osc2_sustain_label: "Sustain\n0 ms".to_string(),
            osc2_release_state: knob::State::new(adsr_range.default_normal_param()),
            osc2_release_label: "Release\n0 ms".to_string(),

            // f1 state
            f1_cutoff_state: knob::State::new(freq_range.default_normal_param()),
            f1_cutoff_label: "Cutoff\n ".to_string(),
            f1_resonance_state: knob::State::new(resonance_range.default_normal_param()),
            f1_resonance_label: "Resonance\n ".to_string(),

            // f2 state
            f2_cutoff_state: knob::State::new(freq_range.default_normal_param()),
            f2_cutoff_label: "Cutoff\n ".to_string(),
            f2_resonance_state: knob::State::new(resonance_range.default_normal_param()),
            f2_resonance_label: "Resonance\n w".to_string(),
        }
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::DetuneOsc1(normal) => {
                let value = self.detune_range.unmap_to_value(normal);
                self.osc1_detune_label = format!("Detune\n{:+.1}", value);
                println!("detune osc1: {value} Hz")
            }
            Message::AttackOsc1(normal) => {
                let value = self.adsr_range.unmap_to_value(normal);
                self.osc1_attack_label = format!("Attack\n{:.2}", value);
                println!("attack osc1: {value} ms")
            }
            Message::DecayOsc1(normal) => {
                let value = self.adsr_range.unmap_to_value(normal);
                self.osc1_decay_label = format!("Decay\n{:.2}", value);
                println!("decay osc1: {value} ms")
            }
            Message::SustainOsc1(normal) => {
                let value = self.adsr_range.unmap_to_value(normal);
                self.osc1_sustain_label = format!("Sustain\n{:.2}", value);
                println!("sustain osc1: {value} ms")
            }
            Message::ReleaseOsc1(normal) => {
                let value = self.adsr_range.unmap_to_value(normal);
                self.osc1_release_label = format!("Release\n{:.2}", value);
                println!("release osc1: {value} ms")
            }
            Message::DetuneOsc2(normal) => {
                let value = self.detune_range.unmap_to_value(normal);
                self.osc2_detune_label = format!("Detune\n{:+.1}", value);
                println!("detune osc2: {value} Hz")
            }
            Message::AttackOsc2(normal) => {
                let value = self.adsr_range.unmap_to_value(normal);
                self.osc2_attack_label = format!("Attack\n{:.2}", value);
                println!("attack osc2: {value} ms")
            }
            Message::DecayOsc2(normal) => {
                let value = self.adsr_range.unmap_to_value(normal);
                self.osc2_decay_label = format!("Decay\n{:.2}", value);
                println!("decay osc2: {value} ms")
            }
            Message::SustainOsc2(normal) => {
                let value = self.adsr_range.unmap_to_value(normal);
                self.osc2_sustain_label = format!("Sustain\n{:.2}", value);
                println!("sustain osc2: {value} ms")
            }
            Message::ReleaseOsc2(normal) => {
                let value = self.adsr_range.unmap_to_value(normal);
                self.osc2_release_label = format!("Release\n{:.2}", value);
                println!("release osc2: {value} ms")
            }
            Message::CutoffF1(normal) => {
                let value = self.freq_range.unmap_to_value(normal);
                self.f1_cutoff_label = format!("Cutoff\n{:.2} Hz", value);
                println!("cutoff f1: {value} Hz")
            }
            Message::ResonanceF1(normal) => {
                let value = self.resonance_range.unmap_to_value(normal);
                self.f1_resonance_label = format!("Resonance\n{:.2}", value);
                println!("resonance f1: {value} Hz")
            }
            Message::CutoffF2(normal) => {
                let value = self.freq_range.unmap_to_value(normal);
                self.f2_cutoff_label = format!("Cutoff\n{:.2} Hz", value);
                println!("cutoff f2: {value} Hz")
            }
            Message::ResonanceF2(normal) => {
                let value = self.resonance_range.unmap_to_value(normal);
                self.f2_resonance_label = format!("Resonance\n{:.2}", value);
                println!("resonance f2: {value} Hz")
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
                )
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
                )
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

        let lfos_container = Container::new(Container::new(
            Row::new()
                .align_items(Alignment::End)
                .push(Image::new(image::Handle::from_path(format!(
                    "{}/assets/lfos_text.png",
                    env!("CARGO_MANIFEST_DIR")
                ))))
                .push(
                    Column::new()
                        .spacing(20)
                        .padding(20)
                        .align_items(Alignment::Start),
                    // .push(fader_widget3),
                ),
        ))
        .align_x(iced::alignment::Horizontal::Left)
        .style(styling::LFOsContainer)
        .width(Length::Units(214))
        .height(Length::Units(465))
        .max_width(214)
        .max_height(465)
        .style(styling::LFOsContainer);

        // Push widgets into the iced DOM
        let content: Element<_> = Row::new()
            .spacing(4)
            .align_items(Alignment::Start)
            .push(oscillators_container)
            .push(filters_container)
            .push(lfos_container)
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
