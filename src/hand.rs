
use crate::card::Card;

#[allow(dead_code, unused_variables)]
#[derive(Clone, Debug)]
pub struct Hand {
    card_vec: Vec<Card>
}
#[allow(dead_code, unused_variables)]
impl Hand {
    fn init_frm_vec(cards: Vec<Card>) -> Hand {
        Hand { card_vec: cards }
    }
    fn init_frm_cards(cards: &mut [Card]) -> Hand {
        unimplemented!()
    }
    fn comp(other: &Hand) -> bool {
        // returns true if the current hand is better than other
        return false;
    }
}
