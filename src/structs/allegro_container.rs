/*
    f_engine - A farming and village life game game engine.
    Copyright (C) 2024  BattoJutsu

    Licensed under GNU AGPLv3
*/

use crate::animation_container::AnimationStructure;
// This file defines struct allegro_structure and how to initialize
// one. This file is meant to combine allegro variables to easily pass between functions.
use crate::color_container;
use crate::color_container::ColorStructure;
use crate::constants;
use crate::lm;

use allegro::*;
use allegro_font::*;
use allegro_image::*;
use allegro_primitives::*;
use allegro_ttf::*;

const MSG_TOP_X: f32 = 0 as f32;
const MSG_TOP_Y: f32 = (constants::HEIGHT - constants::HEIGHT / 3) as f32;
const MSG_BOT_X: f32 = constants::WIDTH as f32;
const MSG_BOT_Y: f32 = constants::HEIGHT as f32;

pub struct AllegroStructure {
    pub core: Core,
    pub primitives_addon: PrimitivesAddon,
    pub display: Display,
    pub queue: EventQueue,
    pub timer: Timer,
    pub bitmap: Option<Bitmap>,
    pub msg_font: Font,
    pub colors: ColorStructure,
    pub is_new_map: bool,
    pub player: AnimationStructure
}

pub fn allegro_constructor() -> AllegroStructure {
    // Core init
    let core: Core = Core::init().unwrap();

    // Addon init
    ImageAddon::init(&core).unwrap();
    let primitives_addon: PrimitivesAddon = PrimitivesAddon::init(&core).unwrap();
    let font_addon: FontAddon = FontAddon::init(&core).unwrap();
    let ttf_addon: TtfAddon = TtfAddon::init(&font_addon).unwrap();

    let colors: ColorStructure = color_container::color_constructor();

    let queue: EventQueue = match EventQueue::new(&core) {
        Ok(v) => v,
        Err(e) => panic!("Error loading queue. Error: {e:?}"),
    };

    let timer: Timer = match Timer::new(&core, 1.0 / 60.0) {
        Ok(v) => v,
        Err(e) => panic!("Error loading timer. Error: {e:?}"),
    };

    let display: Display = match Display::new(&core, constants::WIDTH, constants::HEIGHT) {
        Ok(v) => v,
        Err(e) => panic!("Error loading allegro display. Error: {e:?}"),
    };

    let msg_font = match ttf_addon.load_ttf_font(
        "src/resources/fonts/unifont/unifont.otf",
        50,
        TTF_MONOCHROME,
    ) {
        Ok(v) => v,
        Err(e) => panic!("Error loading msg_font. Error: {e:?}"),
    };

    core.install_keyboard().unwrap();
    core.install_mouse().unwrap();

    queue.register_event_source(display.get_event_source());
    queue.register_event_source(timer.get_event_source());
    queue.register_event_source(core.get_keyboard_event_source().unwrap());

    let player:AnimationStructure = AnimationStructure::new(&core, 32, 32);

    let allegro_structure: AllegroStructure = AllegroStructure {
        core,
        primitives_addon,
        display,
        queue,
        timer,
        colors,
        bitmap: None,
        msg_font,
        is_new_map: false,
        player
    };

    allegro_structure
}

impl AllegroStructure {
    pub fn display_message(&self, message: &str) {
        self.primitives_addon.draw_filled_rectangle(
            MSG_TOP_X,
            MSG_TOP_Y,
            MSG_BOT_X,
            MSG_BOT_Y,
            self.colors.gray,
        );
        self.core.draw_text(
            &self.msg_font,
            self.colors.white,
            MSG_TOP_X,
            MSG_TOP_Y,
            FontAlign::Left,
            message,
        );
    }

    /// reset_backbuffer is an abstraction that sets the target bitmap to the displays backbuffer.
    /// It then continues to clear to black. This should be called before drawing to the screen.
    fn reset_backbuffer(&self) {
        self.core
            .set_target_bitmap(Some(self.display.get_backbuffer()));
        self.core.clear_to_color(self.colors.black);
    }

    /// replace_screen_with:: Takes a bitmap and replaces the current display bitmap with the frame_bitmap.
    fn replace_screen_with(&self, frame_bitmap: &Bitmap) {
        self.reset_backbuffer();
        self.core.draw_bitmap(frame_bitmap, 0.0, 0.0, FLIP_NONE);
    }

    /// draw_screen: Replace current display bitmap with bitmap drawn from map data structure.
    /// Caller is responsible for checking if map is defined.
    pub fn draw_screen(&mut self, map: &tiled::Map) {
        self.reset_backbuffer();
        if self.is_new_map {
            self.bitmap = lm::load_map(&mut self.core, map);
            self.is_new_map = false;
        }
        self.replace_screen_with(self.bitmap.as_ref().unwrap());
    }
}
