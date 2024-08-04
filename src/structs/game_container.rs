/*
    f_engine - A farming and village life game game engine.
    Copyright (C) 2024  BattoJutsu

    Licensed under GNU AGPLv3
*/

use allegro::EventSourceLike;
use tiled::Loader;
use crate::allegro_container;
use crate::player_container;

pub struct GameStructure {
    pub alleg: allegro_container::AllegroStructure,
    pub player: player_container::PlayerStructure,
    pub map: tiled::Map,
}

pub fn game_constructor(filename: &str) -> GameStructure {
    let mut loader: Loader = Loader::new();
    let map: tiled::Map = loader
        .load_tmx_map(filename)
        .unwrap();
    GameStructure {
        alleg: allegro_container::allegro_constructor(&map),
        player: player_container::player_constructor(),
        map: map,
    }
}

impl GameStructure {
    pub fn handle_character_movements(&mut self) {
        match self.alleg.queue.wait_for_event()
        {
            allegro::DisplayClose{source: src, ..} =>
            {
                assert!(self.alleg.display.get_event_source().get_event_source() == src);
                println!("Display close event...");
                std::process::exit(1);
            },
            allegro::KeyDown{keycode: k, ..} if k == allegro::KeyCode::Escape =>
            {
                println!("Pressed Escape!");
                std::process::exit(1);
            },
            allegro::KeyChar{keycode: c, ..} =>
            {
                if c == allegro::KeyCode::W {
                    self.player.PLYR_Y -= 1;
                } 
                else if c == allegro::KeyCode::A {
                    self.player.PLYR_X -= 1;
                }
                else if c == allegro::KeyCode::S {
                    self.player.PLYR_Y += 1;
                } else if c == allegro::KeyCode::D {
                    self.player.PLYR_X += 1;
                }
            },
            _ => println!("Some other event...") 
        }
    }
}