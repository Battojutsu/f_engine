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

use tiled::{Image, Loader, Tileset};

use allegro::*;
//use allegro_image::*;
#[path = "constants.rs"]
mod constants;

pub fn load_map(core: &Core, display: &Display, filename: &str, map: &tiled::Map) -> Bitmap {
    const READONLY_BINARY: &str = "rb";
    let mut loader: Loader = Loader::new();

    let cute_tileset: Tileset =
        match loader.load_tsx_tileset("src/resources/tilesets/house_interior.tsx") {
            Ok(v) => v,
            Err(e) => panic!("Error loading tileset"),
        };

    let image: Image = match cute_tileset.image {
        Some(v) => v,
        None => panic!("Cannot load the image."),
    };

    let map_dir: &str = image.source.to_str().unwrap();
    let tileset_bitmap: Bitmap = match Bitmap::load(&core, map_dir) {
        Ok(v) => v,
        Err(e) => panic!("Error loading tileset_bitmap {e:?}"),
    };

    let map_bitmap = Bitmap::new(&core, constants::WIDTH, constants::HEIGHT);

    tileset_bitmap
}
