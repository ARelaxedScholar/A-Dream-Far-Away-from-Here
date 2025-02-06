use crate::prelude::*;

pub mod hands_on_generator;
pub trait MapGenerator {
    fn build_map(&self) -> (Map, Vec<Rect>, Point);
}
