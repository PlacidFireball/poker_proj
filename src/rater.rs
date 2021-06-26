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
    let mut rating: u16 = 0;

    // Occurrences array
    let mut occurrences: [u8; 14] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for card in &hand.cards {
        occurrences[card.get_rank() as usize] += 1;
    }

    // Preprocessing
    let is_straight = is_straight(occurrences);
    let same_suit = same_suit(hand);
    let is_pair = is_pair(occurrences);
    let most_occ = most_occurrences(occurrences);
    let is_full_house = is_full_house(occurrences);

    if is_straight && same_suit && high_card.get_rank() == 14 {
        rating = 30;
    } else if is_straight && same_suit {
        rating = 29;
    } else if most_occ == 4 {
        rating = 28;
    } else if is_full_house {
        rating = 27;
    } else if same_suit {
        rating = 26;
    } else if is_straight {
        rating = 25;
    } else if most_occ == 3 {
        rating = 24;
    } else if is_pair == 2 {
        rating = 25;
    } else if is_pair == 1 {
        rating = 24;
    } else {
        rating = high_card.get_rank() as u16;
    }

    rating
}

/*  detects if the hand is a straight or not */
fn is_straight(occurence_arr: [u8; 14]) -> bool {
    let mut first_index = 0;
    for i in 0..occurence_arr.len() {
        if occurence_arr[i] == 1 {
            // find the lowest card that occurs
            first_index = i;
            break;
        }
    }
    for j in first_index..first_index + 5 {
        // check the next 4 indices to see if there is a straight
        if occurence_arr[j] != 1 {
            return false;
        }
    }
    true
}

fn is_full_house(occurrences: [u8; 14]) -> bool {
    let mut found_3 = false;
    let mut found_2 = false;
    for i in 0..occurrences.len() {
        if occurrences[i] == 3 {
            found_3 = true;
        } else if occurrences[i] == 2 {
            found_2 = true;
        }
    }
    found_3 && found_2
}

fn is_flush(hand: &Hand) -> bool {
    false
}

/*  detects if there are 1 (or more) pairs in the hand */
fn is_pair(occurrence_arr: [u8; 14]) -> u8 {
    let mut num_pairs = 0;
    for i in 0..occurrence_arr.len() {
        if occurrence_arr[i] == 2 {
            num_pairs += 1;
        }
    }
    num_pairs
}

fn most_occurrences(occurrences: [u8; 14]) -> u8 {
    let mut top: u8 = 0;
    for i in 1..occurrences.len() {
        if occurrences[i] > top {
            top = occurrences[i]
        }
    }
    top
}

fn same_suit(hand: &Hand) -> bool {
    let suit: Suit = hand.cards[0].get_suit();
    for card in hand.cards.iter() {
        if card.get_suit() != suit {
            return false;
        }
    }
    true
}
