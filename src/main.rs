// Code written by Jared Weiss
// Contact: PlacidFireball on github,
// PlacidFireball.dev@gmail.com

mod card;
//use crate::card::Card;
mod hand;
use crate::hand::Hand;
mod deck;
use crate::deck::Deck;

fn main() {
    /*  example card code */
    //let my_card = Card::init(String::from("Ace"), 10);
    // my_card.print();

    let mut deck: Deck = Deck::init();
    deck.print();
    let mut hand: Hand = Deck::create_hand(&mut deck);
    hand.print();
    hand.sort();

    deck.print();
    hand.clear_hand(&mut deck);
    deck.print();
    hand.print();
}
