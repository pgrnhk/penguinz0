use super::player::Direction;
use super::walls::WallType;
use std::collections::HashMap;
pub mod sides {
    pub const MIN_Y: i32 = 0;
    pub const MIN_X: i32 = 0;
    pub const MAX_Y: i32 = 10;
    pub const MAX_X: i32 = 10;
}
#[derive(Debug)]
pub enum State<'a> {
    Void,
    Empty,
    Player,
    Food,
    Cannon(&'a Direction),
    Potion,
    Wall(&'a WallType),
    Breakable,
    Hole,
}
#[derive(Debug)]
pub struct Point<'a> {
    pub index: (i32, i32),
    pub state: State<'a>,
}
impl<'a> Point<'a> {
    pub fn new(index: (i32, i32), state: State) -> Point {
        Point { index, state }
    }
}
pub fn draw_map(map: &HashMap<(i32, i32), Point>) {
    for i in 0..=sides::MAX_Y {
        for j in 0..=sides::MAX_Y {
            if let Some(point) = map.get(&(j, i)) {
                let letter = match point.state {
                    State::Empty => "🌿  ",
                    State::Player => "   ",
                    State::Food => "🍔  ",
                    State::Cannon(direction) => match direction {
                        Direction::Up => "   ",
                        Direction::Down => "   ",
                        Direction::Left => "  ",
                        Direction::Right => "   ",
                    },
                    State::Potion => "🧪  ",
                    State::Wall(wall_type) => match wall_type {
                        WallType::Small => "🧱  ",
                        WallType::Medium => "🏛️   ",
                        WallType::Large => "🗼  ",
                    },
                    State::Void => "    ",
                    State::Breakable => "🚧  ",
                    State::Hole => "🕳️   ",
                };
                if point.index.0 == sides::MAX_X {
                    print!("{}\n\n", letter);
                } else {
                    print!("{}", letter);
                }
            }
        }
    }
}
pub fn build_map(hash: &mut HashMap<(i32, i32), Point>) {
    for y in 0..=sides::MAX_Y {
        for x in 0..=sides::MAX_X {
            hash.insert((x, y), Point::new((x, y), State::Empty));
        }
    }
}
