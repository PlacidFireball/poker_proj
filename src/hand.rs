use crate::card::Card;
use crate::deck::Deck;
use std::convert::From;

/*
The Hand struct, used by players to keep track of cards in their hand, holds 5 cards
--Traits--
Clone, Debug, From<Vec<Card>>
--Methods--
init(&mut [Card])       - initializes a hand from an array of Cards
clear_hand(&mut Deck)   - clears a hand and returns cards to the deck
sort()                  - sorts the hand by the cards' numbers
comp(&Hand)             - compares one hand to another
print()                 - prints debug information to the console
*/
#[allow(dead_code, unused_variables)]
#[derive(Clone, Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}
impl From<Vec<Card>> for Hand {
    fn from(cards: Vec<Card>) -> Hand {
        Hand { cards }
    }
}
#[allow(dead_code, unused_variables)]
impl Hand {
    /*  initializes a hand from an array of Cards
    WILL NOT ACCEPT ANY MORE THAN 5 CARDS */
    pub fn init(cards: &mut [Card]) -> Self {
        let mut cards: Vec<Card> = Vec::new();
        for i in 0..cards.len() {
            if i > 4 {
                break;
            }
            cards.push(cards[i]);
        }
        Hand::from(cards)
    }
    /*  clears a hand and returns cards to the deck */
    pub fn clear_hand(&mut self, deck: &mut Deck) {
        for i in 0..self.cards.len() {
            let card = self.cards.pop().unwrap();
            deck.return_card(card);
        }
    }

    /*  sorts the cards in a hand */
    pub fn sort(&mut self) {
        let mut n = self.cards.len();
        n -= 1;
        for i in 0..n {
            for j in 0..(n - i) {
                if self.cards[j].comp(&self.cards[j + 1]) == 1 {
                    self.cards.swap(j, j + 1);
                }
            }
        }
    }
    /*  prints a hand to the console */
    pub fn print(&self) {
        for card in self.cards.iter() {
            card.print();
        }
    }
}
