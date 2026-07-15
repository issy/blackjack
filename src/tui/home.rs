use crate::game::state::GameState;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

pub struct Home;

impl StatefulWidget for Home {
    type State = GameState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [main, footer] =
            Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(area);

        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).render(main, buf);
        ControlBar::new(false).render(footer, buf);
    }
}

#[derive(Default)]
struct ControlBar {
    game_in_progress: bool,
}

impl ControlBar {
    fn new(game_in_progress: bool) -> Self {
        ControlBar { game_in_progress }
    }
}

impl Widget for ControlBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let constraints: [_; 5] = Layout::horizontal([Constraint::Fill(1); 5])
            .areas(area);
        for a in constraints.iter() {
            Paragraph::new("Foo")
                .style(Style::default().blue())
                .render(*a, buf);
        }
    }
}
