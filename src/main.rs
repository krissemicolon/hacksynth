use app::App;
use crossbeam_queue::SegQueue;
use iced::{window, Sandbox, Settings};
use std::sync::Arc;

mod app;
mod midi;
mod styling;
mod util;

fn main() {
    env_logger::init();

    let messages = Arc::new(SegQueue::new());
    // This has to be retained to ensure the connection is not dropped
    let _connection = midi::run(messages.clone()).unwrap();
    midi::setup_output(messages);

    let settings = Settings {
        window: window::Settings {
            size: (650, 465),
            resizable: false,
            decorations: true,
            //icon: todo!(), TODO: maybe icon?
            ..window::Settings::default()
        },
        // default_font: todo!(), TODO: Roboto font
        ..iced::Settings::default()
    };

    App::run(settings).unwrap();
}
