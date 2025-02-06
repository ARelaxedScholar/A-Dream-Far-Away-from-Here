use std::cmp::{max, min};

use crate::prelude::*;

pub struct HandsOnGenerator;

impl MapGenerator for HandsOnGenerator {
    fn build_map(&self) -> (Map, Vec<Rect>, Point) {
        let mut map = Map::new();

        // Map Generation
        HandsOnGenerator::fill(&mut map, TileType::Wall);
        let mut rooms = HandsOnGenerator::build_rooms(&mut map);
        HandsOnGenerator::build_corridors(&mut map, &mut rooms);
        let player_start = rooms[0].center();

        // Return
        (map, rooms, player_start)
    }
}

impl HandsOnGenerator {
    fn fill(map: &mut Map, tile_type: TileType) {
        map.tiles
            .iter_mut()
            .for_each(|tile| tile.change_to_type(tile_type));
    }

    fn build_rooms(map: &mut Map) -> Vec<Rect> {
        let mut rng = rand::rng();
        let mut rooms = Vec::with_capacity(NUMBER_OF_ROOMS);

        while rooms.len() < NUMBER_OF_ROOMS {
            let room = Rect::with_size(
                rng.random_range(0..SCREEN_WIDTH - 15),
                rng.random_range(0..SCREEN_HEIGHT - 15),
                rng.random_range(2..15),
                rng.random_range(2..15),
            );

            let overlaps = rooms.iter().any(|r| room.intersect(r));

            if !overlaps {
                room.for_each(|point| {
                    if map.in_bounds(point) {
                        let index = Map::index(point.x, point.y);
                        map.tiles[index].change_to_type(TileType::Floor);
                    }
                });
            }

            rooms.push(room);
        }
        rooms
    }

    fn connect_rooms_horizontally(map: &mut Map, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(index) = map.try_index(Point::new(x, y)) {
                map.tiles[index as usize].change_to_type(TileType::Floor);
            }
        }
    }

    fn connect_rooms_vertically(map: &mut Map, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(index) = map.try_index(Point::new(x, y)) {
                map.tiles[index as usize].change_to_type(TileType::Floor);
            }
        }
    }

    fn build_corridors(map: &mut Map, rooms: &[Rect]) {
        let mut rooms: Vec<Rect> = rooms.iter().cloned().collect();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let previous = rooms[i - 1].center();
            let current = room.center();

            let coinflip = || rand::random_bool(0.5);

            let heads = coinflip();

            if heads {
                HandsOnGenerator::connect_rooms_horizontally(
                    map, previous.x, current.x, previous.y,
                );
                HandsOnGenerator::connect_rooms_vertically(map, previous.y, current.y, current.x);
            } else {
                // tails

                HandsOnGenerator::connect_rooms_vertically(map, previous.y, current.y, previous.x);
                HandsOnGenerator::connect_rooms_horizontally(map, previous.x, current.x, current.y);
            }
        }
    }
}
