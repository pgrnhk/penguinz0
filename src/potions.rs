use super::map::{Point, State};
use super::player::{Player, PlayerState};
use std::collections::HashMap;
pub trait Drink {
    fn drink(&mut self);
}
pub struct Potion {
    function: fn(&mut Player),
    index: (i32, i32),
}
impl Potion {
    pub fn new(function: fn(&mut Player), index: (i32, i32)) -> Self {
        Potion { function, index }
    }
    pub fn update(&self, map: &mut HashMap<(i32, i32), Point>) {
        if let Some(point) = map.get_mut(&self.index) {
            point.state = State::Potion;
        }
    }
    pub fn check_if_drinking(&self, player: &mut Player) {
        if let PlayerState::Drinking(index) = player.state {
            if index == self.index {
                (self.function)(player);
                player.state = PlayerState::Normal;
            }
        }
    }
}
