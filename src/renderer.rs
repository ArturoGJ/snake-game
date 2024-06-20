use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas, video::Window};

use crate::{game_context::{GameContext, Point}, DOT_SIZE_IN_PXS};

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

        self.canvas.set_draw_color(Color::RED);
        self.draw_dot(&game_context.food)?;

        for point in game_context.player_position.iter() {
            self.canvas.set_draw_color(Color::GREEN);
            self.draw_dot(point)?;
        }

        self.canvas.present();

        Ok(())
    }
}
