use std::time::Duration;

use crate::game::game::GameDifficulty;

use crate::utils::vec2d::Vec2D;

use ratatui::crossterm::event::KeyCode;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Circle, Context, Line};

pub enum LoseCondition {
    None,
    Border,
    SelfIntersect,
}

#[derive(Clone, Copy)]
enum MovementDir {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,
}

impl MovementDir {
    pub fn from_vec2d(d: &Vec2D) -> MovementDir {
        if d.x == 0.0 && d.y == 1.0 {
            return MovementDir::Up;
        } else if d.x == 0.0 && d.y == -1.0 {
            return MovementDir::Down;
        } else if d.x == 1.0 && d.y == 0.0 {
            return MovementDir::Left;
        } else if d.x == -1.0 && d.y == 0.0 {
            return MovementDir::Right;
        }

        return MovementDir::Up;
    }

    pub fn to_vec2d(&self) -> Vec2D {
        match *self {
            MovementDir::Up => Vec2D { x: 0.0, y: 1.0 },
            MovementDir::Down => Vec2D { x: 0.0, y: -1.0 },
            MovementDir::Left => Vec2D { x: -1.0, y: 0.0 },
            MovementDir::Right => Vec2D { x: 1.0, y: 0.0 },
        }
    }

    pub fn as_idx(&self) -> usize {
        *self as usize
    }

    pub fn opposing_directions(a: MovementDir, b: MovementDir) -> bool {
        return a.as_idx() % 2 == b.as_idx() % 2;
    }
}

pub struct Snake {
    pub difficulty: GameDifficulty,
    pub pos: Vec2D,
    pub dir: Vec2D,
    pub snake_speed: f64,
    pub lose_condition: LoseCondition,
    pub snake_len: f64,
    boundaries: Vec2D,
    corners: Vec<Vec2D>,
}

impl Snake {
    pub fn new(difficulty: GameDifficulty, boundaries: Vec2D) -> Snake {
        Snake {
            difficulty: difficulty,
            pos: Vec2D::new(50.0, 50.0),
            dir: Vec2D::new(1.0, 0.0),
            snake_speed: 25.0,
            snake_len: 0.0,
            lose_condition: LoseCondition::None,
            boundaries: boundaries,
            corners: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.pos = Vec2D::new(50.0, 50.0);
        self.dir = Vec2D::new(1.0, 0.0);
        self.snake_speed = 25.0;
        self.snake_len = 0.0;
        self.corners = Vec::new();
        self.lose_condition = LoseCondition::None;
    }

    pub fn draw(&self, ctx: &mut Context<'_>) {
        ctx.draw(&Circle {
            x: self.pos.x,
            y: self.pos.y,
            radius: 1.0,
            color: Color::Green,
        });

        let mut prev_pos = &self.pos;
        for corner in self.corners.iter() {
            ctx.draw(&Line {
                x1: prev_pos.x,
                y1: prev_pos.y,
                x2: corner.x,
                y2: corner.y,
                color: Color::Green,
            });

            prev_pos = corner;
        }

        ctx.print(5.0, 0.0, format!("x: {}, y: {}", self.dir.x, self.dir.y));
        ctx.print(27.0, 0.0, format!("speed: {:.2}", self.snake_speed));
        ctx.print(
            54.0,
            0.0,
            format!("Snake Len: {:.2}", self.calc_snake_length()),
        );
    }

    pub fn handle_event(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('w') => {
                self.change_movement_dir(MovementDir::Up);
            }
            KeyCode::Char('s') => {
                self.change_movement_dir(MovementDir::Down);
            }
            KeyCode::Char('a') => {
                self.change_movement_dir(MovementDir::Left);
            }
            KeyCode::Char('d') => {
                self.change_movement_dir(MovementDir::Right);
            }
            _ => {}
        }
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.move_snake(delta_time);
        self.check_border();
        self.snake_self_intersect();
    }

    fn change_movement_dir(&mut self, m_dir: MovementDir) {
        let current_m_dir: MovementDir = MovementDir::from_vec2d(&self.dir);

        // early exit if the directions are opposing
        if MovementDir::opposing_directions(current_m_dir, m_dir) {
            return;
        }

        self.dir = m_dir.to_vec2d();
        self.corners.insert(0, self.pos.clone());
    }

    fn move_snake(&mut self, delta_time: Duration) {
        self.pos.x += self.snake_speed * delta_time.as_secs_f64() * self.dir.x;
        self.pos.y += self.snake_speed * delta_time.as_secs_f64() * self.dir.y;

        let mut current_len = self.calc_snake_length();

        while current_len > self.snake_len {
            let excess = current_len - self.snake_len;

            if self.corners.is_empty() {
                break;
            }

            let last_idx = self.corners.len() - 1;
            let follow_target = if last_idx == 0 {
                self.pos
            } else {
                self.corners[last_idx - 1]
            };

            let last_corner = &mut self.corners[last_idx];
            let dist_to_target = Vec2D::dist(*last_corner, follow_target);

            if dist_to_target <= excess {
                self.corners.pop();
                current_len -= dist_to_target;
            } else {
                let dir_to_target = (follow_target - *last_corner).norm();
                last_corner.x += dir_to_target.x * excess;
                last_corner.y += dir_to_target.y * excess;
                break;
            }
        }
    }

    fn check_border(&mut self) {
        if matches!(self.difficulty, GameDifficulty::Easy) {
            self.clip_border();
            return;
        }

        if self.pos.x < 0.0
            || self.pos.x > self.boundaries.x
            || self.pos.y < 0.0
            || self.pos.y > self.boundaries.y
        {
            self.lose_condition = LoseCondition::Border;
        }
    }

    fn snake_self_intersect(&mut self) {
        if self.corners.len() <= 0 {
            return;
        }
        let first_ele = self.corners[0];

        for i in 1..self.corners.len() - 1 {
            if intersect(self.pos, first_ele, self.corners[i], self.corners[i + 1]) {
                self.lose_condition = LoseCondition::SelfIntersect;
            }
        }
    }

    fn clip_border(&mut self) {
        if !matches!(self.difficulty, GameDifficulty::Easy) {
            return;
        }

        if self.pos.x < 0.0 {
            self.pos.x = self.boundaries.x;
        } else if self.pos.x > self.boundaries.x {
            self.pos.x = 0.0;
        }

        if self.pos.y < 0.0 {
            self.pos.y = self.boundaries.y;
        } else if self.pos.y > self.boundaries.y {
            self.pos.y = 0.0;
        }
    }

    fn calc_snake_length(&self) -> f64 {
        let mut len: f64 = 0.0;

        let mut prev_point = &self.pos;
        for corner in self.corners.iter() {
            len += Vec2D::dist(*prev_point, *corner);

            prev_point = corner;
        }

        len
    }
}

fn ccw(a: Vec2D, b: Vec2D, c: Vec2D) -> bool {
    return (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x);
}

fn intersect(a: Vec2D, b: Vec2D, c: Vec2D, d: Vec2D) -> bool {
    return ccw(a, c, d) != ccw(b, c, d) && ccw(a, b, c) != ccw(a, b, d);
}
