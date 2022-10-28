use app::App;
use crossbeam_queue::SegQueue;
use iced::{window, Sandbox, Settings};
use std::sync::Arc;
use std::thread;

mod app;
mod audio;
mod midi;
mod styling;
mod util;

fn main() {
    env_logger::init();

    let messages = Arc::new(SegQueue::new());
    let io_thread = thread::spawn(move || {
        if let Ok(_connection) = midi::run(messages.clone()) {
            audio::test_output(messages);
        }
    });

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

    io_thread.join().unwrap();
}
