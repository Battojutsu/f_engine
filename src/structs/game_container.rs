/*
    f_engine - A farming and village life game game engine.
    Copyright (C) 2024  BattoJutsu

    Licensed under GNU AGPLv3
*/

use crate::allegro_container;
use crate::player_container;
use allegro::EventSourceLike;
use tiled::Loader;

/// GameStructure is the main game_engine.
pub struct GameStructure {
    pub alleg: allegro_container::AllegroStructure,
    pub player: player_container::PlayerStructure,
    pub map: Option<tiled::Map>,
    pub redraw: bool,
    pub ticker: u32,
    pub displaying_text: bool
}

/// Implementation of GameStructure methods.
impl GameStructure {
    /// Constructor.
    pub fn new() -> GameStructure {
        GameStructure {
            alleg: allegro_container::allegro_constructor(/*&map*/),
            player: player_container::player_constructor(),
            map: None,
            redraw: true,
            ticker: 0,
            displaying_text: false
        }
    }

    /// load_default_map initializes a map loader then loads the default inside1 test map.
    pub fn load_default_map(&mut self) {
        let mut loader: Loader = Loader::new();
        self.alleg.is_new_map = true;
        self.map = Some(
            loader
                .load_tmx_map("src/resources/maps/inside1.tmx")
                .unwrap(),
        );
    }

    /// Check event queue for a timer, a display close or a keydown. Handle the events here.
    pub fn handle_events(&mut self) {
        match self.alleg.queue.wait_for_event() {
            allegro::TimerTick { .. } => {
                self.redraw = true;
                self.ticker += 1;

                if self.ticker % 60 == 0 {
                    println!("60 frame tick.")
                }
            }
            allegro::DisplayClose { source: src, .. } => {
                assert!(self.alleg.display.get_event_source().get_event_source() == src);
                println!("Display close event...");
                std::process::exit(1);
            }
            allegro::KeyDown { keycode: k, .. } => {
                if k == allegro::KeyCode::W {
                    self.player.plyr_y -= 1;
                } else if k == allegro::KeyCode::A {
                    self.player.plyr_x -= 1;
                } else if k == allegro::KeyCode::S {
                    self.player.plyr_y += 1;
                } else if k == allegro::KeyCode::D {
                    self.player.plyr_x += 1;
                } else if k == allegro::KeyCode::Enter {
                    if self.displaying_text {
                        self.displaying_text = false;
                    }
                } else if k == allegro::KeyCode::Escape {
                    println!("Pressed Escape!");
                    std::process::exit(1);
                }
            }
            _ => println!("Some other event..."),
        }
    }
}
