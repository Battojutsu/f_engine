/*
    f_engine - A farming and village life game game engine.
    Copyright (C) 2024  BattoJutsu

    Licensed under GNU AGPLv3
*/

use allegro::*;

pub struct AnimationStructure {
	pub frame: Bitmap,
    pub width: i32,
    pub height: i32
}

impl AnimationStructure {
	pub fn new(core: &Core, width: i32, height: i32) -> AnimationStructure {
        let frame = Bitmap::new(core, width, height).unwrap();

        let sprite_bitmap: Bitmap = match Bitmap::load(&core, "src/resources/sprites/Heroes_01/Heroes_01.png") {
            Ok(v) => v,
            Err(e) => panic!("Error loading tileset_bitmap {e:?}"),
        };

        core.set_target_bitmap(Some(&frame));
        core.draw_bitmap_region(&sprite_bitmap, 0.0, 0.0, width as f32, height as f32, 0.0, 0.0, FLIP_NONE);
        
        AnimationStructure {
            frame,
            width,
            height
        }
	}

    pub fn draw(&self, core: &Core) {
        core.draw_bitmap(&self.frame, 0.0, 0.0, FLIP_NONE);
    }
}
