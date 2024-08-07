use allegro::*;
use allegro_color::*;

pub struct ColorStructure {
    pub black: Color,
    pub white: Color,
    pub screen: Color,
    pub gray: Color
}

pub fn color_constructor() -> ColorStructure {
    ColorStructure {
        black: Color::from_html_hex("#000000"),
        white: Color::from_html_hex("#FFFFFF"),
        screen: Color::from_html_hex("#000000"),
        gray: Color::from_html_hex("#777777")
    }
}