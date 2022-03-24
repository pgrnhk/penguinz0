mod breakable_ground;
mod cannon;
mod food;
mod game_result;
mod holes;
mod levels;
mod map;
mod player;
mod potions;
mod void;
mod walls;
use cannon::*;
use food::*;
use game_result::GameResult;
use player::*;
use potions::*;
use void::*;
use walls::*;
fn main() {
    let mut exit_status = GameResult::Pass;
    loop {
        match levels::create_level(
            Player::new(5),
            Food::new((5, 0)),
            "Level 0",
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ) {
            GameResult::Error => panic!("Wtf"),
            GameResult::Lost => continue,
            GameResult::Won => exit_status = GameResult::Won,
            GameResult::Quit => {
                exit_status = GameResult::Quit;
                break;
            }
            GameResult::Pass => panic!("Not Expected"),
        }
        match levels::create_level(
            Player::new(6),
            Food::new((8, 4)),
            "Level 1",
            vec![Potion::new(|player| player.velocity += 1, (0, 2))],
            vec![],
            vec![Cannon::new(2, (8, 2), Direction::Down)],
            vec![],
            vec![],
            vec![],
        ) {
            GameResult::Error => panic!("Wtf"),
            GameResult::Lost => continue,
            GameResult::Won => exit_status = GameResult::Won,
            GameResult::Quit => {
                exit_status = GameResult::Quit;
                break;
            }
            GameResult::Pass => (),
        }

        match levels::create_level(
            Player::new(12),
            Food::new((8, 4)),
            "Level 2",
            vec![Potion::new(|player| player.moves -= 5, (2, 4))],
            vec![Void::new(vec![
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 0),
                (7, 0),
                (8, 0),
                (9, 4),
                (9, 5),
                (8, 5),
                (7, 5),
                (7, 4),
                (7, 3),
                (8, 3),
                (9, 3),
                (1, 0),
                (1, 1),
                (2, 1),
                (3, 1),
                (2, 2),
                (3, 2),
                (1, 2),
                (1, 3),
                (2, 3),
                (3, 3),
                (0, 9),
                (0, 10),
                (0, 8),
                (1, 8),
                (2, 8),
                (3, 8),
                (4, 8),
                (5, 8),
                (6, 8),
                (7, 8),
                (8, 8),
                (9, 8),
                (10, 8),
                (0, 9),
                (1, 9),
                (2, 9),
                (3, 9),
                (4, 9),
                (5, 9),
                (6, 9),
                (7, 9),
                (8, 9),
                (9, 9),
                (10, 9),
                (0, 10),
                (1, 10),
                (2, 10),
                (3, 10),
                (4, 10),
                (5, 10),
                (6, 10),
                (7, 10),
                (8, 10),
                (9, 10),
                (10, 10),
            ])],
            vec![
                Cannon::new(2, (4, 2), Direction::Right),
                Cannon::new(2, (6, 2), Direction::Down),
                Cannon::new(2, (6, 4), Direction::Right),
                Cannon::new(2, (5, 4), Direction::Right),
            ],
            vec![Wall::new(vec![(5, 5)], WallType::Small)],
            vec![],
            vec![],
        ) {
            GameResult::Error => panic!("Wtf"),
            GameResult::Lost => continue,
            GameResult::Won => {
                exit_status = GameResult::Won;
            }
            GameResult::Quit => {
                exit_status = GameResult::Quit;
                break;
            }
            GameResult::Pass => (),
        }
        break;
    }
    std::process::Command::new("clear").status().unwrap();
    if let GameResult::Won = exit_status {
        println!("Congratulations! You beat my game!");
    }
}
