use crate::game::state::GameState;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

pub struct Home;

impl StatefulWidget for Home {
    type State = GameState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Paragraph::new("Hello world!").render(area, buf);
    }
}
