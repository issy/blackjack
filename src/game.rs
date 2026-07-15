use crate::card::{Deck, FinalisedHand, Hand};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum GameState {
    PlayerTurn {
        deck: Arc<RefCell<Deck>>,
        player_hand: Arc<RefCell<Hand>>,
        dealer_hand: Arc<RefCell<Hand>>,
    },
    DealerTurn {
        deck: Arc<RefCell<Deck>>,
        player_hand: FinalisedHand,
        dealer_hand: Arc<RefCell<Hand>>,
    },
    PlayerWin {
        deck: Arc<RefCell<Deck>>,
        player_hand: FinalisedHand,
        dealer_hand: FinalisedHand,
    },
    DealerWin {
        deck: Arc<RefCell<Deck>>,
        player_hand: FinalisedHand,
        dealer_hand: FinalisedHand,
    },
    Draw {
        deck: Arc<RefCell<Deck>>,
        player_hand: FinalisedHand,
        dealer_hand: FinalisedHand,
    },
}

#[derive(Debug)]
pub enum TurnTransitionError {
    InvalidGameStateForAction,
    DeckExhausted,
}

impl GameState {
    fn start_with_deck(mut deck: Arc<RefCell<Deck>>) -> Result<Self, TurnTransitionError> {
        let mut player_hand = Hand::default();
        player_hand.add_card(deck.borrow_mut().draw_card().ok_or(TurnTransitionError::DeckExhausted)?);
        player_hand.add_card(deck.borrow_mut().draw_card().ok_or(TurnTransitionError::DeckExhausted)?);

        let mut dealer_hand = Hand::default();
        dealer_hand.add_card(deck.borrow_mut().draw_card().ok_or(TurnTransitionError::DeckExhausted)?);

        Ok(GameState::PlayerTurn {
            deck: deck.clone(),
            player_hand: Arc::new(RefCell::new(player_hand)),
            dealer_hand: Arc::new(RefCell::new(dealer_hand)),
        })
    }

    pub fn start() -> Self {
        let mut deck = Arc::new(RefCell::new(Deck::default()));
        deck.borrow_mut().shuffle();

        Self::start_with_deck(deck).unwrap()
    }

    pub fn restart(&self) -> Result<Self, TurnTransitionError> {
        let deck = match self {
            GameState::PlayerTurn { deck, .. } => deck,
            GameState::DealerTurn { deck, .. } => deck,
            GameState::PlayerWin { deck, .. } => deck,
            GameState::DealerWin { deck, .. } => deck,
            GameState::Draw { deck, .. } => deck,
        };
        Self::start_with_deck(deck.clone())
    }

    pub fn player_hit(&mut self) -> Result<Self, TurnTransitionError> {
        match self {
            GameState::PlayerTurn {
                deck,
                player_hand,
                dealer_hand,
            } => {
                let card = deck
                    .borrow_mut()
                    .draw_card()
                    .ok_or(TurnTransitionError::DeckExhausted)?;
                player_hand.borrow_mut().add_card(card);
                if player_hand.borrow().is_bust() {
                    Ok(GameState::DealerWin {
                        deck: deck.clone(),
                        player_hand: player_hand.borrow().finalise(),
                        dealer_hand: dealer_hand.borrow().finalise(),
                    })
                } else if player_hand.borrow().is_blackjack() {
                    Ok(GameState::PlayerWin {
                        deck: deck.clone(),
                        player_hand: player_hand.borrow().finalise(),
                        dealer_hand: dealer_hand.borrow().finalise(),
                    })
                } else {
                    Ok(GameState::PlayerTurn {
                        deck: deck.clone(),
                        player_hand: player_hand.clone(),
                        dealer_hand: dealer_hand.clone(),
                    })
                }
            }
            _ => Err(TurnTransitionError::InvalidGameStateForAction),
        }
    }

    pub fn player_stand(&mut self) -> Result<Self, TurnTransitionError> {
        match self {
            GameState::PlayerTurn {
                deck,
                player_hand,
                dealer_hand,
            } => Ok(GameState::DealerTurn {
                deck: deck.clone(),
                player_hand: player_hand.borrow().finalise(),
                dealer_hand: dealer_hand.clone(),
            }),
            _ => Err(TurnTransitionError::InvalidGameStateForAction),
        }
    }

    pub fn execute_dealer_turn(&mut self) -> Result<Self, TurnTransitionError> {
        // TODO: Refactor to execute turn-by-turn instead of all at once
        match self {
            GameState::DealerTurn {
                deck,
                player_hand,
                dealer_hand,
            } => {
                let mut hand = dealer_hand.borrow_mut();
                while hand.get_hand_value().lt(&17) {
                    let card = deck
                        .borrow_mut()
                        .draw_card()
                        .ok_or(TurnTransitionError::DeckExhausted)?;
                    hand.add_card(card);
                }
                if hand.is_bust() || hand.get_hand_value() < player_hand.get_value() {
                    return Ok(GameState::PlayerWin {
                        deck: deck.clone(),
                        player_hand: player_hand.clone(),
                        dealer_hand: dealer_hand.borrow().finalise(),
                    });
                }
                Ok(GameState::DealerWin {
                    deck: deck.clone(),
                    player_hand: player_hand.clone(),
                    dealer_hand: hand.finalise(),
                })
            }
            _ => Err(TurnTransitionError::InvalidGameStateForAction),
        }
    }
}
