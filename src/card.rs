use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let character = match self {
            Suit::Hearts => "♥️",
            Suit::Diamonds => "♦️",
            Suit::Clubs => "♣️",
            Suit::Spades => "♠️",
        };
        write!(f, "{}", character)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn get_value(self) -> (u8, Option<u8>) {
        match self {
            Rank::Ace => (1, Some(11)),
            Rank::Two => (2, None),
            Rank::Three => (3, None),
            Rank::Four => (4, None),
            Rank::Five => (5, None),
            Rank::Six => (6, None),
            Rank::Seven => (7, None),
            Rank::Eight => (8, None),
            Rank::Nine => (9, None),
            Rank::Ten => (10, None),
            Rank::Jack => (10, None),
            Rank::Queen => (10, None),
            Rank::King => (10, None),
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let character = match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        };
        write!(f, "{}", character)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

impl Card {
    pub fn get_value(&self) -> (u8, Option<u8>) {
        self.rank.get_value()
    }
}

#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards = Vec::new();
        for suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in &[
                Rank::Ace,
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
            ] {
                cards.push(Card {
                    suit: *suit,
                    rank: *rank,
                });
            }
        }
        Deck { cards }
    }
}

impl Deck {
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut ThreadRng::default());
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn get_cards(&self) -> &[Card] {
        &self.cards
    }

    fn get_all_possible_hand_values(&self) -> Vec<u8> {
        let mut all_hand_permutations = Vec::<Vec<u8>>::new();
        all_hand_permutations.push(Vec::new());
        for card in &self.cards {
            let (value, alt_value) = card.get_value();

            if let Some(alt) = alt_value {
                let mut new_permutations = all_hand_permutations.clone();

                for p in &mut all_hand_permutations {
                    p.push(value);
                }

                for p in &mut new_permutations {
                    p.push(alt);
                }
                all_hand_permutations.extend(new_permutations);
            } else {
                for p in &mut all_hand_permutations {
                    p.push(value);
                }
            }
        }

        all_hand_permutations
            .into_iter()
            .map(|hand| hand.into_iter().sum())
            .collect::<Vec<u8>>()
    }

    pub fn get_hand_value(&self) -> u8 {
        let all_possible_hand_values = self.get_all_possible_hand_values();
        all_possible_hand_values
            .iter()
            .filter(|v| v.le(&&21))
            .max()
            .unwrap_or_else(|| all_possible_hand_values.iter().min().unwrap())
            .clone()
    }

    pub fn is_bust(&self) -> bool {
        self.get_hand_value() > 21
    }

    pub fn is_blackjack(&self) -> bool {
        self.get_hand_value() == 21
    }

    pub fn calculate_bust_on_next_card_probability(&self, deck: &Deck) -> f64 {
        deck.cards.iter().map(|card| {
            let mut hand_clone = self.clone();
            hand_clone.add_card(*card);
            if hand_clone.is_bust() {
                1.0
            } else {
                0.0
            }
        }).sum::<f64>() / deck.cards.len() as f64
    }

    pub fn finalise(&self) -> FinalisedHand {
        FinalisedHand {
            cards: self.cards.clone(),
            value: self.get_hand_value()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinalisedHand {
    cards: Vec<Card>,
    value: u8
}

impl FinalisedHand {
    pub fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }
}
