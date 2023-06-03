use crate::board::Board;
use crate::board::Side;
use crate::player::Player;
use std::io;

pub struct HumanPlayer {
    name: String,
}

impl Player for HumanPlayer {
    fn new_from_name(name: &str) -> Self {
        HumanPlayer {
            name: name.to_owned(),
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn is_interactive(&self) -> bool {
        true
    }

    fn choose_move(&self, b: &Board, _: Side) -> i32 {
        loop {
            println!("Select a hole, {}: ", self.name);

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
                        return i;
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
