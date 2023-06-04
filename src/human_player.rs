use crate::board::Board;
use crate::board::Side;
use crate::player::Player;
use std::io::{self, Write};

pub struct HumanPlayer {
    name: String,
}

impl HumanPlayer {
    pub fn new_from_name(name: &str) -> Self {
        HumanPlayer {
            name: name.to_owned(),
        }
    }
}

impl Player for HumanPlayer {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn is_interactive(&self) -> bool {
        true
    }

    fn choose_move(&self, b: &Board, s: Side) -> i32 {
        // first check for a legal move, and if none is available, then return -1

        let mut has_legal_move = false;
        for hole in 1..=b.holes() {
            if b.beans(s, hole) > 0 {
                has_legal_move = true;
                break;
            }
        }
        if !has_legal_move {
            return -1;
        }

        loop {
            print!("Select a hole, {}: ", self.name);
            io::stdout().flush().expect("failed to flush stdout!");

            let mut input_text = String::new();
            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");

            let trimmed = input_text.trim();
            match trimmed.parse::<u32>() {
                Ok(i) => {
                    let i = i as i32;
                    if i < 1 || i > b.holes() {
                        println!("The hole number must be from 1 to {}.", b.holes());
                        continue;
                    } else {
                        // check that there are beans in the hole:
                        if b.beans(s, i) > 0 {
                            return i;
                        } else {
                            println!("There are no beans in that hole.")
                        }
                    }
                }
                Err(..) => {
                    println!("The hole number must be from 1 to {}.", b.holes());
                    continue;
                }
            };
        }
    }
}
