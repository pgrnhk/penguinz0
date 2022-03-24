use super::game_result::GameResult;
use super::map::{Point, State};
use super::player::{Direction, Player, PlayerState};
use std::collections::HashMap;
pub trait CannonAble {
    fn enter_cannon(&mut self);
}
#[derive(Debug)]
pub enum CannonState {
    Empty,
    Player,
}
#[derive(Debug)]
pub struct Cannon {
    index: (i32, i32),
    state: CannonState,
    distance: i32,
    direction: Direction,
}
impl Cannon {
    pub fn new(distance: i32, index: (i32, i32), direction: Direction) -> Self {
        Cannon {
            index,
            state: CannonState::Empty,
            direction,
            distance,
        }
    }
    pub fn update<'a>(&'a self, map: &mut HashMap<(i32, i32), Point<'a>>) {
        if let Some(point) = map.get_mut(&self.index) {
            point.state = State::Cannon(&self.direction);
        }
    }
    pub fn check_if_player(
        &self,
        player: &mut Player,
        map: &mut HashMap<(i32, i32), Point>,
    ) -> GameResult {
        if let PlayerState::InCannon(index) = player.state {
            if index == self.index {
                match self.shoot_player(player, map) {
                    result => return result,
                    GameResult::Pass => (),
                };
            }
        }
        GameResult::Pass
    }
    pub fn shoot_player(
        &self,
        player: &mut Player,
        map: &mut HashMap<(i32, i32), Point>,
    ) -> GameResult {
        match player.check(map, &self.direction, true, self.distance) {
            GameResult::Lost => return GameResult::Lost,
            _ => (),
        };
        player.move_player(&self.direction, self.distance);
        player.state = PlayerState::Normal;
        match player.update(map, None) {
            result => return result,
            GameResult::Pass => (),
        };
        GameResult::Pass
    }
}
