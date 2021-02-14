use crate::hand::Hand;

#[derive(PartialEq, Debug, Clone, Copy)]
enum PStatus {
    Check,
    Bet,
    Call,
    Fold,
    Raise,
    RRaise,
    ERROR
}

#[derive(Debug)]
struct Player {
    hand: Hand,
    hand_rating: u32,
    is_turn: bool,
    is_small_blind: bool,
    is_big_blind: bool,
    is_dealer: bool,
    money: i32,
}