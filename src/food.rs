use super::map;
use super::map::Point;
use std::collections::HashMap;
pub struct Food {
    index: (i32, i32),
}
impl Food {
    pub fn new(index: (i32, i32)) -> Food {
        Food { index }
    }
    pub fn update(&self, map: &mut HashMap<(i32, i32), Point>) {
        if let Some(point) = map.get_mut(&self.index) {
            point.state = map::State::Food;
        }
    }
}
