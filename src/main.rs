#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod structures;
use structures::Card;
use structures::Player;

fn main() {
    println!("Hello World");
    let newCard = Card::white(String::from("Les juifs"));
    newCard.PrOnXt();
    let p1 = Player::new(1,String::from("Paul"));
    let truc: i16 = 0xFFF;
}
