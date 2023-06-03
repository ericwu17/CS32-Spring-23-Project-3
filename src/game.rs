use crate::board::{Board, Side};
use crate::player::Player;

pub struct Game {
    board: Board,
    north: Box<dyn Player>,
    south: Box<dyn Player>,
    is_over: bool,
    winner: Option<Side>,
}

pub struct GameStatus {
    over: bool,
    winner: Option<Side>,
}

impl Game {
    pub fn new(board: Board, south: Box<dyn Player>, north: Box<dyn Player>) -> Self {
        Game {
            board,
            north,
            south,
            is_over: false,
            winner: None,
        }
    }

    pub fn display(&self) {
        let total_line_length = 2 * self.board.holes() + 5;

        // print north name
        let shift_amt = (total_line_length - self.north.get_name().len() as i32) / 2;
        for _ in 0..shift_amt {
            print!(" ");
        }
        println!("{}", self.north.get_name());

        // print north holes
        print!("   ");
        for hole in 1..=self.board.holes() {
            print!("{}", self.board.beans(Side::North, hole));
            print!(" ");
        }
        print!("\n");

        // print both players' pots
        print!(" ");
        print!("{} ", self.board.beans(Side::North, 0));
        for _ in 1..=self.board.holes() {
            print!("  ")
        }
        print!("{} \n", self.board.beans(Side::South, 0));

        // print south holes
        print!("   ");
        for hole in 1..=self.board.holes() {
            print!("{}", self.board.beans(Side::South, hole));
            print!(" ");
        }
        print!("\n");

        // print south name
        let shift_amt = (total_line_length - self.south.get_name().len() as i32) / 2;
        for _ in 0..shift_amt {
            print!(" ");
        }
        println!("{}", self.south.get_name());
    }

    fn status(&self) -> GameStatus {
        GameStatus {
            over: self.is_over,
            winner: self.winner,
        }
    }

    fn make_move(&mut self, s: Side) -> bool {
        // Attempt to make a complete move for the player playing side s.
        // "Complete" means that the player sows the seeds from a hole and takes any additional
        // turns required or completes a capture. Whenever the player gets an additional turn,
        // you should display the board so someone looking at the screen can follow what's
        // happening. If the move can be completed, return true; if not, because the move is not
        // yet completed but side s has no holes with beans to pick up and sow, sweep any beans
        // in s's opponent's holes into that opponent's pot and return false.

        // TODO:

        true
    }

    fn beans(&self, s: Side, hole: i32) -> i32 {
        return self.board.beans(s, hole);
    }
}
