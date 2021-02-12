#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod structures;
use structures::Card;
use structures::Player;

fn main() {
    let mut p1 = Player::new(1,String::from("Paul"));
    let newCard = Card::white(String::from("Les juifs"));
    p1.awddCawd(newCard);
    let newCard = Card::white(String::from("Hitler"));
    p1.awddCawd(newCard);
    let newCard = Card::white(String::from("Ma bite"));
    p1.awddCawd(newCard);
    p1.pwintCawds();
    p1.destrowyewr();
}
