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
    pub redraw: bool,
    pub ticker: u32,
    pub displaying_text: bool
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
        redraw: true,
        ticker: 0,
        displaying_text: false
    }
}

impl GameStructure {
    pub fn handle_events(&mut self) {
        match self.alleg.queue.wait_for_event()
        {
            allegro::TimerTick { .. } => {
                self.redraw = true;
                self.ticker += 1;

                if(self.ticker % 60 == 0) {
                    println!("60 frame tick.")
                }
            },
            allegro::DisplayClose{source: src, ..} =>
            {
                assert!(self.alleg.display.get_event_source().get_event_source() == src);
                println!("Display close event...");
                std::process::exit(1);
            },
            allegro::KeyDown{keycode: k, ..} => {
                if k == allegro::KeyCode::W {
                    self.player.PLYR_Y -= 1;
                } else if k == allegro::KeyCode::A {
                    self.player.PLYR_X -= 1;
                } else if k == allegro::KeyCode::S {
                    self.player.PLYR_Y += 1;
                } else if k == allegro::KeyCode::D {
                    self.player.PLYR_X += 1;
                } else if k == allegro::KeyCode::Enter {
                    if self.displaying_text {
                        self.displaying_text = false;
                    }
                } else if k == allegro::KeyCode::Escape {
                    println!("Pressed Escape!");
                    std::process::exit(1);
                }
            },
            _ => println!("Some other event...") 
        }
    }
}