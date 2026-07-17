use crate::game::card::{Card, Deck, FinalisedHand, Hand};

#[derive(Debug, Clone)]
pub enum GameState {
    PlayerTurn {
        deck: Deck,
        player_hand: Hand,
        dealer_hand: Hand,
    },
    DealerTurn {
        deck: Deck,
        player_hand: FinalisedHand,
        dealer_hand: Hand,
    },
    PlayerWin {
        deck: Deck,
        player_hand: FinalisedHand,
        dealer_hand: FinalisedHand,
    },
    DealerWin {
        deck: Deck,
        player_hand: FinalisedHand,
        dealer_hand: FinalisedHand,
    },
    Draw {
        deck: Deck,
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
    fn start_with_deck(deck: &mut Deck) -> Result<Self, TurnTransitionError> {
        let mut player_hand = Hand::default();
        player_hand.add_card(deck.draw_card().ok_or(TurnTransitionError::DeckExhausted)?);
        player_hand.add_card(deck.draw_card().ok_or(TurnTransitionError::DeckExhausted)?);

        let mut dealer_hand = Hand::default();
        dealer_hand.add_card(deck.draw_card().ok_or(TurnTransitionError::DeckExhausted)?);

        if player_hand.is_blackjack() {
            return Ok(GameState::PlayerWin {
                deck: deck.to_owned(),
                player_hand: player_hand.finalise(),
                dealer_hand: dealer_hand.finalise(),
            });
        }

        Ok(GameState::PlayerTurn {
            deck: deck.to_owned(),
            player_hand,
            dealer_hand,
        })
    }

    pub fn start() -> Self {
        let mut deck = Deck::default();
        deck.shuffle();

        Self::start_with_deck(&mut deck).unwrap()
    }

    pub fn restart(&self) -> Result<Self, TurnTransitionError> {
        let mut deck = match self {
            GameState::PlayerTurn { deck, .. }
            | GameState::DealerTurn { deck, .. }
            | GameState::PlayerWin { deck, .. }
            | GameState::DealerWin { deck, .. }
            | GameState::Draw { deck, .. } => deck,
        }
        .to_owned(); // FIXME
        Self::start_with_deck(&mut deck)
    }

    pub fn player_hit(&mut self) -> Result<Self, TurnTransitionError> {
        match self {
            GameState::PlayerTurn {
                deck,
                player_hand,
                dealer_hand,
            } => {
                let card = deck.draw_card().ok_or(TurnTransitionError::DeckExhausted)?;
                player_hand.add_card(card);
                if player_hand.is_bust() {
                    Ok(GameState::DealerWin {
                        deck: deck.clone(),
                        player_hand: player_hand.finalise(),
                        dealer_hand: dealer_hand.finalise(),
                    })
                } else if player_hand.is_blackjack() {
                    Ok(GameState::PlayerWin {
                        deck: deck.clone(),
                        player_hand: player_hand.finalise(),
                        dealer_hand: dealer_hand.finalise(),
                    })
                } else {
                    Ok(GameState::PlayerTurn {
                        deck: deck.to_owned(),
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
                player_hand: player_hand.finalise(),
                dealer_hand: dealer_hand.to_owned(),
            }),
            _ => Err(TurnTransitionError::InvalidGameStateForAction),
        }
    }

    pub fn execute_dealer_turn(&mut self) -> Result<Self, TurnTransitionError> {
        match self {
            GameState::DealerTurn {
                deck,
                player_hand,
                dealer_hand,
            } => {
                if dealer_hand.get_hand_value().lt(&17) {
                    let card = deck.draw_card().ok_or(TurnTransitionError::DeckExhausted)?;
                    dealer_hand.add_card(card);
                }
                if dealer_hand.get_hand_value().lt(&17) {
                    Ok(self.to_owned())
                } else if dealer_hand.is_bust()
                    || dealer_hand.get_hand_value() < player_hand.get_value()
                {
                    Ok(GameState::PlayerWin {
                        deck: deck.clone(),
                        player_hand: player_hand.to_owned(),
                        dealer_hand: dealer_hand.finalise(),
                    })
                } else if dealer_hand.get_hand_value() == player_hand.get_value() {
                    Ok(GameState::Draw {
                        deck: deck.clone(),
                        player_hand: player_hand.to_owned(),
                        dealer_hand: dealer_hand.finalise(),
                    })
                } else {
                    Ok(GameState::DealerWin {
                        deck: deck.clone(),
                        player_hand: player_hand.to_owned(),
                        dealer_hand: dealer_hand.finalise(),
                    })
                }
            }
            _ => Err(TurnTransitionError::InvalidGameStateForAction),
        }
    }

    pub fn get_player_value(&self) -> u8 {
        match self {
            GameState::PlayerTurn { player_hand, .. } => player_hand.get_hand_value(),
            GameState::DealerTurn { player_hand, .. }
            | GameState::PlayerWin { player_hand, .. }
            | GameState::DealerWin { player_hand, .. }
            | GameState::Draw { player_hand, .. } => player_hand.get_value(),
        }
    }

    pub fn get_player_cards(&self) -> Vec<Card> {
        match self {
            GameState::PlayerTurn { player_hand, .. } => player_hand.get_cards().to_vec(),
            GameState::DealerTurn { player_hand, .. }
            | GameState::PlayerWin { player_hand, .. }
            | GameState::DealerWin { player_hand, .. }
            | GameState::Draw { player_hand, .. } => player_hand.get_cards().to_vec(),
        }
    }

    pub fn get_player_bust_probability(&self) -> Option<f64> {
        match self {
            GameState::PlayerTurn {
                deck, player_hand, ..
            } => Some(player_hand.calculate_bust_on_next_card_probability(deck)),
            _ => None,
        }
    }

    pub fn get_dealer_value(&self) -> u8 {
        match self {
            GameState::PlayerTurn { dealer_hand, .. }
            | GameState::DealerTurn { dealer_hand, .. } => dealer_hand.get_hand_value(),
            GameState::Draw { dealer_hand, .. }
            | GameState::DealerWin { dealer_hand, .. }
            | GameState::PlayerWin { dealer_hand, .. } => dealer_hand.get_value(),
        }
    }

    pub fn get_dealer_cards(&self) -> Vec<Card> {
        match self {
            GameState::PlayerTurn { dealer_hand, .. }
            | GameState::DealerTurn { dealer_hand, .. } => dealer_hand.get_cards().to_vec(),
            GameState::PlayerWin { dealer_hand, .. }
            | GameState::DealerWin { dealer_hand, .. }
            | GameState::Draw { dealer_hand, .. } => dealer_hand.get_cards().to_vec(),
        }
    }
}
