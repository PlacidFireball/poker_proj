// Code written by Jared Weiss
// Contact: PlacidFireball on github,
// PlacidFireball.dev@gmail.com

mod card;
use crate::card::*;
mod hand;
use crate::hand::*;
mod deck;
use crate::deck::*;
mod rater;

fn main() {
    const BIG_BLIND: u32 = 10;
    const LITTLE_BLIND: u32 = 5;

    test();
}

fn test() {
    /* tests for poker_proj */

    let mut cards: Vec<Card> = vec![];
    cards.push(Card::init(String::from("Heart"), 10));
    cards.push(Card::init(String::from("Heart"), 9));
    cards.push(Card::init(String::from("Heart"), 8));
    cards.push(Card::init(String::from("Heart"), 7));
    cards.push(Card::init(String::from("Heart"), 6));
    let mut hand = Hand::from(cards);
    hand.print();
    println!("{}", rater::rate(&mut hand));
}
