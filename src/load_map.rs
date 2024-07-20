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

use std::ops::Deref;

use tiled::{Image, Loader, TileLayer, Tileset, LayerTile};

use allegro::*;
//use allegro_image::*;
#[path = "constants.rs"]
mod constants;

pub fn load_map(core: &Core, filename: &str, map: &tiled::Map) -> Bitmap {
    let mut loader: Loader = Loader::new();

    let tiled_tileset: Tileset =
        match loader.load_tsx_tileset(filename) {
            Ok(v) => v,
            Err(e) => panic!("Error loading tileset {e}."),
        };
    
    let image: Image = match tiled_tileset.image {
        Some(v) => v,
        None => panic!("Cannot load the image."),
    };

    let map_dir: &str = image.source.to_str().unwrap();
    let tileset_bitmap: Bitmap = match Bitmap::load(&core, map_dir) {
        Ok(v) => v,
        Err(e) => panic!("Error loading tileset_bitmap {e:?}"),
    };

    let map_bitmap = Bitmap::new(&core, constants::WIDTH, constants::HEIGHT).unwrap();
    let layers = map.layers();
    core.set_target_bitmap(Some(&map_bitmap));
    
    for layer in layers {
        let tile_layer: TileLayer = layer.as_tile_layer().unwrap();

        let width: i32 = tile_layer.width().unwrap() as i32;
        let height: i32 = tile_layer.height().unwrap() as i32;
        let mut x: i32 = 0;

        while x < width {
            let mut y: i32 = 0;
            while y < height {
                let tile: LayerTile = tile_layer.get_tile(x, y).unwrap();
                y += 1;

                let position: u32 = tile.deref().id();

                let sx:u32 = (position % width as u32) * map.tile_width as u32;
                let sy:u32 = (position / height as u32) * map.tile_height;
                                
                core.draw_bitmap_region(&tileset_bitmap,
                    sx as f32,
                    sy as f32,
                    map.tile_width as f32,
                    map.tile_height as f32,
                    (x * map.tile_width as i32) as f32,
                    (y * map.tile_height as i32) as f32,
                FLIP_NONE);
            }
            x += 1;
        }
    }
    map_bitmap
}
