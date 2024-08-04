#[path = "structs/allegro_container.rs"]
mod allegro_container;
#[path = "structs/game_container.rs"]
mod game;

use allegro::*;
use game::GameStructure;

pub fn main_loop() -> u32 {
    let mut engine: GameStructure = game::game_constructor("src/resources/maps/inside1.tmx");
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
