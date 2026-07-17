use std::time::Duration;

use crate::game::state::GameState;
use crate::tui::app::App;
use color_eyre::Result;
use color_eyre::eyre::Context;
use crossterm::event::{self, KeyCode};
use ratatui::DefaultTerminal;

pub fn start_app() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(run).context("failed to run app")
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut state = GameState::start();

    loop {
        if let Some(message) = handle_event(&state)? {
            let maybe_new_state = match message {
                Message::Quit => {
                    break;
                }
                Message::NewGame => Ok(GameState::start()),
                Message::Hit => state.player_hit(),
                Message::Stand => state.player_stand(),
            };
            if let Some(new_state) = maybe_new_state.ok() {
                state = new_state;
            }
        }

        match state {
            GameState::DealerTurn { deck: _, player_hand: _, dealer_hand: _ } => {
                if let Some(new_state) = state.execute_dealer_turn().ok() {
                    state = new_state;
                }
            },
            _ => {}
        }

        terminal.draw(|frame| {
            frame.render_widget(App { state: &state, available_controls: get_applicable_messages(&state) }, frame.area());
        })?;
    }
    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Message {
    NewGame,
    Hit,
    Stand,
    Quit,
}

impl Message {
    pub fn get_key(&self) -> &str {
        match self {
            Message::NewGame => "n",
            Message::Hit => "<space>",
            Message::Stand => "s",
            Message::Quit => "q",
        }
    }

    pub fn get_display_text(&self) -> &str {
        match self {
            Message::NewGame => "New game",
            Message::Hit => "Hit",
            Message::Stand => "Stand",
            Message::Quit => "Quit",
        }
    }
}

fn handle_event(state: &GameState) -> Result<Option<Message>> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Some(key) = event::read()
            .context("event read failed")?
            .as_key_press_event()
        {
            return Ok(handle_key(key).filter(|m| get_applicable_messages(state).contains(m)));
        }
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char(' ') => Some(Message::Hit),
        KeyCode::Char('s') => Some(Message::Stand),
        KeyCode::Char('n') => Some(Message::NewGame),
        KeyCode::Esc | KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}

fn get_applicable_messages(state: &GameState) -> Vec<Message> {
    match state {
        GameState::PlayerTurn { .. } => vec![
            Message::NewGame,
            Message::Stand,
            Message::Hit,
            Message::Quit,
        ],
        GameState::DealerTurn { .. }
        | GameState::PlayerWin { .. }
        | GameState::DealerWin { .. }
        | GameState::Draw { .. } => vec![Message::NewGame, Message::Quit],
    }
}
