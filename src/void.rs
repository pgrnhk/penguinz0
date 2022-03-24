use super::map::{Point, State};
use std::collections::HashMap;
pub struct Void {
    indexes: Vec<(i32, i32)>,
}
impl Void {
    pub fn new(indexes: Vec<(i32, i32)>) -> Self {
        Void { indexes }
    }
    pub fn update(&self, map: &mut HashMap<(i32, i32), Point>) {
        for index in &self.indexes {
            if let Some(point) = map.get_mut(&index) {
                point.state = State::Void;
            }
        }
    }
}
