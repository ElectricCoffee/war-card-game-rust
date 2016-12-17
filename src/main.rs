extern crate cards;

fn split<T>(mut left: Vec<T>) -> (Vec<T>, Vec<T>) {
    let half_way = left.len() / 2;
    let right = left.split_off(half_way);
    (left, right)
}

fn main() {
    let mut deck = cards::make_standard_deck();
    cards::shuffle(&mut deck);

    let (p1, p2) = split(deck);
}
