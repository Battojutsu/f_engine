#[path = "game_loop.rs"] mod game_loop;
#[path = "constants.rs"] mod constants;
#[path = "load_map.rs"] mod lm;
#[path = "structs/player_container.rs"] mod player_container;
#[path = "structs/allegro_container.rs"] mod allegro_container;
#[path = "structs/game_container.rs"] mod game_container;

fn main() {
    game_loop::main_loop();
}
