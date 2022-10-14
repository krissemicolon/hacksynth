use iced::{Alignment, Column, Container, Element, Sandbox, Settings}
use iced_audio::{Normal, VSliderDB, v_slider};

#[derive(Debug, Clone)]
pub enum Message {
    VSliderDB(Normal);
}

pub struct App {
    fader_range: LogDBRange,
    fader_state: v_slider::State,

    center_tick_mark: tick_marks::Group,
}

impl Sandbox for App {
    type Message = Message;

    fn title() -> String {
        "Hacksynth".to_string()
    }

    fn new() -> App {
        let fader_range = LogDBRange::new(-12.0, 12.0, 0.5.into());

        App {
            fader_range,
            fader_state: v_slider::State::new(
                db_range.default_normal_param(),
            )
            
            center_tick_mark: tick_marks::Group::center(tick_marks::Tier::Two),
        }
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::VSliderDB(normal) => {
                let value = self.db_range.unmap_to_value(normal);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let fader_widget = VSlider::new(&mut self.fader_state, Message::VSliderDB)
            .tick_marks(&self.center_tick_mark);

        // Push widgets into the iced DOM
        let content: Element<_> = Column::new()
            .max_width(300)
            .max_height(500)
            .spacing(20)
            .padding(20)
            .align_items(Alignment::Center)
            .push(fader_widget)
            .into();

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}