// Code written by Jared Weiss
// Contact: PlacidFireball on github,
// PlacidFireball.dev@gmail.com

mod card;
use crate::card::Card;
mod hand;
mod deck;

fn main() {
    let my_card = Card::init(String::from("Ace"), 10);
    my_card.print();
}
