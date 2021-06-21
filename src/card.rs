/*
suit enum - used to keep track of the suit of a card
--Types--
Ace, Spade, Diamond, Heart, ERROR (for error handling)
--Traits--
PartialEq, Clone, Copy, Debug
--Methods--
as_str(&suit)           - converts a (borrowed) suit to a string
str_to_suit(String)     - converts a string to a suit
 */
use std::fmt::Display;
use std::fmt;
use std::convert::TryFrom;
use core::fmt::Error;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Suit {
    Ace,
    Spade,
    Heart,
    Diamond
}
impl TryFrom<&str> for Suit {
    type Error = core::fmt::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "Ace" { Ok(Suit::Ace) }
        else if value == "Spade" { Ok(Suit::Spade) }
        else if value == "Heart" { Ok(Suit::Heart) }
        else if value == "Diamond" { Ok(Suit::Diamond) }
        else { Err(core::fmt::Error) }
    }
}
impl Suit {
    pub fn as_str(suit: &Suit) -> String {
        match suit {
            Suit::Ace => String::from("Ace"),
            Suit::Spade => String::from("Spade"),
            Suit::Heart => String::from("Heart"),
            Suit::Diamond => String::from("Diamond")
        }
    }
}


/*
The Card struct, the basis for all other structs
in poker_proj.
--Traits--
Clone, Copy, Debug, Default, PartialEq
--Methods--
init(String, u8)    - initializes a card from a string and and 8bit unsigned integer
comp(Card)          - compares one card to another (based on number) returns -1 if less than,
                        0 if equal and 1 if greater
print()             - prints off debug information to the console
*/
#[allow(dead_code, unused_variables)]
#[derive(Clone, Copy, Debug)]
pub struct Card {
    suit: Suit,
    num: u8,
    visible: bool
}
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.suit == other.suit
    }
}
#[allow(dead_code, unused_variables)]
impl Card {
    /*  initializes a card from a string and an 8bit unsigned integer*/
    pub fn init(suit: String, num: u8) -> Self {
        Card { suit: Suit::try_from(suit.as_str()).unwrap(), num, visible: false }
    }

    /*  getter methods */
    pub fn get_rank(&self) -> u8 { return self.num; }
    pub fn get_suit(&self) -> Suit { return self.suit; }

    /*  compares one card to another */
    pub fn comp(&self, other: &Card) -> i8 {
        return if self.num < other.num {
            -1
        } else if self.num == other.num {
            0
        } else {
            1
        }
    }

    /*  prints debug information to the console*/
    pub fn print(&self) {
        println!("struct Card suit: {:?} num: {} visible: {}", &self.suit, &self.num, &self.visible);
    }
}