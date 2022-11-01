use fundsp::prelude::AudioUnit64;
use iced::Color;

pub fn hex_to_color(hex: &str) -> Option<Color> {
    if hex.len() == 7 {
        let hash = &hex[0..1];
        let r = u8::from_str_radix(&hex[1..3], 16);
        let g = u8::from_str_radix(&hex[3..5], 16);
        let b = u8::from_str_radix(&hex[5..7], 16);

        return match (hash, r, g, b) {
            ("#", Ok(r), Ok(g), Ok(b)) => Some(Color {
                r: r as f32 / 255.0,
                g: g as f32 / 255.0,
                b: b as f32 / 255.0,
                a: 1.0,
            }),
            _ => None,
        };
    }

    None
}

pub fn combine(sounds: Vec<Box<dyn AudioUnit64>>) -> (f64, f64) {
    let (mut l, mut r) = (0.0, 0.0);
    for mut sound in sounds {
        let (ls, rs) = sound.get_stereo();
        l += ls;
        r += rs;
    }
    (l, r)
}
