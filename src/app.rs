// ANCHOR: all
use ratatui::crossterm::event::KeyCode;
use std::collections::HashMap;

use crate::{
    game::game::{Game, GameDifficulty},
    utils::vec2d::Vec2D,
};

// ANCHOR: screen_modes
pub enum CurrentScreen {
    Home,
    Play,
}

pub enum HomeSelection {
    StartGame,
}

pub struct App {
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub current_home_selection: HomeSelection,
    pub game: Game,
}
// ANCHOR_END: app_fields

// ANCHOR: impl_new
impl App {
    pub fn new() -> App {
        App {
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Home,
            current_home_selection: HomeSelection::StartGame,
            game: Game::new(GameDifficulty::Normal, Vec2D::new(150.0, 75.0)),
        }
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{output}");
        Ok(())
    }

    pub fn handle_event(&mut self, key: KeyCode) -> bool {
        match self.current_screen {
            CurrentScreen::Home => match key {
                KeyCode::Char('q') | KeyCode::Esc => {
                    return true;
                }
                KeyCode::Char('a') => {
                    self.game.decrease_difficulty();
                }
                KeyCode::Char('d') => {
                    self.game.increase_difficulty();
                }
                KeyCode::Enter => {
                    self.game.reset();
                    self.current_screen = CurrentScreen::Play;
                }
                _ => {}
            },
            CurrentScreen::Play => match key {
                KeyCode::Esc => {
                    self.current_screen = CurrentScreen::Home;
                    self.current_home_selection = HomeSelection::StartGame;
                }
                _ => {
                    self.game.handle_event(key);
                }
            },
        }

        false
    }
}
