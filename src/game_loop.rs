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
     */
    let mut engine: game_container::GameStructure = game_container::game_constructor();

    engine.alleg.timer.start();

    loop {
        if engine.redraw && engine.alleg.queue.is_empty() {
            engine.redraw = false;

            match &engine.map {
                Some(v) => { engine.alleg.draw_screen(v); },
                None => {
                    // If there isn't a map loaded then load the default one.
                    engine.load_default_map();
                }
            }
                        
            engine.reset_backbuffer();

            let map: &Bitmap = match engine.alleg.bitmap.as_ref() {
                Some(v) => v,
                None => {
                    continue;
                }
            };

            engine
                .alleg
                .core
                .draw_bitmap(map, 0.0, 0.0, FLIP_NONE);

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
