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
    pub core: Core,
    pub display: Display,
    pub queue: EventQueue,
    pub timer: Timer,
    pub font: Font,
    pub black: Color,
    pub white: Color,
    pub screen: Color,
    pub bitmap: Bitmap,
}

pub fn allegro_constructor(map: &tiled::Map) -> AllegroStructure {
    let core: Core = Core::init().unwrap();
    core.install_keyboard().unwrap();
	core.install_mouse().unwrap();
    let queue: EventQueue = EventQueue::new(&core).unwrap();
    let timer: Timer = Timer::new(&core, 1.0 / 60.0).unwrap();
    let font_addon: FontAddon = FontAddon::init(&core).unwrap();
    let font: Font = Font::new_builtin(&font_addon).unwrap();
    let bitmap: Bitmap = Bitmap::new(&core, map.width as i32, map.height as i32).unwrap();
    let display: Display = Display::new(&core, constants::WIDTH, constants::HEIGHT).unwrap();
    
    queue.register_event_source(display.get_event_source());
    queue.register_event_source(timer.get_event_source());
    queue.register_event_source(core.get_keyboard_event_source().unwrap());
    ImageAddon::init(&core).unwrap();

    let allegro_structure: AllegroStructure = AllegroStructure {
        core: core,
        display: display,
        queue: queue,
        timer: timer,
        font: font,
        black: Color::from_html_hex("#000000"),
        white: Color::from_html_hex("#FFFFFF"),
        screen: Color::from_html_hex("#000000"),
        bitmap
    };

    allegro_structure
}
