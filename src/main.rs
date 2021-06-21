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
    /*  example card code */
    //let my_card = Card::init(String::from("Ace"), 10);
    // my_card.print();

    const BIG_BLIND: u32 = 10;
    const LITTLE_BLIND: u32 = 5;

    //let mut deck: Deck = Deck::init();
    //let mut hand: Hand = Deck::create_hand(&mut deck);
    let mut cards : Vec<Card> = vec!();
    cards.push(Card::init(String::from("Heart"), 10));
    cards.push(Card::init(String::from("Heart"), 9));
    cards.push(Card::init(String::from("Heart"), 8));
    cards.push(Card::init(String::from("Heart"), 7));
    cards.push(Card::init(String::from("Heart"), 6));
    let mut hand = Hand::from(cards);
    hand.print();
    println!("{}", rater::rate(&mut hand));

}
