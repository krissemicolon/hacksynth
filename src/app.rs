use iced::{image, Alignment, Column, Container, Element, Image, Length, Row, Sandbox, Text};
use iced_audio::native::text_marks;
use iced_audio::text_marks::Group;
use iced_audio::{
    knob, tick_marks, v_slider, FloatRange, FreqRange, Knob, LogDBRange, Normal, VSlider,
};
use midi_msg::MidiMsg;
use midir::MidiInputConnection;
use std::sync::Arc;
use crossbeam_queue::SegQueue;

use crate::audio::Audio;
use crate::{styling, midi};

#[derive(Debug, Clone)]
pub enum Message {
    Float(Normal),
}

pub struct App {
    audio: Audio,

    detune_range: FloatRange,

    osc1_detune: knob::State,
    osc1_detune_label: String,
}

impl Sandbox for App {
    type Message = Message;

    fn title(&self) -> String {
        "Hacksynth".to_string()
    }

    fn new() -> App {
        let detune_range = FloatRange::new(-100.0, 100.0);

        App {
            audio: Audio::new(),

            detune_range,

            // osc1 state
            osc1_detune: knob::State::new(detune_range.default_normal_param()),
            osc1_detune_label: "Detune\n0 Hz".to_string(),
        }
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::Float(normal) => {
                let value = self.detune_range.unmap_to_value(normal);
                self.osc1_detune_label = format!("Detune\n{:+.1}", value);
                println!("detune osc1: {value} Hz")
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let detune_knob = Knob::new(
            &mut self.osc1_detune,
            Message::Float,
            || None,
            || None,
        );

        let osc1 = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .push(Text::new("osc1").size(12))
                .push(Text::new(&self.osc1_detune_label).size(12))
                .push(detune_knob)
                .spacing(5)
                .padding(5),
        )
        .style(styling::OscillatorContainer)
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
                        .push(osc1),
                ),
        ))
        .align_x(iced::alignment::Horizontal::Left)
        .width(Length::Units(214))
        .height(Length::Units(465))
        .max_width(214)
        .max_height(465)
        .style(styling::OscillatorsContainer);

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
                        // .push(fader_widget2),
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
                        .align_items(Alignment::Start)
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
