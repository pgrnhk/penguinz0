use super::*;
use map::*;
use player::*;
use std::collections::HashMap;

pub enum BreakState {
    Solid,
    Void,
}
pub struct Breakable {
    index: (i32, i32),
    break_counter: i32,
    state: BreakState,
    on_player: bool,
}
impl Breakable {
    pub fn new(
        index: (i32, i32),
        mut break_counter: i32,
        state: BreakState,
        on_player: bool,
    ) -> Self {
        if !on_player {
            break_counter = break_counter + 1;
        }
        Breakable {
            state,
            index,
            break_counter,
            on_player,
        }
    }
    pub fn update(&self, map: &mut HashMap<(i32, i32), Point>) {
        if let Some(point) = map.get_mut(&self.index) {
            if let State::Player = point.state {
            } else {
                if let BreakState::Solid = self.state {
                    point.state = State::Breakable;
                } else {
                    point.state = State::Void;
                }
            }
        }
    }
    pub fn check_if_break(&mut self) {
        if self.break_counter <= 0 {
            self.break_ground();
        }
    }
    pub fn break_ground(&mut self) {
        self.state = BreakState::Void;
    }
    pub fn check_if_player(&mut self, player: &mut Player) {
        if self.on_player {
            if let PlayerState::OnBreak(index) = player.state {
                if index == self.index {
                    self.break_counter -= 1;
                }
            }
        } else {
            self.break_counter -= 1;
        }
    }
}
