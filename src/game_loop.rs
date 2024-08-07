/*
    f_engine - A farming and village life game game engine.
    Copyright (C) 2024  BattoJutsu

    Licensed under GNU AGPLv3
*/

use allegro::*;
use crate::game_container;

pub fn main_loop() -> u32 {
    /*
     * Initialize Game Engine
     * 
     * TODO
     * Make it not require a map file for the constructor.
     */
    let mut engine: game_container::GameStructure = game_container::game_constructor("src/resources/maps/inside1.tmx");

    engine.alleg.timer.start();

    loop {
        if engine.redraw && engine.alleg.queue.is_empty() {
            engine.redraw = false;

            { engine.alleg.draw_screen(&engine.map); }
            
            engine
                .alleg
                .core
                .set_target_bitmap(Some(engine.alleg.display.get_backbuffer()));
            engine.alleg.core.clear_to_color(engine.alleg.colors.black);

            engine
                .alleg
                .core
                .draw_bitmap(&engine.alleg.bitmap, 0.0, 0.0, FLIP_NONE);

            // TODO Implement DRAW character where he should be.
            if engine.displaying_text {
                engine.alleg.display_message("Testing");
            }
            
            engine.alleg.core.flip_display();
        }
        {
            engine.handle_events();
        }
    }
}
