use std::ops;

use ratatui::crossterm::cursor::MoveRight;

#[derive(Copy, Clone)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64
}

impl Vec2D {
    pub fn new(x: f64, y: f64) -> Vec2D {
        Vec2D {
            x: x,
            y: y,
        }
    }

    pub fn len(&self) -> f64 {
        return (self.x * self.x + self.y * self.y).sqrt();
    }

    pub fn dist(a: Vec2D, b: Vec2D) -> f64 {
        return ((b.x - a.x).powf(2.0) + (b.y - a.y).powf(2.0)).sqrt();
    }

    pub fn norm(&mut self) -> Vec2D{
        let len = self.len();
        self.x /= len;
        self.y /= len;

        Vec2D { x: self.x / len, y: self.y / len }
    }
}

impl ops::Sub<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: Vec2D) -> Self::Output {
        Vec2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Add<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Vec2D) -> Self::Output {
        Vec2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Mul<Vec2D> for Vec2D {
    type Output = f64;

    fn mul(self, rhs: Vec2D) -> Self::Output {
        return self.x * rhs.x + self.y * rhs.y;
    }
}