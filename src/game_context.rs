use std::{collections::HashSet, ops::Add};

use rand::{seq::SliceRandom, thread_rng};

use crate::{GRID_X_SIZE, GRID_Y_SIZE};

pub enum GameState { Playing, Paused, Over }
pub enum PlayerDirection { Up, Down, Right, Left }

#[derive(Copy, Clone, Eq, Hash)]
pub struct Point(pub i32, pub i32);

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0 && self.1 == other.1
    }
}

pub struct GameContext {
    pub player_position: Vec<Point>,
    pub player_direction: PlayerDirection,
    pub food: Point,
    pub state: GameState,
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            player_position: vec![Point(3, 1), Point(2, 1), Point(1, 1)],
            player_direction: PlayerDirection::Right,
            food: Point(3, 3),
            state: GameState::Playing,
        }
    }

    pub fn update(&mut self) {
        if let GameState::Paused | GameState::Over = self.state {
            return;
        }

        let head_position = self.player_position.first().unwrap();
        let next_head_position = match self.player_direction {
            PlayerDirection::Up => *head_position + Point(0, -1),
            PlayerDirection::Down => *head_position + Point(0, 1),
            PlayerDirection::Right => *head_position + Point(1, 0),
            PlayerDirection::Left => *head_position + Point(-1, 0),
        };

        if self.food != *head_position {
            self.player_position.pop();
        } else {
            self.eat();
        }

        self.player_position.reverse();
        self.player_position.push(next_head_position);
        self.player_position.reverse();
    }

    pub fn check_game_over_conditions(&mut self) {
        self.check_wall_collition();
        self.check_self_collition();
    }

    fn check_wall_collition(&mut self) {
        let head_position = self.player_position.first().unwrap();
        if head_position.0 < 0 || head_position.0 >= GRID_X_SIZE || 
            head_position.1 < 0 || head_position.1 >= GRID_Y_SIZE {

            self.state = GameState::Over;
        }
    }

    fn check_self_collition(&mut self) {
        let head_position = self.player_position.first().unwrap();
        for point in self.player_position.iter().skip(1) {
            if head_position == point {
                self.state = GameState::Over;
            }
        }
    }

    fn eat(&mut self) {
        // Unavailable positions.
        let mut set = HashSet::<&Point>::new();
        for point in self.player_position.iter() {
            set.insert(point);
        }

        // Possible food positions
        let mut possible_positions: Vec<Point> = Vec::new();
        for i in 0..GRID_X_SIZE {
            for j in 0..GRID_Y_SIZE {
                let point = Point(i, j);
                if !set.contains(&point) {
                    possible_positions.push(point);
                }
            }
        }

        // Get the new_food_position
        let mut rng = thread_rng();
        let new_food_position = possible_positions.choose(&mut rng);


        if let Some(food_position) = new_food_position {
            self.food = *food_position;
        } else {
            self.state = GameState::Over;
        }
    }

    pub fn move_up(&mut self) {
        if let PlayerDirection::Down = self.player_direction {
            return;
        }
        self.player_direction = PlayerDirection::Up;
    }

    pub fn move_down(&mut self) {
        if let PlayerDirection::Up = self.player_direction {
            return;
        }
        self.player_direction = PlayerDirection::Down;
    }

    pub fn move_left(&mut self) {
        if let PlayerDirection::Right = self.player_direction {
            return;
        }
        self.player_direction = PlayerDirection::Left;
    }

    pub fn move_right(&mut self) {
        if let PlayerDirection::Left = self.player_direction {
            return;
        }
        self.player_direction = PlayerDirection::Right;
    }

    pub fn pause_toggle(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
            GameState::Over => todo!("Handle over state"),
        };
    }
}
