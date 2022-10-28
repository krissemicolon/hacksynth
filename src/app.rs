use iced::{image, Alignment, Column, Container, Element, Image, Length, Row, Sandbox};
use iced_audio::native::text_marks;
use iced_audio::text_marks::Group;
use iced_audio::{
    knob, tick_marks, v_slider, FloatRange, FreqRange, Knob, LogDBRange, Normal, VSlider,
};

use crate::audio::Audio;
use crate::styling;

#[derive(Debug, Clone)]
pub enum Message {
    VSliderDB(Normal),
    Float(Normal),
}

pub struct App {
    audio: Audio,

    detune_range: FloatRange,
    detune_text_marks: Group,

    detune_knob_state: knob::State,

    fader_range: LogDBRange,
    fader_state: v_slider::State,
    fader_state2: v_slider::State,
    fader_state3: v_slider::State,

    center_tick_mark: tick_marks::Group,
}

impl Sandbox for App {
    type Message = Message;

    fn title(&self) -> String {
        "Hacksynth".to_string()
    }

    fn new() -> App {
        let fader_range = LogDBRange::new(-12.0, 12.0, 0.5.into());
        let detune_range = FloatRange::new(-100.0, 100.0);

        App {
            audio: Audio::new(),

            detune_range,
            detune_text_marks: Group::min_max_and_center("-100Hz", "100Hz", "Detune"),

            // osc1
            detune_knob_state: knob::State::new(detune_range.default_normal_param()),

            // demo elements
            fader_range,
            fader_state: v_slider::State::new(fader_range.default_normal_param()),
            fader_state2: v_slider::State::new(fader_range.default_normal_param()),
            fader_state3: v_slider::State::new(fader_range.default_normal_param()),

            center_tick_mark: tick_marks::Group::center(tick_marks::Tier::Two),
        }
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::VSliderDB(normal) => {
                let value = self.fader_range.unmap_to_value(normal);
                println!("Hacksynth Value: {value}");
            }
            Message::Float(normal) => {
                let value = self.detune_range.unmap_to_value(normal);
                println!("detune osc1: {value} Hz")
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        // let fader_widget = VSlider::new(&mut self.fader_state, Message::VSliderDB)
        //     .tick_marks(&self.center_tick_mark);
        let fader_widget2 = VSlider::new(&mut self.fader_state2, Message::VSliderDB)
            .tick_marks(&self.center_tick_mark);
        let fader_widget3 = VSlider::new(&mut self.fader_state3, Message::VSliderDB)
            .tick_marks(&self.center_tick_mark);

        let detune_knob = Knob::new(
            &mut self.detune_knob_state,
            Message::Float,
            || None,
            || None,
        )
        .text_marks(&self.detune_text_marks);

        let osc1 = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .push(detune_knob)
                .spacing(5)
                .padding(40),
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
                        .push(fader_widget2),
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
                        .push(fader_widget3),
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
