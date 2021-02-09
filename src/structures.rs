
pub struct Player{
	id :i8,
	name: String,
	hand: Vec<Card>,
	score : i8
}

impl Player{
    pub fn new(id:i8, name: String) -> Player {
        Player{
            id,
            name,
            hand: Vec::new(),
            score: 0,
        }
    }
}

pub enum Card{
    white(String),
    black(String, i8)
}

impl Card{
    pub fn PrOnXt(&self){
        match self{
            Card::white(string) => println!("{}",string),
            Card::black(string,_) => println!("{}",string)
        }
    }
}
