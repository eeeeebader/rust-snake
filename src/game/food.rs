use ratatui::style::Color;
use ratatui::widgets::canvas::{Circle, Context};
use crate::utils::vec2d::Vec2D;
use rand::Rng; // Add 'rand = "0.8"' to Cargo.toml

pub struct Food {
    pub pos: Vec2D,
    pub size: f64,
}

impl Food {
    pub fn new() -> Food {
        // Default starting position
        Food {
            pos: Vec2D::new(150.0, 50.0),
            size: 2.0,
        }
    }

    // Call this when the snake eats the food
    pub fn respawn(&mut self, boundaries: Vec2D) {
        let mut rng = rand::thread_rng();
        self.pos.x = rng.gen_range(10.0..boundaries.x - 10.0);
        self.pos.y = rng.gen_range(10.0..boundaries.y - 10.0);
    }

    pub fn draw(&self, ctx: &mut Context<'_>) {
        ctx.draw(&Circle {
            x: self.pos.x,
            y: self.pos.y,
            radius: self.size,
            color: Color::Yellow,
        });
    }
}