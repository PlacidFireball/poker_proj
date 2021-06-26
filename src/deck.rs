use crate::card::*;
use crate::hand::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[allow(dead_code, unused_variables)]
#[derive(Clone, Debug)]
pub struct Deck {
    cards: Vec<Card>,
}
#[allow(dead_code, unused_variables)]
impl Deck {
    /*  creates a deck of 52 cards and shuffles it */
    pub fn init() -> Deck {
        let mut suit_vec: Vec<Suit> = Vec::with_capacity(4); // build a vector of the Suits of our cards
        suit_vec.push(Suit::Ace);
        suit_vec.push(Suit::Spade);
        suit_vec.push(Suit::Heart);
        suit_vec.push(Suit::Diamond);
        let mut cards = vec![]; // future cards vec
        for Suit in suit_vec.iter() {
            // iterate through the Suits
            for i in 1..14 {
                // iterate through the cards 1 = ace, 13 = king
                let card = Card::init(Suit::to_string(Suit), i); // make the card
                cards.push(card); // push it onto the vector
            }
        }
        let mut deck = Deck { cards }; // assemble the deck
        deck.shuffle(); // shuffle it
        deck
    }

    /*  makes a new hand from the deck */
    pub fn create_hand(&mut self) -> Hand {
        let mut hand: Vec<Card> = vec![];
        for i in 0..5 {
            hand.push(self.select_card());
        }
        Hand::from(hand)
    }

    /*  returns a card to the deck */
    pub fn return_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    /*  pops a card off the top of the deck */
    pub fn select_card(&mut self) -> Card {
        self.cards.pop().unwrap()
    }

    /*  shuffles the deck using the rand crate */
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    /*  prints off debug information to the console */
    pub fn print(&self) {
        let mut i = 0;
        for card in self.cards.iter() {
            i += 1;
            print!("{} ", i);
            card.print();
        }
    }
}
