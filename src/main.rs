extern crate cards;

fn main() {
    let mut deck = cards::make_standard_deck();
    cards::shuffle(&mut deck);

}
