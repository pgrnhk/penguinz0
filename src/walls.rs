use super::map::{Point, State};
use std::collections::HashMap;
#[derive(Debug)]
pub enum WallType {
    Small,
    Medium,
    Large,
}
#[derive(Debug)]
pub struct Wall {
    indexes: Vec<(i32, i32)>,
    wall_type: WallType,
}
impl Wall {
    pub fn new(indexes: Vec<(i32, i32)>, wall_type: WallType) -> Self {
        Wall { indexes, wall_type }
    }
    pub fn update<'a>(&'a self, map: &mut HashMap<(i32, i32), Point<'a>>) {
        for index in &self.indexes {
            if let Some(point) = map.get_mut(&index) {
                point.state = State::Wall(&self.wall_type);
            }
        }
    }
}
