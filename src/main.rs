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

fn main() {
    /*  example card code */
    // let my_card = Card::init(String::from("Ace"), 10);
    // my_card.print();

    /*  constants */
    const BIGBLIND: u32 = 10;
    const LITTLEBLIND: u32 = 5;

    /*  driver code */
    let mut deck: Deck = Deck::init();
    let mut hand: Hand = deck.create_hand();
    hand.print();
    hand.clear_hand(&mut deck);
}
