use crate::prelude::*;

pub mod generators;
pub mod prelude {
    pub use super::generators::hands_on_generator::HandsOnGenerator;
    pub use super::generators::MapGenerator;
    pub use super::*;
}

pub const NUMBER_OF_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
pub const NUMBER_OF_ROOMS: usize = 30;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new<G: MapGenerator>(generator: G) -> Self {
        let (map, rooms, player_start) = generator.build_map();

        MapBuilder {
            map,
            rooms,
            player_start,
        }
    }
}
