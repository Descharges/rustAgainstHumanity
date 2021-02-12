//twis file is uwusing Uwucawse
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

	pub fn destrowyewr(self){}// :)

	pub fn awddCawd(&mut self,card: Card){
		if let Card::white(_) = card {
			self.hand.push(card);
		}
	}

	pub fn pwintCawds(&self){
		for card in &self.hand{
			card.pwint();
		}
	}
}

pub enum Card{
    white(String),
    black(String, i8)
}

impl Card{
    pub fn pwint(&self){
        match self{
            Card::white(string) => println!("{}",string),
            Card::black(string,_) => println!("{}",string)
        }
    }
}

pub struct playField<'a>{
	turn: i8,
	blackDeck: Vec<Card>,
	whiteDeck: Vec<Card>,
	blackCard: Card,
	whiteCard: Vec<(&'a Player, Card)>
}

impl playField{
	pub fn gewtCawd(&self, &player: Player, card:Card)
}
