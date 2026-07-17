use crate::game::card::{Card, Suit};
use crate::game::state::GameState;
use crate::tui::entrypoint::Message;
use ratatui::buffer::Buffer;
use ratatui::layout::{Flex, Rect};
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use std::ops::Mul;

pub struct App<'a> {
    pub state: &'a GameState,
    pub available_controls: Vec<Message>,
}

impl Widget for App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [main, footer] =
            Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(area);

        let main_container = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let content_area = main_container.inner(main);

        main_container.render(main, buf);
        GameView { state: self.state }.render(content_area, buf);
        ControlBar {
            available_controls: self.available_controls,
        }
        .render(footer, buf);
    }
}

#[derive(Default)]
struct ControlBar {
    available_controls: Vec<Message>,
}

impl Widget for ControlBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(
            self.available_controls
                .iter()
                .map(|m| format!("{}: {}", m.get_display_text(), m.get_key()))
                .collect::<Vec<String>>()
                .join(" | "),
        )
        .style(Style::default().blue())
        .render(area, buf);
    }
}

struct GameView<'a> {
    state: &'a GameState,
}

impl Widget for GameView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [player_area, dealer_area, stage_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(area);

        HandView {
            cards: self.state.get_player_cards(),
            value: self.state.get_player_value(),
        }
        .render(player_area, buf);
        HandView {
            cards: self.state.get_dealer_cards(),
            value: self.state.get_dealer_value(),
        }
        .render(dealer_area, buf);
        GameStageWidget { state: self.state }.render(stage_area, buf);
    }
}

struct HandView {
    cards: Vec<Card>,
    value: u8,
}

impl Widget for HandView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [cards_area, value_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(5)]).areas(area);
        let c = Layout::horizontal(
            self.cards
                .iter()
                .map(|_| Constraint::Max(5))
                .collect::<Vec<Constraint>>(),
        )
        .split(cards_area);

        c.iter().zip(self.cards).for_each(|(area, card)| {
            CardWidget { card }.render(*area, buf);
        });
        Paragraph::new(self.value.to_string())
            .centered()
            .render(value_area, buf);
    }
}

struct CardWidget {
    card: Card,
}

impl Widget for CardWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [suit_area, rank_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)]).areas(area);
        let style = match self.card.suit {
            Suit::Hearts | Suit::Diamonds => Style::default().red(),
            Suit::Clubs | Suit::Spades => Style::default().blue(),
        };

        Paragraph::new(self.card.suit.to_string())
            .style(style)
            .render(suit_area, buf);
        Paragraph::new(self.card.rank.to_string()).render(rank_area, buf);
    }
}

struct GameStageWidget<'a> {
    state: &'a GameState,
}

impl GameStageWidget<'_> {
    fn get_state_text(&self) -> String {
        match self.state {
            GameState::PlayerTurn { .. } => format!(
                "{} ({}% chance of bust)",
                "Your turn!",
                self.state
                    .get_player_bust_probability()
                    .map(|f| f.mul(100f64) as u64)
                    .unwrap()
            ),
            GameState::DealerTurn { .. } => "Dealer playing...".to_string(),
            GameState::PlayerWin { .. } => "You won!".to_string(),
            GameState::DealerWin { .. } => "You lost.".to_string(),
            GameState::Draw { .. } => "Draw.".to_string(),
        }
    }

    fn get_border_style(&self) -> Style {
        match self.state {
            GameState::PlayerTurn { .. } | GameState::PlayerWin { .. } | GameState::DealerTurn { .. } => Style::default().green(),
            GameState::Draw { .. } => Style::default().yellow(),
            GameState::DealerWin { .. } => Style::default().red(),
        }
    }
}

impl Widget for GameStageWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [block_area] = Layout::vertical([Constraint::Max(5)]).areas(area);
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(self.get_border_style());

        let content_area = block.inner(block_area);

        block.render(block_area, buf);
        Paragraph::new(self.get_state_text())
            .centered()
            .render(content_area, buf);
    }
}
