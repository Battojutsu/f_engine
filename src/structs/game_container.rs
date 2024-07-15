/*
    f_engine - A farming and village life game game engine.
    Copyright (C) 2024  BattoJutsu

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, version 3 of the License

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use tiled::Loader;
#[path = "allegro_container.rs"]
mod allegro_container;
#[path = "player_container.rs"]
mod player_container;

pub struct GameStructure {
    alleg: allegro_container::AllegroStructure,
    player: player_container::PlayerStructure,
    map: tiled::Map,
}

pub fn game_constructor(filename: &str) -> GameStructure {
    let mut loader: Loader = Loader::new();
    let map: tiled::Map = loader
        .load_tmx_map("src/resources/maps/inside1.tmx")
        .unwrap();
    GameStructure {
        alleg: allegro_container::allegro_constructor(&map, filename),
        player: player_container::player_constructor(),
        map: map,
    }
}
