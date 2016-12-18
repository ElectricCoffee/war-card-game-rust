pub mod util;
extern crate cards;

use cards::{Deck, Card};
use std::cmp::Ordering;

pub fn play(mut deck1: Deck, mut deck2: Deck) {
    let mut round = 1;
    loop {
        let (maybe_card1, maybe_card2) = (deck1.pop_back(), deck2.pop_back());

        if maybe_card1.is_none() || maybe_card2.is_none() {
            return;
        }

        let (card1, card2) = (maybe_card1.unwrap(), maybe_card2.unwrap());

        println!("Player 1 ({}) plays: {}\nPlayer 2 ({}) plays: {}",
            deck1.len(), card1,
            deck2.len(), card2,
        );

        match card1.cmp(&card2) {
            Ordering::Less => {
                println!("Player 2 wins the round!\n");
                deck2.cards.push_front(card2); // return own card back into deck
                deck2.cards.push_front(card1);
            },
            Ordering::Greater => {
                println!("Player 1 wins the round!\n");
                deck1.cards.push_front(card1);
                deck1.cards.push_front(card2);
            },
            Ordering::Equal =>
                println!("Tied. Discarding cards.\n")
        }
        if round > 10 { return };
        round += 1;
    }
}
