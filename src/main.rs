// Code written by Jared Weiss
// Contact: PlacidFireball on github,
// PlacidFireball.dev@gmail.com

mod card;
use crate::card::*;
mod hand;
use crate::hand::*;
mod deck;
use crate::deck::*;
mod player;
mod rater;


fn main() {
    /*  example card code */
    //let my_card = Card::init(String::from("Ace"), 10);
    // my_card.print();

    const BIG_BLIND: u32 = 10;
    const LITTLE_BLIND: u32 = 5;

    let mut deck: Deck = Deck::init();
    let mut hand: Hand = Deck::create_hand(&mut deck);
    hand.print();
    //rater::rate(&hand);

}
