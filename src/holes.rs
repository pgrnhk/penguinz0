use super::*;
use map::*;
use player::*;
use std::collections::HashMap;
pub trait HoleAble {
    fn enter_hole(&mut self);
}
pub struct Hole {
    indexes: ((i32, i32), (i32, i32)),
}
impl Hole {
    pub fn new(indexes: ((i32, i32), (i32, i32))) -> Self {
        Hole { indexes }
    }
    pub fn update(&self, map: &mut HashMap<(i32, i32), Point>) {
        if let Some(point) = map.get_mut(&self.indexes.0) {
            if let State::Player = point.state {
            } else {
                point.state = State::Hole;
            }
        }
        if let Some(point) = map.get_mut(&self.indexes.1) {
            if let State::Player = point.state {
            } else {
                point.state = State::Hole;
            }
        }
    }
    pub fn check_if_player(&mut self, player: &mut Player, map: &mut HashMap<(i32, i32), Point>) {
        if let PlayerState::InHole(index) = player.state {
            if index == self.indexes.0 {
                player.index = self.indexes.1;
                player.state = PlayerState::Normal;
                player.update(map, Some(index));
                self.update(map);
            } else if index == self.indexes.1 {
                player.index = self.indexes.0;
                player.state = PlayerState::Normal;
                player.update(map, Some(index));
                self.update(map);
            }
        }
    }
}
