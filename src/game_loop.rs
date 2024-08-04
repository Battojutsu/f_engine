/*
    f_engine - A farming and village life game game engine.
    Copyright (C) 2024  BattoJutsu

    Licensed under AGPLV3
*/

use allegro::*;
use crate::game_container;

pub fn main_loop() -> u32 {
    let mut engine: game_container::GameStructure = game_container::game_constructor("src/resources/maps/inside1.tmx");
    let mut redraw: bool = true;
    loop {
        if redraw && engine.alleg.queue.is_empty() {
            {
                engine.alleg.draw_screen(&engine.map);
            }
            redraw = false;
            engine
                .alleg
                .core
                .set_target_bitmap(Some(engine.alleg.display.get_backbuffer()));
            engine.alleg.core.clear_to_color(engine.alleg.black);

            engine
                .alleg
                .core
                .draw_bitmap(&engine.alleg.bitmap, 0.0, 0.0, FLIP_NONE);

            // TODO Implement DRAW character where he should be.
            engine.alleg.display_message("Testing");
            engine.alleg.core.flip_display();
            {
                engine.handle_character_movements();
            }
        }
    }
}
