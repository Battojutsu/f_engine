/*
    f_engine - A farming and village life game game engine.
    Copyright (C) 2024  BattoJutsu

    Licensed under GNU AGPLv3
*/

use allegro::*;
use allegro_color::*;

pub struct ColorStructure {
    pub black: Color,
    pub white: Color,
    pub gray: Color
}

pub fn color_constructor() -> ColorStructure {
    ColorStructure {
        black: Color::from_html_hex("#000000"),
        white: Color::from_html_hex("#FFFFFF"),
        gray: Color::from_html_hex("#777777")
    }
}