use crate::card::Card;
use crate::deck::Deck;
use std::convert::From;

/*
The Hand struct, used by players to keep track of cards in their hand, holds 5 cards
--Traits--
Clone, Debug, From<Vec<Card>>
--Methods--
init(&mut [Card])       - initializes a hand from an array of Cards
clear_hand(&mut Deck)   - clears a hand and returns cards to the deck
sort()                  - sorts the hand by the cards' numbers
comp(&Hand)             - compares one hand to another
print()                 - prints debug information to the console
*/
#[allow(dead_code, unused_variables)]
#[derive(Clone, Debug)]
pub struct Hand {
    cards: Vec<Card>
}
impl From<Vec<Card>> for Hand {
    fn from(cards: Vec<Card>) -> Hand {
        Hand { cards }
    }
}
#[allow(dead_code, unused_variables)]
impl Hand {
    /*  initializes a hand from an array of Cards
        WILL NOT ACCEPT ANY MORE THAN 5 CARDS */
    pub fn init(cards: &mut [Card]) -> Self {
        let mut cards: Vec<Card> = Vec::new();
        for i in 0..cards.len() {
            if i > 4 {break;}
            cards.push(cards[i]);
        }
        Hand::from(cards)
    }
    /*  clears a hand and returns cards to the deck */
    pub fn clear_hand(&mut self, deck: &mut Deck) {
        for i in 0..self.cards.len() {
            let card = match self.cards.pop() {
                Some(card) => card,
                None => Card::default()
            };
            deck.return_card(card);
        }
    }

    /*  sorts the cards in a hand using bubble sort */
    pub fn sort(&mut self) {
        let mut n = self.cards.len();
        n -= 1;
        for i in 0..n {
            for j in 0..(n-i) {
                if self.cards[j].comp(&self.cards[j+1]) == 1 {
                    self.cards.swap(j, j+1);
                }
            }
        }
    }

    /*  compares one hand to another */
    pub fn comp(&self, other: &Hand) -> i8 {
        // TODO: implement comp()

        /*
        HAND RATINGS
        Hand Rankings
A hand always consist of five cards. Individual cards are "ranked" as follows (high-to-low):
A, K, Q, J, 10, 9, 8, 7, 6, 5, 4, 3, 2. ACE can be low, but only when part of an A-2-3-4-5 straight.
Suits (Club, Diamond, Heart, Spade) have no value, so if two players have hands that are identical
except for suit, then they are tied. A "Kicker" card is a high card used to break ties between hands
of the same rank (ex. 2 players with "Four of a Kind", 3 K's on the board. P1 has K, 9 and P2 has K,
6. P1 with K, 9 wins with the "9 Kicker".)

Here are the "Rank of Hands" in the order of Strength with Probability of being dealt.
Royal Flush- A, K, Q, J, 10, all in the same suit. 1 in 650,000
Straight Flush - Five cards in sequence, all of the same suit. 1in 65,000
Four of a Kind- Four cards of one rank. Kicker breaks ties. 1 in 4,000
Full House- Three matching cards of one rank, plus Two matching cards of another rank.
    Higher ranking set of three wins. If two players have the same set of three, the player
    with the higher pair wins. 1 in 700
Flush- Five cards of the same suit. High card wins. 1 in 500
Straight- Five cards of sequential rank, but different suit. High card wins. 1 in 250
Three of a kind- Three cards of the same rank, plus two unmatched cards. High set wins. 1 in 50
Two Pair- Two cards of the same rank, plus Two cards of another rank. High pair wins. 1 in 20
One Pair- Two cards of the same rank, plus Three unmatched cards. High pair wins. 1 in 2 1/3
High Card- One card high, plus four unmatched lower ranking cards. Ace is the Highest card. Kicker breaks ties. 1 in 1
        */
        let mut self_rating= 0;
        let mut other_rating = 0;




        0
    }

    // algorithm: http://nsayer.blogspot.com/2007/07/algorithm-for-evaluating-poker-hands.html
    fn rate_hand(&mut self) -> String {
        /*  finding the */
        hand.sort(); // sort the hand by rank
        let mut rank_arr = [0, 0, 0, 0, 0]; // histogram of ranks -> can determine types of hands
        let mut counts: [u8; 15] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // counts of each rank
        let mut i = 0;
        for card in self.cards.iter() { // set the rank at the ith index
            rank_arr[i] = card.get_rank();
            i += 1;
        }
        for j in 0..4 {
            if rank_arr[i] == rank_arr[i+1] { // count each rank
                counts[rank_arr[i]] += 1;
            }
        }

        String::from("Hello World!")

    }



    /*  prints a hand to the console */
    pub fn print(&self) {
        for card in self.cards.iter() {
            card.print();
        }
    }
}


