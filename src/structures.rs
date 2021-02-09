pub enum Card{
    white(String),
    black(String, i8)
}

impl Card{
    pub fn PrOnXt(self){
        match self{
            Card::white(string) => println!("{}",string),
            Card::black(string,_) => println!("{}",string)
        }
    }
}
