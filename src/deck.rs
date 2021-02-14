use crate::card::Card;
use crate::card::Face;
use crate::hand::Hand;
use rand::thread_rng;
use rand::seq::SliceRandom;

/*
TODO: add comments for the Deck struct
*/

#[allow(dead_code, unused_variables)]
#[derive(Clone, Debug)]
pub struct Deck {
    cards: Vec<Card>
}
#[allow(dead_code, unused_variables)]
impl Deck {
    /*  creates a deck of 52 cards and shuffles it */
    pub fn init() -> Deck {
        let mut face_vec: Vec<Face> = Vec::with_capacity(4); // build a vector of the faces of our cards
        face_vec.push(Face::Ace);
        face_vec.push(Face::Spade);
        face_vec.push(Face::Heart);
        face_vec.push(Face::Diamond);
        let mut cards = vec!(); // future cards vec
        for face in face_vec.iter() { // iterate through the faces
            for i in 1..14 { // iterate through the cards 1 = ace, 13 = king
                let card = Card::init(Face::to_str(face), i); // make the card
                cards.push(card); // push it onto the vector
            }
        }
        let mut deck = Deck { cards }; // assemble the deck
        deck.shuffle(); // shuffle it
        deck
    }

    pub fn create_hand(&mut self) -> Hand {
        let mut hand: Vec<Card> = vec!();
        for i in 0..5 {
            let card = self.select_card();
            if card == Card::default() {
                panic!("Received a default card!");
            }
            else {
                hand.push(card);
            }
        }
        Hand::from(hand)
    }
    pub fn return_card(&mut self, card: Card) {
        if card == Card::default() {
            panic!("Received a default card!");
        }
        self.cards.push(card);
    }

    pub fn select_card(&mut self) -> Card {
        match self.cards.pop() {
            Some(card) => card,
            None => Card::default()
        }
    }
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
    pub fn print(&self) {
        let mut i = 0;
        for card in self.cards.iter() {
            i+= 1;
            print!("{} ", i);
            card.print();
        }
    }
}