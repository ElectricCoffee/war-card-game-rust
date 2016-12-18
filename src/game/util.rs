extern crate cards;

use cards::Deck;

pub fn split(input: Deck) -> (Deck, Deck) {
    let mut left = input.cards;
    let half_way = left.len() / 2;
    let right = left.split_off(half_way);
    (Deck::from_vec_deque(left), Deck::from_vec_deque(right))
}
