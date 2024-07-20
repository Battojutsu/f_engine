#[path = "structs/game_container.rs"] mod game;
#[path = "structs/allegro_container.rs"] mod allegro_container;
use allegro::FLIP_NONE;
use game::GameStructure;


pub fn main_loop() -> u32 {
    let engine: GameStructure = game::game_constructor("src/resources/maps/inside1.tmx");
	let mut redraw = true;

	

	loop {
		if redraw && engine.alleg.queue.is_empty() {
			redraw = false;
			engine.alleg.core.set_target_bitmap(Some(engine.alleg.display.get_backbuffer()));
			engine.alleg.core.clear_to_color(engine.alleg.black);
			engine.alleg.core.draw_bitmap(&engine.alleg.bitmap, 0.0, 0.0, FLIP_NONE);
			// TODO Implement DRAW character where he should be.
			// TODO Implement display message.
			engine.alleg.core.flip_display();
		}
	}
}
