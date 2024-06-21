extern crate sdl2;

pub mod game_context;
pub mod renderer;

use std::time::Duration;

use game_context::GameContext;
use renderer::Renderer;
use sdl2::{event::Event::{KeyDown, Quit}, keyboard::Keycode};

const GRID_X_SIZE: i32 = 20;
const GRID_Y_SIZE: i32 = 20;
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
                Quit { .. } => break 'running,
                KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::W => context.move_up(),
                        Keycode::S => context.move_down(),
                        Keycode::A => context.move_left(),
                        Keycode::D => context.move_right(),
                        Keycode::Escape => context.pause_toggle(),
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        context.check_game_over_conditions();

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
