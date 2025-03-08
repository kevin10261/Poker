//! A module for creating and managing a deck of cards
//! 
//! This module contains the `Deck` struct, which represents a deck of 52 playing cards. The deck can be shuffled, and cards can be dealt from the top of the deck.
use rand::seq::SliceRandom;
use rand::rng;

#[derive (Debug, Clone)]
pub struct Deck {
    next_card_index: i32,
    cards: Vec<i32>,
}

impl Deck {
    /// Create a new 52-card deck
    pub fn new() -> Deck{
        Deck{next_card_index: 0, cards: (0..52).collect()}        
    }

    /// Shuffle the deck
    pub fn shuffle(&mut self){
        self.cards.shuffle(&mut rng());
        self.next_card_index = 0;
    }

    /// Deal one card from the top of the deck
    /// pub fn deal_one_card(&mut self) -> Option<Card> {
    ///     self.cards.pop()
    /// }
    pub fn deal(&mut self) -> i32{
        let card = self.cards[self.next_card_index as usize];
        self.next_card_index += 1;
        card
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_creation() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);
        assert_eq!(deck.next_card_index, 0);
    }

    #[test]
    fn test_shuffle() {
        let mut deck = Deck::new();
        let original_deck = deck.cards.clone();
        deck.shuffle();
        assert_ne!(deck.cards, original_deck);
        assert_eq!(deck.cards.len(), 52);
        assert_eq!(deck.next_card_index, 0);
    }

    #[test]
    fn test_deal() {
        let mut deck = Deck::new();
        let first_card = deck.cards[0];
        let dealt_card = deck.deal();
        assert_eq!(first_card, dealt_card);
        assert_eq!(deck.next_card_index, 1);
    }

    #[test]
    fn test_deal_all_cards() {
        let mut deck = Deck::new();
        for i in 0..52 {
            let card = deck.deal();
            assert_eq!(card, i);
        }
        assert_eq!(deck.next_card_index, 52);
    }
}

