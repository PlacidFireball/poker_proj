use crate::*;

/*
Quick notes:
rate() assigns a positive integer value to the hand, checks for royal flush,
straight flush, four of a kind, full house, flush, straight, three of a kind,
two pair and one pair. If none of those pass then it assigns the value of the
high card to the hand.
*/

pub fn rate(hand: &mut Hand) -> u32 {
    hand.sort();
    let high_card = hand.cards[4];

    // Preprocessing
    let is_straight = is_straight(hand);
    let same_suit = same_suit(hand);

    if is_straight && same_suit && high_card.num == 14 { // Royal flush
        30
    }
    else if is_straight && same_suit { // Straight Flush
        29
    }
    // TODO impl the rest of the ratings from https://www.cardplayer.com/rules-of-poker/hand-rankings
    high_card.num as u32
}

fn is_straight(hand: &Hand) -> bool {
    for i in 0..4 {
        if hand.cards[i].num != hand.cards[i+1].num-1 {
            false
        }
    }
    true
}

fn is_flush(hand: &Hand) -> bool {

    false
}

fn is_pair(hand: &Hand) -> u8 {

    0
}

fn is_three_of_kind(hand: &Hand) -> bool {

    false
}

fn same_suit(hand: &Hand) -> bool {
    let suit: Face = hand.cards[0].face;
    for card in hand.cards {
        if card.face != suit {
            false
        }
    }
    true
}
