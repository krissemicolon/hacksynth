use iced::{Settings, Sandbox, window};
use app::App;

mod app;
mod audio;
mod styling;
mod util;
mod device;

pub fn main() {
    env_logger::init();

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
