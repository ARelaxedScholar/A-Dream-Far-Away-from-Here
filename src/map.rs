use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
}

impl TileType {
    pub fn change_to_type(&mut self, tile_type: TileType) {
        *self = tile_type
    }
}
pub struct Map {
    pub tiles: Vec<TileType>,
}

// Class Methods
impl Map {
    pub fn new() -> Self {
        Map {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn index(x: i32, y: i32) -> usize {
        ((y * SCREEN_WIDTH) + x) as usize
    }

    pub fn xy_from_index(index: usize) -> (i32, i32) {
        let x = index % (SCREEN_WIDTH as usize);
        let y = index / (SCREEN_WIDTH as usize);
        unimplemented!();
    }
}

// Methods
impl Map {
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = Map::index(x, y);
                match self.tiles[index] {
                    TileType::Floor => ctx.set(x, y, YELLOW, BLACK, to_cp437('.')),
                    TileType::Wall => ctx.set(x, y, GREEN, BLACK, to_cp437('#')),
                }
            }
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        let in_bounds_horizontally = 0 <= point.x && point.x < SCREEN_WIDTH;
        let in_bounds_vertically = 0 <= point.y && point.y < SCREEN_HEIGHT;

        in_bounds_horizontally && in_bounds_vertically
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[Map::index(point.x, point.y)] == TileType::Floor
    }

    pub fn try_index(&self, point: Point) -> Option<usize> {
        if self.in_bounds(point) {
            Some(Map::index(point.x, point.y))
        } else {
            None
        }
    }
}
