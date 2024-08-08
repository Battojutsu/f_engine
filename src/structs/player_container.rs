/*
    f_engine - A farming and village life game game engine.
    Copyright (C) 2024  BattoJutsu

    Licensed under GNU AGPLv3
*/

// This file defines struct PlayerStructure and how to initialize
// one. This file is meant to combine player variables to easily pass between functions.

pub struct PlayerStructure {
    pub plyr_x: i32,
    pub plyr_y: i32,
}

pub fn player_constructor() -> PlayerStructure {
    let player = PlayerStructure {
        plyr_x: 0,
        plyr_y: 0,
    };

    player
}
