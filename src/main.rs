// Code written by Jared Weiss
// Contact: PlacidFireball on github,
// PlacidFireball.dev@gmail.com

mod card;
//use crate::card::Card;
mod hand;
use crate::hand::Hand;
mod deck;
mod player;

use crate::deck::Deck;

fn main() {
    /*  example card code */
    //let my_card = Card::init(String::from("Ace"), 10);
    // my_card.print();

    let const BIGBLIND: u32 = 10;
    let const LITTLEBLIND: u32 = 5;

    let mut deck: Deck = Deck::init();
    let mut hand: Hand = Deck::create_hand(&mut deck);
    hand.print();
    hand.clear_hand(&mut deck);

    hand.print();
}
