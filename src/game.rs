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

    fn get_player(&self, s: Side) -> &Box<dyn Player> {
        match s {
            Side::North => &self.north,
            Side::South => &self.south,
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

        let player = match s {
            Side::North => &self.north,
            Side::South => &self.south,
        };

        loop {
            let move_chosen = player.choose_move(&self.board, s);

            if move_chosen == -1 {
                // indicates no move is possible, and so sweep beans into s's opponent's holes and return false.

                println!(
                    "{} has no beans left to sow.",
                    self.get_player(s).get_name()
                );
                println!(
                    "Sweeping remaining beans into {}'s pot.",
                    self.get_player(s.opponent()).get_name()
                );

                for hole in 1..=self.board.holes() {
                    self.board.move_to_pot(s.opponent(), hole, s.opponent());
                }

                self.is_over = true;

                let north_pot_beans = self.board.beans(Side::North, 0);
                let south_pot_beans = self.board.beans(Side::South, 0);

                self.winner = if north_pot_beans > south_pot_beans {
                    Some(Side::North)
                } else if north_pot_beans < south_pot_beans {
                    Some(Side::South)
                } else {
                    None
                };

                return false;
            }

            let mut end_side: Side = Side::North;
            let mut end_hole: i32 = 0;

            self.board.sow(s, move_chosen, &mut end_side, &mut end_hole);

            if false {
                // TODO: if further move is needed:
            } else {
                break;
            }
        }

        true
    }

    pub fn play(&mut self) {
        let mut side_to_move = Side::South;
        loop {
            self.display();
            let res = self.make_move(side_to_move);
            if !res {
                break;
            }

            side_to_move = side_to_move.opponent();
        }

        match self.winner {
            Some(Side::North) => {
                println!("The winner is {}.", self.north.get_name())
            }
            Some(Side::South) => {
                println!("The winner is {}.", self.south.get_name())
            }
            None => {
                println!("The game is a tie.")
            }
        }
    }

    fn beans(&self, s: Side, hole: i32) -> i32 {
        return self.board.beans(s, hole);
    }
}
