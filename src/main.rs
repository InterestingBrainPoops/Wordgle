use std::fs;
use std::io;
use std::vec::Vec;

#[derive(Debug)]
struct Letter {
	letter: String,
	garbage: Vec<String>,
	confirmed: bool
}

impl Letter {
	fn create_struct(character: &str) -> Letter {
		Letter {
			letter: String::from(character),
			garbage: Vec::new(),
			confirmed: false
		}
	}
}

/** Identifier:
* 2: right word right place
* 1: right word wrong place
* 0: wrong word wrong place
*/

/*
	In Final Check:

	check if green characters are the way they are
	check if garbage array not matching
	check if word contains possible characters
*/

fn parse_results(result: String, word: &mut [Letter; 5], green_characters: &mut Vec<String>, possible_characters: &mut Vec<String>, banned_characters: &mut Vec<String>, are_green: &mut bool, are_yellow: &mut bool) {
	let split = result.trim().split("-");

	let result_vec: Vec<&str> = split.collect();
	for i in 0..result_vec.len() {
		match result_vec[i].parse::<u8>().unwrap() {
			2 => {
				green_characters.push(word[i].letter.clone());
				word[i].confirmed = true;
				*are_green = true;
			}
			1 => {
				possible_characters.push(word[i].letter.clone());
				word[i].garbage.push(word[i].letter.clone());
				*are_yellow = true;
			}
			0 => {
				banned_characters.push(word[i].letter.clone());
			}
			_ => unreachable!()
		}
	}
}

fn parse_garbage(banned_characters: &mut Vec<String>, word_vec: &mut Vec<&str>, i: usize, no_banned_chars: &mut bool) {
	for o in 0..banned_characters.len() {
		//WORD CONTAINS BANNED CHARACTER, DEFINITE FALSE
		if word_vec[i].contains(&banned_characters[o]) {
			*no_banned_chars = false;
			break;
		} else {
			*no_banned_chars = true;
		}
	}
}

//DONT FORGET TO CHANGE &STR TO STRING WHEN CHECKING IN DICTIONARY

fn main() {
	let version = 6;
	println!("VERSION {}", version);
	println!("Thank you for choosing Wordgle! It's very simple to use. Seperate the state of characters in the word using -s! Represent green characters with 2, yellow characters with 1, and grey characters with 0.");
	println!("Example: 2-1-0-1-2");
	println!("Next, sit back and relax :). Wordgle's got your back.");
	println!("Made by DkeRee");

	let mut first_round = true;

	let mut green_characters = Vec::new();
	let mut possible_characters = Vec::new();
	let mut banned_characters = Vec::new();

	let words = fs::read_to_string("words.txt").unwrap();
	let mut word_vec: Vec<&str> = words.lines().collect();

	let mut win = false;

	let mut word: [Letter; 5] = [Letter::create_struct("a"), Letter::create_struct("e"), Letter::create_struct("s"), Letter::create_struct("i"), Letter::create_struct("r")];

	let mut are_green = false;
	let mut are_yellow = false;

	while !win {
		//handle cases

		let mut do_rep = true;

		if !first_round {
			let mut green_char = true;
			let mut contains_yellow = false;
			let mut no_garbage = true;
			let mut no_banned_chars = false;

			let mut i = 0;

			//loop through dictionary
			while i < word_vec.len() {

				//loop through characters of current word in dictionary
				//check for green
				if are_green == true {
					for my_index in 0..word.len() {
						if word[my_index].confirmed == true {
							if word[my_index].letter != String::from(word_vec[i].chars().nth(my_index).unwrap()) {
								green_char = false;
								break;
							} else {
								green_char = true;
							}
						}
					}
				} else {
					green_char = true;
				}

				//check if yellow characters are in word
				if are_yellow == true {
					//check for possible characters shown by yellow chars
					for o in 0..possible_characters.len() {
						//WORD DOES NOT CONTAIN THESE CHARACTERS, DEFINITE FALSE
						if !word_vec[i].contains(&possible_characters[o]) {
							contains_yellow = false;
							break;
						} else {
							contains_yellow = true;
						}
					}
				} else {
					contains_yellow = true;
				}

				//check for garbage
				let mut exit = false;
				for my_index in 0..word.len() {
					//check for characters that could have previously known to NOT BE VALID due to yellow chars
					for g in 0..word[my_index].garbage.len() {
						if word[my_index].garbage[g] != String::from(word_vec[i].chars().nth(my_index).unwrap()) {
							no_garbage = true;
						} else {
							no_garbage = false;
							exit = true;
							break;
						}
					}
					if exit == true {
						break;
					}
				}

				//check for banned characters shown by grey chars
				if are_green == true {
					//overide banned char if banned word has duplicate green char
					for gr in 0..green_characters.len() {
						if word_vec[i].contains(&green_characters[gr]) {
							no_banned_chars = true;
							break;
						} else {
							parse_garbage(&mut banned_characters, &mut word_vec, i, &mut no_banned_chars);
							break;
						}
					}
				} else {
					parse_garbage(&mut banned_characters, &mut word_vec, i, &mut no_banned_chars);
				}

				//for debug purposes
				//println!("{}", green_char);
				//println!("{}", contains_yellow);
				//println!("{}", no_garbage);
				//println!("{}", no_banned_chars);
				
				//final check + confirm word
				if green_char && contains_yellow && no_garbage && no_banned_chars {
					for a in 0..word.len() {
						word[a].letter = String::from(word_vec[i].chars().nth(a).unwrap());
					}
					word_vec.remove(i);
					break;
				} else {
					//reached end of current loop without finding answer. SEARCH AGAIN AUTOMATICALLY!
					if i == (word_vec.len() - 1) || i == (word_vec.len() - 2) {
						do_rep = false;
					}
					word_vec.remove(i);
					i += 1;
				}
			}
		} else {
			first_round = false;
		}

		if do_rep == true {
			let mut best_guess = String::new();

			for i in 0..word.len() {
				best_guess += &word[i].letter;
			}

			//println!("{:#?}", word);

			println!("Best Guess: {}", best_guess);

			let mut results = String::new();
			io::stdin().read_line(&mut results);
			
			parse_results(results, &mut word, &mut green_characters, &mut possible_characters, &mut banned_characters, &mut are_green, &mut are_yellow);
		}
	}
}
