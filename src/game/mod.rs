pub mod util;
extern crate cards;

use cards::{Deck, Card};
use std::cmp::Ordering;

fn resolve_buffer(buffer: &mut Vec<Card>, player: &mut Deck) {
    player.cards.extend(buffer.drain(..));
}

fn fill_buffer(buffer: &mut Vec<Card>, c1: Card, c2: Card, p1: &mut Deck, p2: &mut Deck) {
    buffer.push(c1);
    buffer.push(c2);

    let (mc1, mc2) = (p1.pop_back(), p2.pop_back());
    if mc1.is_none() || mc2.is_none() {
        return;
    }

    buffer.push(mc1.unwrap());
    buffer.push(mc2.unwrap());
}

pub fn play(mut deck1: Deck, mut deck2: Deck) {
    let mut tie_buffer = Vec::new();

    while let (Some(card1), Some(card2)) = (deck1.pop_back(), deck2.pop_back()) {
        println!("Player 1 ({}) plays: {}\nPlayer 2 ({}) plays: {}",
            deck1.len(), card1,
            deck2.len(), card2,
        );

        match card1.cmp(&card2) {
            Ordering::Less => {
                println!("Player 2 wins!");
                deck2.cards.push_front(card2); // return own card back into deck
                deck2.cards.push_front(card1);
                resolve_buffer(&mut tie_buffer, &mut deck2);
            },
            Ordering::Greater => {
                println!("Player 1 wins!");
                deck1.cards.push_front(card1);
                deck1.cards.push_front(card2);
                resolve_buffer(&mut tie_buffer, &mut deck1);
            },
            Ordering::Equal => {
                println!("Tied.");
                fill_buffer(&mut tie_buffer, card1, card2, &mut deck1, &mut deck2);
            }
        }
        println!("\nStatus: P1: {}, P2: {}, Buf: {}\n", deck1.len(), deck2.len(), tie_buffer.len());
    }
}
