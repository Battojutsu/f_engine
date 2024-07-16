/*
    f_engine - A farming and village life game game engine.
    Copyright (C) 2024  BattoJutsu

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, version 3 of the License

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
// This file defines struct allegro_structure and how to initialize
// one. This file is meant to combine allegro variables to easily pass between functions.

use allegro::*;
use allegro_color::*;
use allegro_font::*;
use allegro_image::*;
use allegro_primitives::*;
use allegro_ttf::*;
use tiled::Loader;
#[path = "../constants.rs"]
mod constants;

#[path = "../load_map.rs"]
mod lm;

pub struct AllegroStructure {
    display: *const Display,
    queue: *const EventQueue,
    timer: *const Timer,
    font: *const Font,
    black: *const Color,
    white: *const Color,
    screen: *const Color,
    bitmap: Bitmap,
}

pub fn allegro_constructor(map: &tiled::Map, filename: &str) -> AllegroStructure {
    let core = Core::init().unwrap();
    let primitives_addon = PrimitivesAddon::init(&core).unwrap();
    let font_addon = FontAddon::init(&core).unwrap();
    let ttf_addon = TtfAddon::init(&font_addon).unwrap();
    let image_addon = ImageAddon::init(&core).unwrap();
    let font: Font = Font::new_builtin(&font_addon).unwrap();

    let mut loader: Loader = Loader::new();
    let display: Display = Display::new(&core, constants::WIDTH, constants::HEIGHT).unwrap();

    let allegro_structure: AllegroStructure = AllegroStructure {
        display: &display,
        queue: &EventQueue::new(&core).unwrap(),
        timer: &Timer::new(&core, 1.0 / 60.0).unwrap(),
        font: &font,
        black: &Color::from_html_hex("#000000"),
        white: &Color::from_html_hex("#FFFFFF"),
        screen: &Color::from_html_hex("#000000"),
        bitmap: lm::load_map(&core, &display, filename, map),
    };

    allegro_structure
}
