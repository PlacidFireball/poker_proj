use crate::*;

/*
Quick notes:
rate() assigns a positive integer value to the hand, checks for royal flush,
straight flush, four of a kind, full house, flush, straight, three of a kind,
two pair and one pair. If none of those pass then it assigns the value of the
high card to the hand.
*/

pub fn rate(hand: &mut Hand) -> u16 {
    hand.sort();
    let high_card = hand.cards[4];
    let mut rating : u16 = 0;

    // Preprocessing
    let is_straight = is_straight(hand);
    let same_suit = same_suit(hand);
    let is_pair = is_pair(hand);

    if is_straight && same_suit && high_card.get_rank() == 14 { // Royal flush
        rating = 30;
    }
    else if is_straight && same_suit { // Straight Flush
        rating = 29;
    }
    else if is_pair == 2 {
        rating = 26;
    }
    else if is_pair == 1 {
        rating = 25;
    }
    // TODO impl the rest of the ratings from https://www.cardplayer.com/rules-of-poker/hand-rankings
    rating
}

/*  detects if the hand is a straight or not */
fn is_straight(hand: &Hand) -> bool {
    for i in 0..4 {
        if hand.cards[i].get_rank() != hand.cards[i+1].get_rank()-1 {
            return false;
        }
    }
    true
}

fn is_flush(hand: &Hand) -> bool {

    false
}

/*  detects if there are 1 (or more) pairs in the hand */
fn is_pair(hand: &mut Hand) -> u8 {
    let mut curr_card : Card = hand.cards[0];
    let mut counter : u8 = 0;
    for i in 1..hand.cards.len() {
        if hand.cards[i].get_rank() == curr_card.get_rank() {
            counter += 1;
        }
        curr_card = hand.cards[i];
    }
    counter
}

fn most_occurences(hand: &Hand) -> u8 {
    let mut occurences : [u8; 15] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for card in hand.cards {
        occurences[card.get_rank()] += 1;
    }
    let mut top : u8 = 0;
    for i in 0..occurences.len() {
        if occurences[i] > top {
            top = occurences[i]
        }
    }
    top
}

fn same_suit(hand: &Hand) -> bool {
    let suit: Face = hand.cards[0].get_face();
    for card in hand.cards.iter() {
        if card.get_face() != suit {
            return false;
        }
    }
    true
}
