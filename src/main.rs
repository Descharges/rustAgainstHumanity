#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod structures;
use structures::Card;

fn main() {
    println!("Hello World");
    let newCard = Card::white(String::from("Les juifs"));
    newCard.PrOnXt();
}
