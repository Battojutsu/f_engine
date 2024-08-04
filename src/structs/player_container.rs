/*
    f_engine - A farming and village life game game engine.
    Copyright (C) 2024  BattoJutsu

    Licensed under AGPLV3
*/

// This file defines struct PlayerStructure and how to initialize
// one. This file is meant to combine player variables to easily pass between functions.

pub struct PlayerStructure {
    pub PLYR_X: i32,
    pub PLYR_Y: i32,
}

pub fn player_constructor() -> PlayerStructure {
    let player = PlayerStructure {
        PLYR_X: 0,
        PLYR_Y: 0,
    };

    player
}
