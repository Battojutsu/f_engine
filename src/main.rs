#[path = "structs/game_container.rs"]
mod game;

fn main() {
    game::game_constructor("test");
    println!("Hello, world!");
}
