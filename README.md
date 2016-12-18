# War

Hopefully you're familiar with (arguably) the simplest card game ever created: war.

The objective of the game is simple: 

* Two players, each with one half of the deck draw the top card
* The highest numerical value wins, and the played cards are sent to the bottom of the winner's deck
* If a tie occurs, an unknown card from each player is added to a victory pool and a new round begins
  * If this round is decided, the winning player gets all the cards in the pool
  * If it's a tie again, the pool grows as before.
  
This is a simple program, written in Rust, that simulates this game between two virtual players.

This program was made to get to know Rust a little better though a not-so-challenging challenge, 
that explores some of the most basic aspects of the language.
