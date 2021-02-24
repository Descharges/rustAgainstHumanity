//twis file is uwusing Uwucase
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

mod structures as struc;
use struc::Card;
use struc::playField;

pub fn gewtcontewnt (input : str, playfield : &mut playField) {
	let f = File::open(input)
        .expect("Something went wong weading the file");
    let f = BufReader::new(f);
	let white:bool = true;
	for line in f.lines(){
		println(line.unwarp());
	}
}
