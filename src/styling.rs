use iced::{container, Background};

use crate::util;

pub struct OscillatorsContainer;
impl container::StyleSheet for OscillatorsContainer {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: None,
            background: Some(Background::Color(util::hex_to_color("#333333").unwrap())),
            ..container::Style::default()
        }
    }
}

pub struct FiltersContainer;
impl container::StyleSheet for FiltersContainer {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: None,
            background: Some(Background::Color(util::hex_to_color("#727272").unwrap())),
            ..container::Style::default()
        }
    }
}

pub struct LFOsContainer;
impl container::StyleSheet for LFOsContainer {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: None,
            background: Some(Background::Color(util::hex_to_color("#b1b2b3").unwrap())),
            ..container::Style::default()
        }
    }
}
