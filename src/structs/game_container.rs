//#include "../libraries/cute_headers/cute_tiled.h"
//#include "../load_map.c"
//#include "allegro_container.c"
//#include "player_container.c"

use tiled::Loader;
/*
struct GameStructure {
    alleg: AllegroStructure,
    player: PlayerStructure,
    map: Map,
}

    */

pub fn game_constructor(filename: &str) /*-> GameStructure */
{
    //cute_tiled_map_t *map = cute_tiled_load_map_from_file(filename, NULL);

    let mut loader: Loader = Loader::new();
    let map = loader.load_tmx_map("src/resources/tests.tmx").unwrap();

    /*
    let game_structure: GameStructure = {
        0,
        0,
        map
    }

    */
    /*
    struct game_structure game =
        {
            allegro_constructor(map, filename),
            player_constructor(),
            map};

    return game;

    */
}
//#endif
