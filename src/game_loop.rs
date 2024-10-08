/*
    f_engine - A farming and village life game game engine.
    Copyright (C) 2024  BattoJutsu

    Licensed under GNU AGPLv3
*/

use crate::game_container::GameStructure;

pub fn main_loop() -> u32 {
    /*
     * Initialize Game Engine
     */
    let mut engine: GameStructure = GameStructure::new();

    engine.alleg.timer.start();

    loop {
        if engine.redraw && engine.alleg.queue.is_empty() {
            engine.redraw = false;

            match &engine.map {
                Some(v) => {
                    // If there is a map loaded then draw this screen from the map file.
                    engine.alleg.draw_screen(v);
                },
                None => {
                    // If there isn't a map loaded then load the default one.
                    engine.load_default_map();
                    continue;
                }
            }

            engine.alleg.player.draw(&engine.alleg.core);
            
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
