
use crate::card::Card;
use std::convert::From;

#[allow(dead_code, unused_variables)]
#[derive(Clone, Debug)]
pub struct Hand {
    card_vec: Vec<Card>
}
impl From<Vec<Card>> for Hand {
    fn from(card_vec: Vec<Card>) -> Self {
        Hand { card_vec }
    }
}
#[allow(dead_code, unused_variables)]
impl Hand {
    /*  initializes a hand from an array of Cards
        WILL NOT ACCEPT ANY MORE THAN 5 CARDS */
    pub fn init(cards: &mut [Card]) -> Self {
        let mut card_vec: Vec<Card> = Vec::new();
        for i in 0..cards.len() {
            if i > 4 {break;}
            card_vec.push(cards[i]);
        }
        Hand::from(card_vec)
    }
    /*  clears a hand */
    pub fn clear_hand(mut self) -> Self {
        self.card_vec = Vec::new();
        self
    }

    pub fn sort(mut self) {
        let mut n = self.card_vec.len();
        n -= 2;
        for i in 0..n {
            for j in 0..(n-i) {
                if self.card_vec[j].comp(&self.card_vec[j+1]) == -1 {
                    self.card_vec.swap(j, j+1);
                }
            }
        }
    }
    /*  compares one hand to another */
    pub fn comp(other: &Hand) -> i8 {
        0
    }
}


