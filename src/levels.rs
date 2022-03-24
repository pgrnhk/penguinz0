use super::*;
use breakable_ground::*;
use cannon::*;
use food::Food;
use game_result::GameResult;
use holes::*;
use player::{Direction, Player};
use potions::*;
use std::collections::HashMap;
use std::io;
use void::*;
use walls::*;
pub fn create_level(
    mut player: Player,
    food: Food,
    level_name: &str,
    mut potions: Vec<Potion>,
    void: Vec<Void>,
    cannons: Vec<Cannon>,
    walls: Vec<Wall>,
    mut breakables: Vec<Breakable>,
    mut holes: Vec<Hole>,
) -> GameResult {
    let mut map = HashMap::new();
    map::build_map(&mut map);
    match player.update(&mut map, None) {
        GameResult::Pass => (),
        result => return result,
    }
    food.update(&mut map);
    walls.iter().for_each(|wall| wall.update(&mut map));
    void.iter().for_each(|wall| wall.update(&mut map));
    cannons.iter().for_each(|wall| wall.update(&mut map));
    breakables
        .iter()
        .for_each(|breakable| breakable.update(&mut map));
    potions.iter().for_each(|potion| potion.update(&mut map));
    holes.iter().for_each(|hole| hole.update(&mut map));
    loop {
        for cannon in &cannons {
            match cannon.check_if_player(&mut player, &mut map) {
                GameResult::Pass => (),
                result => return result,
            }
        }
        for breakable in &mut breakables {
            breakable.check_if_player(&mut player);
            breakable.check_if_break();
            breakable.update(&mut map);
        }
        for hole in &mut holes {
            hole.update(&mut map);
            hole.check_if_player(&mut player, &mut map);
        }
        for potion in &mut potions {
            potion.check_if_drinking(&mut player);
        }
        std::process::Command::new("clear").status().unwrap();
        println!("\r{}", level_name);
        println!("Velocity: {}", player.velocity);
        println!("Index: {:?}", player.index);
        println!("Moves: {}", player.moves);
        map::draw_map(&map);
        match player.check_moves() {
            GameResult::Pass => (),
            result => return result,
        }

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        let input = input.trim();
        let direction = match input {
            "w" => Direction::Up,
            "a" => Direction::Left,
            "s" => Direction::Down,
            "d" => Direction::Right,
            "quit" | "exit" => return GameResult::Quit,
            _ => continue,
        };
        match player.travel(&direction, &mut map) {
            GameResult::Pass => (),
            result => return result,
        }
    }
}
