use crate::card::Card;
use crate::card::Face;

#[allow(dead_code, unused_variables)]
#[derive(Clone, Debug)]
pub struct Deck {
    cards: Vec<Card>
}
#[allow(dead_code, unused_variables)]
impl Deck {
    pub fn init() -> Deck {
        let mut face_vec: Vec<Face> = Vec::with_capacity(4);
        face_vec.push(Face::Ace);
        face_vec.push(Face::Spade);
        face_vec.push(Face::Heart);
        face_vec.push(Face::Diamond);
        let mut cards = vec!();
        for face in face_vec.iter() {
            for i in 1..14 {
                let card = Card::init(Face::to_str(face), i);
                cards.push(card);
            }
        }
        Deck { cards }
    }
    pub fn print(&self) {
        for card in self.cards.iter() {
            card.print();
        }
    }
    pub fn select_card(&mut self) {

    }
}