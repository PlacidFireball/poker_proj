use crate::hand::Hand;
use crate::card::Card;

#[derive(PartialEq, Debug, Clone, Copy)]
enum PStatus {
    Check,
    Bet,
    Call,
    Fold,
    Raise,
    RRaise,
    Default,
    ERROR
}

#[derive(Debug)]
struct Player {
    hand: Hand,
    status: PStatus,
    hand_rating: u32,
    is_turn: bool,
    is_small_blind: bool,
    is_big_blind: bool,
    is_dealer: bool,
    money: i32,
}
impl Default for Player {
    fn default() -> Player {
        let vec: Vec<Card> = vec!();
        let hand = Hand::from(vec);
        Player {
            hand,
            status: PStatus::Default,
            hand_rating: 0,
            is_turn: false,
            is_small_blind: false,
            is_big_blind: false,
            is_dealer: false,
            money: 0,
        }
    }
}

impl Player {
    /*  initializes a new player */
    pub fn init(hand: Hand, hand_rating: u32, money: i32) -> Player {
        let mut player = Player::default();
        player.hand = hand;
        player.hand_rating = hand_rating;
        player.money = money;
        player
    }

    /*  setter methods */
    pub fn set_turn(&mut self, is_turn: bool) { self.is_turn = is_turn; }
    pub fn set_small_blind(&mut self, is_small_blind: bool) { self.is_small_blind = is_small_blind; }
    pub fn set_big_blind(&mut self, is_big_blind: bool) { self.is_big_blind = is_big_blind; }
    pub fn set_dealer(&mut self, is_dealer: bool) { self.is_dealer = is_dealer; }




}