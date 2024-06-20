use std::ops::Add;

use rand::Rng;

use crate::{GRID_X_SIZE, GRID_Y_SIZE};

pub enum GameState { Playing, Paused }
pub enum PlayerDirection { Up, Down, Right, Left }

#[derive(Copy, Clone)]
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
        if let GameState::Paused = self.state {
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

    fn eat(&mut self) {
        let x = rand::thread_rng().gen_range(0..GRID_X_SIZE);
        let y = rand::thread_rng().gen_range(0..GRID_Y_SIZE);
        self.food = Point(x, y);
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
        };
    }
}