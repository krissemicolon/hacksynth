use iced::{Settings, Sandbox, window};
use ui::App;

mod ui;
mod styling;
mod util;

pub fn main() {
    let settings = Settings {
        window: window::Settings {
            size: (650, 465),
            resizable: false,
            decorations: true,
            //icon: todo!(), TODO: maybe icon?
            ..window::Settings::default()
        },
        // default_font: todo!(), TODO: Roboto font
        ..Settings::default()
    };

    App::run(settings).unwrap();
}
