extern crate sdl2;

use std::{ops::Add, time::Duration};

use sdl2::{
    event::Event::{KeyDown, Quit}, keyboard::Keycode, pixels::Color, rect::Rect, render::WindowCanvas, video::Window
};

const GRID_X_SIZE: i32 = 40;
const GRID_Y_SIZE: i32 = 30;
const DOT_SIZE_IN_PXS: i32 = 20;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Snake", 
            (GRID_X_SIZE * DOT_SIZE_IN_PXS) as u32, 
            (GRID_Y_SIZE * DOT_SIZE_IN_PXS) as u32
        )
        .position_centered()
        .opengl() // Sets the window to be usable with an OpenGL context
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut context = GameContext::new();
    let mut renderer = Renderer::new(window)?;
    let mut frame_count = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Quit { .. }
                | KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        frame_count += 1;
        if frame_count % 10 == 0 {
            context.update();
            frame_count = 0;
        }

        let _ = renderer.draw(&context);

        // We wait until the next loop, in this case it means we wait one
        // thirtieth of a second for each frame, which means that we process
        // at most, 30 frames per second.
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}

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
        let head_position = self.player_position.first().unwrap();
        let next_head_position = match self.player_direction {
            PlayerDirection::Up => *head_position + Point(0, 1),
            PlayerDirection::Down => *head_position + Point(0, -1),
            PlayerDirection::Right => *head_position + Point(1, 0),
            PlayerDirection::Left => *head_position + Point(-1, 0),
        };
        
        self.player_position.pop();
        self.player_position.reverse();
        self.player_position.push(next_head_position);
        self.player_position.reverse();
    }
}

pub struct Renderer { canvas: WindowCanvas }

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
        let Point(x, y) = point;
        self.canvas.fill_rect(Rect::new(
            x * DOT_SIZE_IN_PXS,
            y * DOT_SIZE_IN_PXS,
            DOT_SIZE_IN_PXS as u32,
            DOT_SIZE_IN_PXS as u32
        ))?;

        Ok(())
    }

    pub fn draw(&mut self, game_context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        for point in game_context.player_position.iter() {
            self.canvas.set_draw_color(Color::GREEN);
            self.draw_dot(point)?;
        }

        self.canvas.set_draw_color(Color::RED);
        self.draw_dot(&game_context.food)?;

        self.canvas.present();

        Ok(())
    }
}
