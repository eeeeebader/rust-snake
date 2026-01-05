use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Clear},
    Frame,
};

use crate::{app::{App, CurrentScreen}, game::game::GameDifficulty};

pub fn ui(f: &mut Frame, app: &App) {
    // Create the main layout areas
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    
    match app.current_screen {
        CurrentScreen::Play => {
            f.render_widget(app.game.draw(), chunks[0]);
        },
        _ => {}
    }

    let status_text = match app.current_screen {
        CurrentScreen::Home => " [Q] Quit | [Enter] Start Game | [A/D] Difficulty ",
        CurrentScreen::Play => " [Esc] Menu | [W/A/S/D] Move ",
        _ => "",
    };

    let status_bar = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL).title(" Controls "));
    
    f.render_widget(status_bar, chunks[1]);

    // 3. Overlay for Menu or Game Over
    if let CurrentScreen::Home = app.current_screen {
        render_menu(f, app);
    }
}

fn render_menu(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 40, f.size());
    f.render_widget(Clear, area);

    let menu_block = Block::default()
        .title(" RUST SNAKE ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let diff_text = match app.game.difficulty {
        GameDifficulty::Easy => "Easy",
        GameDifficulty::Normal => "Normal",
        GameDifficulty::Hard => "Hard",
    };
        
    
    let text = vec![
        Line::from(""),
        Line::from(diff_text),
        Line::from(""),
        Line::from("Press ENTER to start"),
    ];

    let paragraph = Paragraph::new(text)
        .block(menu_block)
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(paragraph, area);
}

// Helper function to create a centered window for the menu
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}