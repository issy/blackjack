use std::time::Duration;

use crate::game::state::GameState;
use crate::tui::home::Home;
use color_eyre::eyre::Context;
use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::DefaultTerminal;

pub fn start_app() -> Result<()> {
    color_eyre::install()?; // augment errors / panics with easy to read messages
    ratatui::run(run).context("failed to run app")
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut state = GameState::start();

    loop {
        terminal.draw(|frame| {
            frame.render_stateful_widget(Home, frame.area(), &mut state);
        })?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        let q_pressed = event::read()
            .context("event read failed")?
            .as_key_press_event()
            .is_some_and(|key| key.code == KeyCode::Char('q'));
        return Ok(q_pressed);
    }
    Ok(false)
}
