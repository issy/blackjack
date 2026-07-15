use std::cell::RefCell;
use std::sync::Arc;
use crate::card::{Card, Deck, Hand};
use crate::game::GameState::DealerTurn;
use crate::game::TurnTransitionError::InvalidGameStateForAction;

#[derive(Debug, Clone)]
pub enum GameState {
    PlayerTurn {
        deck: Arc<RefCell<Deck>>,
        player_hand: Arc<RefCell<Hand>>,
        dealer_hand: Arc<RefCell<Hand>>,
    },
    DealerTurn {
        deck: Arc<RefCell<Deck>>,
        player_hand: Arc<RefCell<Vec<Card>>>,
        dealer_hand: Arc<RefCell<Hand>>,
    },
    PlayerWin {
        deck: Arc<RefCell<Deck>>,
        player_hand: Arc<RefCell<Vec<Card>>>,
        dealer_hand: Arc<RefCell<Vec<Card>>>,
    },
    DealerWin {
        deck: Arc<RefCell<Deck>>,
        player_hand: Arc<RefCell<Vec<Card>>>,
        dealer_hand: Arc<RefCell<Vec<Card>>>,
    },
}

#[derive(Debug)]
enum TurnTransitionError {
    InvalidGameStateForAction,
    DeckExhausted
}

impl GameState {
    fn start() -> Self {
        let mut deck = Deck::default();
        deck.shuffle();

        let mut player_hand = Hand::default();
        player_hand.add_card(deck.draw_card().unwrap());
        player_hand.add_card(deck.draw_card().unwrap());

        let mut dealer_hand = Hand::default();
        dealer_hand.add_card(deck.draw_card().unwrap());

        GameState::PlayerTurn {
            deck: Arc::new(RefCell::new(deck)),
            player_hand: Arc::new(RefCell::new(player_hand)),
            dealer_hand: Arc::new(RefCell::new(dealer_hand))
        }
    }

    fn player_hit(&mut self) -> Result<Self, TurnTransitionError> {
        match self {
            GameState::PlayerTurn { deck, player_hand, dealer_hand } => {
                let card = deck.borrow_mut().draw_card().ok_or(TurnTransitionError::DeckExhausted)?;
                player_hand.borrow_mut().add_card(card);
                if player_hand.borrow().is_bust() {
                    Ok(GameState::DealerWin {
                        deck: deck.clone(),
                        player_hand: Arc::new(RefCell::new(Vec::from(player_hand.borrow().get_cards()))),
                        dealer_hand: Arc::new(RefCell::new(Vec::from(dealer_hand.borrow().get_cards()))),
                    })
                } else if player_hand.borrow().is_blackjack() {
                    Ok(GameState::PlayerWin {
                        deck: deck.clone(),
                        player_hand: Arc::new(RefCell::new(Vec::from(player_hand.borrow().get_cards()))),
                        dealer_hand: Arc::new(RefCell::new(Vec::from(dealer_hand.borrow().get_cards()))),
                    })
                } else {
                    Ok(GameState::PlayerTurn {
                        deck: deck.clone(),
                        player_hand: player_hand.clone(),
                        dealer_hand: dealer_hand.clone(),
                    })
                }
            }
            _ => Err(InvalidGameStateForAction)
        }
    }

    fn player_stand(&mut self) -> Result<Self, TurnTransitionError> {
        match self {
            GameState::PlayerTurn { deck, player_hand, dealer_hand } => {
                Ok(DealerTurn {
                    deck: deck.clone(),
                    player_hand: Arc::new(RefCell::new(Vec::from(player_hand.borrow().get_cards()))),
                    dealer_hand: dealer_hand.clone(),
                })
            }
            _ => Err(InvalidGameStateForAction)
        }
    }
}
