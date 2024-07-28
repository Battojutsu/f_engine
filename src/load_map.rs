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
use std::sync::Arc;

use tiled::{Image, LayerTile, Loader, TileLayer, Tileset};

use allegro::*;
//use allegro_image::*;
#[path = "constants.rs"]
mod constants;

pub fn load_map(core: &Core, map: &tiled::Map) -> Bitmap {
    let tileset_reference: &Arc<Tileset> = match map.tilesets().first() {
        Some(v) => v,
        None => panic!("Error referencing tileset file"),
    };

    let tileset_image = tileset_reference.image.as_ref().unwrap();
    let tileset_filename: &str = tileset_image.source.as_os_str().to_str().unwrap();

    let tileset_bitmap: Bitmap = match Bitmap::load(&core, tileset_filename) {
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

                let sx: u32 = (position % tileset_reference.columns as u32)
                    * tileset_reference.tile_width as u32;
                let sy: u32 = (position
                    / (tileset_reference.tilecount / tileset_reference.columns) as u32)
                    * tileset_reference.tile_height as u32;

                core.draw_bitmap_region(
                    &tileset_bitmap,
                    sx as f32,
                    sy as f32,
                    map.tile_width as f32,
                    map.tile_height as f32,
                    (x * map.tile_width as i32) as f32,
                    (y * map.tile_height as i32) as f32,
                    FLIP_NONE,
                );
            }
            x += 1;
        }
    }
    map_bitmap
}
