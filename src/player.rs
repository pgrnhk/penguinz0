use super::cannon::CannonAble;
use super::game_result::GameResult;
use super::holes::*;
use super::map::{Point, State};
use super::potions::Drink;
use super::walls::*;
use std::collections::HashMap;
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
pub enum PlayerState {
    Normal,
    InCannon((i32, i32)),
    Drinking((i32, i32)),
    OnBreak((i32, i32)),
    InHole((i32, i32)),
}
#[derive(Debug)]
pub struct Player {
    pub index: (i32, i32),
    pub velocity: i32,
    pub score: i32,
    pub state: PlayerState,
    pub moves: i32,
}
impl Drink for Player {
    fn drink(&mut self) {
        self.state = PlayerState::Drinking(self.index);
    }
}
impl CannonAble for Player {
    fn enter_cannon(&mut self) {
        self.state = PlayerState::InCannon(self.index);
    }
}
impl HoleAble for Player {
    fn enter_hole(&mut self) {
        self.state = PlayerState::InHole(self.index);
    }
}
impl Player {
    pub fn move_player(&mut self, direction: &Direction, distance: i32) {
        match direction {
            Direction::Up => self.index.1 -= distance,
            Direction::Down => self.index.1 += distance,
            Direction::Left => self.index.0 -= distance,
            Direction::Right => self.index.0 += distance,
        }
    }
    pub fn travel(
        &mut self,
        direction: &Direction,
        map: &mut HashMap<(i32, i32), Point>,
    ) -> GameResult {
        let prev_index = self.index;
        match self.check(map, &direction, false, self.velocity) {
            GameResult::Pass => (),
            GameResult::Lost => return GameResult::Lost,
            _ => panic!("Wtf"),
        }

        self.move_player(direction, self.velocity);
        match self.update(map, Some(prev_index)) {
            GameResult::Quit | GameResult::Error => panic!("Not Expected"),
            result => return result,
        }
    }
    pub fn update(
        &mut self,
        map: &mut HashMap<(i32, i32), Point>,
        prev_index: Option<(i32, i32)>,
    ) -> GameResult {
        self.moves -= 1;
        self.remove_prev(map, prev_index);
        match self.add_new(map) {
            GameResult::Quit | GameResult::Error => panic!("Not Expected"),
            result => return result,
        }
    }
    pub fn eat(&self) -> GameResult {
        GameResult::Won
    }
    pub fn check(
        &self,
        map: &mut HashMap<(i32, i32), Point>,
        direction: &Direction,
        cannon: bool,
        distance: i32,
    ) -> GameResult {
        pub fn check(
            map: &mut HashMap<(i32, i32), Point>,
            index: (i32, i32),
            cannon: bool,
        ) -> GameResult {
            if let Some(point) = map.get(&index) {
                if let State::Wall(wall_type) = point.state {
                    match wall_type {
                        WallType::Small => (),
                        WallType::Medium => {
                            if !cannon {
                                return GameResult::Lost;
                            }
                        }
                        WallType::Large => return GameResult::Lost,
                    }
                }
            } else {
                return GameResult::Lost;
            }
            GameResult::Pass
        }
        match direction {
            Direction::Up => {
                for i in self.index.1 - distance..=self.index.1 {
                    match check(map, (self.index.0, i), cannon) {
                        GameResult::Pass => (),
                        GameResult::Lost => return GameResult::Lost,
                        _ => panic!("Wtf"),
                    }
                }
            }
            Direction::Down => {
                for i in self.index.1..=self.index.1 + distance {
                    match check(map, (self.index.0, i), cannon) {
                        GameResult::Pass => (),
                        GameResult::Lost => return GameResult::Lost,
                        _ => panic!("Wtf"),
                    }
                }
            }

            Direction::Left => {
                for i in self.index.0 - distance..self.index.0 {
                    match check(map, (i, self.index.1), cannon) {
                        GameResult::Pass => (),
                        GameResult::Lost => return GameResult::Lost,
                        _ => panic!("Wtf"),
                    }
                }
            }
            Direction::Right => {
                for i in self.index.0..=self.index.0 + distance {
                    match check(map, (i, self.index.1), cannon) {
                        GameResult::Pass => (),
                        GameResult::Lost => return GameResult::Lost,
                        _ => panic!("Wtf"),
                    }
                }
            }
        }
        GameResult::Pass
    }

    pub fn add_new(&mut self, map: &mut HashMap<(i32, i32), Point>) -> GameResult {
        if let Some(point) = map.get_mut(&self.index) {
            match point.state {
                State::Food => return self.eat(),
                State::Cannon(_) => {
                    self.enter_cannon();
                    return GameResult::Pass;
                }
                State::Potion => self.drink(),
                State::Wall(_) | State::Void => return GameResult::Lost,
                State::Breakable => self.state = PlayerState::OnBreak(self.index),
                State::Hole => self.enter_hole(),
                _ => (),
            }
            point.state = State::Player;
        }
        GameResult::Pass
    }
    pub fn remove_prev(
        &self,
        map: &mut HashMap<(i32, i32), Point>,
        prev_index: Option<(i32, i32)>,
    ) {
        if let Some(val) = prev_index {
            if let Some(point) = map.get_mut(&val) {
                match point.state {
                    State::Player | State::Potion => point.state = State::Empty,
                    _ => (),
                }
            }
        }
    }
    pub fn new(moves: i32) -> Player {
        Player {
            index: (0, 0),
            velocity: 1,
            score: 0,
            state: PlayerState::Normal,
            moves: moves + 1,
        }
    }
    pub fn check_moves(&self) -> GameResult {
        if self.moves <= 0 {
            return GameResult::Lost;
        }
        GameResult::Pass
    }
}
