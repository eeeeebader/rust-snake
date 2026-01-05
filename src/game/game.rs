use std::time::Duration;
use rand::Rng;

use ratatui::{crossterm::event::KeyCode, widgets::{Block, Widget, canvas::Canvas}};

use crate::{game::{food::Food, snake::{LoseCondition, Snake}}, utils::vec2d::Vec2D};


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameDifficulty {
    Easy = 0,
    Normal = 1,
    Hard = 2,
}

impl GameDifficulty {
    pub fn to_idx(&self) -> usize {
        *self as usize
    }

    pub fn from_idx(idx: usize) -> Self {
        match idx {
            0 => GameDifficulty::Easy,
            1 => GameDifficulty::Normal,
            2 => GameDifficulty::Hard,
            _ => GameDifficulty::Easy,
        }
    }

    pub fn size() -> usize {
        return 3;
    }

    pub fn increase(&mut self) {
        let next_idx = (self.to_idx() + 1) % GameDifficulty::size();
        *self = Self::from_idx(next_idx);
    }

    pub fn decrease(&mut self) {
        let prev_idx = self.to_idx().saturating_sub(1);
        *self = Self::from_idx(prev_idx);
    }
}

pub enum GameState {
    Playing, 
    GameOver
}

pub struct Game {
    pub game_state: GameState,
    pub difficulty: GameDifficulty,
    pub snake: Snake,
    pub food: Food,
    pub score: usize,
    pub screen_size: Vec2D
}

impl Game {
    pub fn new(difficulty: GameDifficulty, screen_size: Vec2D) -> Game {

        Game {
            game_state: GameState::Playing,
            difficulty: difficulty, 
            snake: Snake::new(difficulty, screen_size),
            food: Food::new(),
            score: 0,
            screen_size: screen_size,
        }
    }

    pub fn reset(&mut self) {
        self.snake.reset();
        self.game_state = GameState::Playing;
        self.score = 0;
        self.food.pos = Vec2D::new(70.0, 70.0);
    }

    pub fn draw(&self) -> impl Widget + '_ {

        Canvas::default()
            .block(Block::bordered())
            .x_bounds([0.0, self.screen_size.x]) 
            .y_bounds([0.0, self.screen_size.y])
            .paint(|ctx| {
                // 1. Draw Snake
                let food_dist = Vec2D::dist(self.snake.pos, self.food.pos);

                self.snake.draw(ctx);
                
                // 2. Draw Food
                self.food.draw(ctx);
                
                // 3. You can still draw UI elements directly
                ctx.print(5.0, self.screen_size.y-2.0, format!("Score: {}", self.score));
                ctx.print(self.screen_size.x - 30.0, self.screen_size.y-2.0, format!("GameOver: {}", matches!(self.game_state, GameState::GameOver)));
                ctx.print(self.screen_size.x - 30.0, self.screen_size.y-5.0, format!("Food Dist: {:.2}", food_dist));
            })
    }

    pub fn handle_event(&mut self, code: KeyCode) {
        self.snake.handle_event(code);
    }

    pub fn update(&mut self, delat_time: Duration) {
        if matches!(self.game_state, GameState::GameOver) {
            return;
        }

        self.snake.update(delat_time);

        if !matches!(self.snake.lose_condition, LoseCondition::None) {
            self.game_state = GameState::GameOver;
        }

        self.check_snake_food_coll();

    }

    fn check_snake_food_coll(&mut self) {
        let mut rng = rand::thread_rng();
        if Vec2D::dist(self.snake.pos, self.food.pos) - 5.0 < self.food.size {
            self.score += 1;
            self.food.pos = Vec2D::new(rng.gen_range(0..(self.screen_size.x as usize)) as f64, rng.gen_range(0..(self.screen_size.y as usize)) as f64);
            self.snake.snake_len += 5.0 + (self.difficulty as usize as f64) * 3.0;
            // increasing the snake_speed with respect tot he difficulty. The introduction of the damping keeps the game playable in late game scenarios
            self.snake.snake_speed += (1.25 * (self.difficulty as usize + 1) as f64) / (self.snake.snake_speed - 32.0 - (self.difficulty as usize) as f64 * 10.0).max(1.0);
        }
    }

    pub fn increase_difficulty(&mut self) {
        self.difficulty.increase();
        self.snake.difficulty = self.difficulty;
    }

    pub fn decrease_difficulty(&mut self) {
        self.difficulty.decrease();
        self.snake.difficulty = self.difficulty;
    }
}