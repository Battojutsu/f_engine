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
#[path = "../constants.rs"]
mod constants;

#[path = "../load_map.rs"]
mod lm;

const MSG_TOP_X: f32 = 0 as f32;
const MSG_TOP_Y: f32 = (constants::HEIGHT - constants::HEIGHT / 3) as f32;
const MSG_BOT_X: f32 = constants::WIDTH as f32;
const MSG_BOT_Y: f32 = constants::HEIGHT as f32;



pub struct AllegroStructure {
    pub core: Core,
    pub primitives_addon: PrimitivesAddon,
    pub font_addon: FontAddon,
    pub display: Display,
    pub queue: EventQueue,
    pub timer: Timer,
    pub font: Font,
    pub black: Color,
    pub white: Color,
    pub screen: Color,
    pub bitmap: Bitmap,
    pub gray: Color,
    pub msg_font: Font,
}

pub fn allegro_constructor(map: &tiled::Map) -> AllegroStructure {
    let core: Core = Core::init().unwrap();
    core.install_keyboard().unwrap();
    core.install_mouse().unwrap();
    let queue: EventQueue = EventQueue::new(&core).unwrap();
    let timer: Timer = Timer::new(&core, 1.0 / 60.0).unwrap();
    let font_addon: FontAddon = FontAddon::init(&core).unwrap();
    let primitives_addon: PrimitivesAddon = PrimitivesAddon::init(&core).unwrap();
    
    let font: Font = Font::new_builtin(&font_addon).unwrap();
    let bitmap: Bitmap = Bitmap::new(&core, map.width as i32, map.height as i32).unwrap();
    let display: Display = Display::new(&core, constants::WIDTH, constants::HEIGHT).unwrap();

    let ttf_addon = TtfAddon::init(&font_addon).unwrap();
    let msg_font = match ttf_addon.load_ttf_font( "src/resources/fonts/unifont/unifont.otf", 50, TTF_MONOCHROME ) {
        Ok(v) => v,
        Err(e) => panic!("Error loading msg_font {e:?}"),
    };
    
    queue.register_event_source(display.get_event_source());
    queue.register_event_source(timer.get_event_source());
    queue.register_event_source(core.get_keyboard_event_source().unwrap());
    ImageAddon::init(&core).unwrap();

    let allegro_structure: AllegroStructure = AllegroStructure {
        core: core,
        primitives_addon: primitives_addon,
        font_addon: font_addon,
        display: display,
        queue: queue,
        timer: timer,
        font: font,
        black: Color::from_html_hex("#000000"),
        white: Color::from_html_hex("#FFFFFF"),
        screen: Color::from_html_hex("#000000"),
        gray: Color::from_html_hex("#777777"),
        bitmap,
        msg_font
    };

    allegro_structure
}


impl AllegroStructure {
    pub fn display_message(&self, message: &str) {
        self.primitives_addon.draw_filled_rectangle(MSG_TOP_X, MSG_TOP_Y, MSG_BOT_X, MSG_BOT_Y, self.gray);
        self.core.draw_text(&self.msg_font, self.white, MSG_TOP_X, MSG_TOP_Y, FontAlign::Left, message);
    }    
}