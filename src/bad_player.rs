use crate::board::Board;
use crate::board::Side;
use crate::player::Player;
pub struct BadPlayer {
    name: String,
}

impl BadPlayer {
    pub fn new_from_name(name: &str) -> Self {
        BadPlayer {
            name: name.to_owned(),
        }
    }
}
impl Player for BadPlayer {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn is_interactive(&self) -> bool {
        false
    }

    fn choose_move(&self, b: &Board, s: Side) -> i32 {
        // choose the first legal move. Returns -1 if there is no legal move

        for hole in 1..=b.holes() {
            if b.beans(s, hole) > 0 {
                println!("{} chooses hole {}", self.name, hole);
                return hole;
            }
        }
        -1
    }
}
