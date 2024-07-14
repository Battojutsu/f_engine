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

// This file defines struct PlayerStructure and how to initialize
// one. This file is meant to combine player variables to easily pass between functions.

struct PlayerStructure {
    PLYR_X: i32,
    PLYR_Y: i32,
}

pub fn player_constructor() -> PlayerStructure {
    let player = PlayerStructure {
        PLYR_X: 0,
        PLYR_Y: 0,
    };

    player
}
