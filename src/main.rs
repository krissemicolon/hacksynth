use app::App;
use iced::{window, Sandbox, Settings};

mod app;
mod audio;
mod oscillator;
mod styling;
mod util;

fn main() {
    env_logger::init();

    let settings = Settings {
        window: window::Settings {
            size: (650, 465),
            resizable: false,
            decorations: true,
            //icon: todo!(), TODO: maybe icon?
            ..window::Settings::default()
        },
        ..iced::Settings::default()
    };

    App::run(settings).unwrap();
}
