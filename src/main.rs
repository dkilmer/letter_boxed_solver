use std::io::{self, BufReader};
use std::io::prelude::*;
use std::env;
use std::fs;
use std::fmt::{self, Display};
use std::time::Instant;
use std::process;

struct Word {
	strip: String,
	start: u8,
	end: u8,
	mask: u32
}

impl Display for Word {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {}", self.strip, self.mask)
	}
}

fn make_word(bytes: &[u8]) -> Word {
	let mut msk = 0u32;
	let strp : Vec<u8> = bytes.iter().filter(|&c| (*c >= 65u8 && *c <= 90u8) || (*c >= 97u8 && *c <= 122u8))
		.map(|c| {
			if *c >= 97u8 {
				msk = msk | (1 << (*c - 97u8) as u32);
				*c - 32u8
			} else {
				msk = msk | (1 << (*c - 65u8) as u32);
				*c
			}
	}).collect();
	let sb : u8 = strp[0];
	let eb : u8 = strp[strp.len()-1];
	let s = String::from_utf8(strp).unwrap();
	Word {strip: s, start: sb, end: eb, mask: msk}
}

fn no_spaces(bytes: &[u8]) -> bool {
	for c in bytes.iter() {
		if *c == b' ' {
			return false;
		}
	}
	true
}

fn valid_word(bytes: &[u8]) -> bool {
	if bytes.len() < 3 {
		false
	} else {
		no_spaces(bytes)
	}
}

fn get_side(byte: &u8, sides: &Vec<Word>) -> i32 {
	for (i, side) in sides.iter().enumerate() {
		for b in side.strip.as_bytes().iter() {
			if *byte == *b {
				return i as i32;
			}
		}
	}
	-1
}

fn no_repeated_sides(bytes: &[u8], sides: &Vec<Word>) -> bool {
	let mut last_side: i32 = -1;
	for b in bytes.iter() {
		let this_side = get_side(b, sides);
		if this_side == last_side {
			return false
		}
		last_side = this_side;
	}
	true
}

fn has_repeating_letters(w: &Word) -> bool {
	let bytes = w.strip.as_bytes();
	for i in 0..bytes.len() {
		for j in i+1..bytes.len() {
			if bytes[i] == bytes[j] {
				return true
			}
		}
	}
	false
}

fn main() -> io::Result<()> {
	let args: Vec<String> = env::args().collect();
	if args.len() < 3 {
		println!("usage: letter_boxed_solver /path/to/dict.txt RKM,UIC,PHG,NAY");
		process::exit(1);
	}
	let filename = &args[1];
	let puzzle = &args[2];

	// turn the puzzle into a Vex of Words
	let sides: Vec<Word> = puzzle.split(',').map(|s| make_word(s.as_bytes())).collect();
	if sides.len() < 3 {
		println!("The puzzle must have at least three sides. Use letters separated by commas.");
		process::exit(1);
	}
	// turn the whole puzzle into a Word
	let whole = make_word(puzzle.as_bytes());
	if has_repeating_letters(&whole) {
		println!("The puzzle must not have any repeating letters.");
		process::exit(1);		
	}

	let now = Instant::now();

	let mut words : Vec<Word> = Vec::new();
	let f = fs::File::open(filename)?;
	let f = BufReader::new(f);
	// read the file line by line. assumes one entry per line
	for line in f.lines() {
		let l = line.unwrap();
		let bytes = l.as_bytes();
		// only accept words 3 bytes or longer with no spaces
		if valid_word(bytes) {
			let w = make_word(bytes);
			// make sure the word only has characters that are in the puzzle
			if (w.mask & whole.mask) == w.mask && w.strip.len() > 2 {
				// make sure no adjacent letters in the word fall on the same side of the puzzle
				if no_repeated_sides(&w.strip.as_bytes(), &sides) {
					words.push(w);
				}
			}
		}
	}
	println!("solve puzzle {} using word list {} ({} valid words)", puzzle, filename, words.len());
	// try all the two-word combinations
	for w1 in words.iter() {
		for w2 in words.iter() {
			if w1.end == w2.start && ((w1.mask | w2.mask) & whole.mask) == whole.mask {
				println!("{} {}", w1.strip, w2.strip);
			}
		}
	}
	println!("solved in {}ms", now.elapsed().as_millis());

	Ok(())
}

// cargo run ./words.txt AKZ,JTI,PCD,ORE