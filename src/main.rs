mod game;
extern crate cards;

fn main() {
    let deck = cards::Deck::make_standard().shuffled();
    let foo = game::util::split(deck);
    game::play(foo.0, foo.1);
}
