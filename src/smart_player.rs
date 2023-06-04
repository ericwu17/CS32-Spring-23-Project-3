#![allow(dead_code)]

use std::cmp::Ordering;

use crate::board::Board;
use crate::board::Side;
use crate::player::Player;
pub struct SmartPlayer {
    name: String,
}

impl SmartPlayer {
    pub fn new_from_name(name: &str) -> Self {
        SmartPlayer {
            name: name.to_owned(),
        }
    }
}

struct BoardEval {
    eval: i32,
    best_move: i32,
}

fn compute_heuristic(b: &Board) -> i32 {
    if b.beans(Side::North, 0) > b.total_beans() / 2 {
        i32::MIN
    } else if b.beans(Side::South, 0) > b.total_beans() / 2 {
        i32::MAX
    } else {
        b.beans(Side::South, 0) - b.beans(Side::North, 0)
    }
}

// evaluates a board recursively using the minimax algorithm defined in the spec
// b is a copy of the board, and evaluation_depth represents how much further we would like to go.
// returns a positive integer if the board is good for South, and a negative integer if the board
// is good for North.
fn evaluate_board(b: Board, evaluation_depth: i32, side_to_play: Side) -> BoardEval {
    if b.beans_in_play(side_to_play) == 0 {
        // no legal moves, the game is over. Returns i32::MAX if south has won, and i32::MIN if north has won. return 0 if it is a tie.

        let beans_i_have = b.beans(side_to_play, 0);
        let total_beans = b.total_beans();

        let eval = match beans_i_have.cmp(&(total_beans / 2)) {
            Ordering::Equal => 0,
            Ordering::Greater => {
                if side_to_play == Side::South {
                    i32::MAX
                } else {
                    i32::MIN
                }
            }
            Ordering::Less => {
                if side_to_play == Side::South {
                    i32::MIN
                } else {
                    i32::MAX
                }
            }
        };

        return BoardEval {
            eval,
            best_move: -1,
        };
    }

    if evaluation_depth == 0 {
        return BoardEval {
            eval: compute_heuristic(&b),
            best_move: 1,
        };
    }

    let mut best_value = match side_to_play {
        // south is the maximizing player
        Side::South => i32::MIN,
        // north is the minimizing player
        Side::North => i32::MAX,
    };
    let mut best_move = -1;

    for hole in 1..=b.holes() {
        if b.beans(side_to_play, hole) == 0 {
            continue;
        }
        let mut board_clone = b.clone();

        let mut end_side: Side = Side::North;
        let mut end_hole: i32 = 0;
        board_clone.sow(side_to_play, hole, &mut end_side, &mut end_hole);

        // execute a capture if the move ends in a player's own empty hole, and the opponent's hole
        // opposite the ending hole is nonempty:
        #[allow(clippy::collapsible_if)]
        if end_hole != 0 && end_side == side_to_play {
            if board_clone.beans(side_to_play, end_hole) == 1 {
                // if the hole used to be empty
                if board_clone.beans(side_to_play.opponent(), end_hole) != 0 {
                    // if the opponent's corresponding hole is nonempty, execute capture
                    board_clone.move_to_pot(side_to_play, end_hole, side_to_play);
                    board_clone.move_to_pot(side_to_play.opponent(), end_hole, side_to_play);
                }
            }
        }

        let next_player = if end_hole == 0 {
            side_to_play
        } else {
            side_to_play.opponent()
        };

        let BoardEval { eval, best_move: _ } =
            evaluate_board(board_clone, evaluation_depth - 1, next_player);

        match side_to_play {
            Side::South => {
                if eval >= best_value {
                    best_value = eval;
                    best_move = hole;
                }
            }
            Side::North => {
                if eval <= best_value {
                    best_value = eval;
                    best_move = hole;
                }
            }
        }
    }

    BoardEval {
        eval: best_value,
        best_move,
    }
}

impl Player for SmartPlayer {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn is_interactive(&self) -> bool {
        false
    }

    fn choose_move(&self, b: &Board, s: Side) -> i32 {
        // check for legal moves first. If none exists, return -1
        if b.beans_in_play(s) == 0 {
            return -1;
        }

        let BoardEval { eval, best_move } = evaluate_board(b.clone(), 8, s);
        println!("{} thinks the evaluation is currently {}", self.name, eval);
        println!("{} chooses hole {}", self.name, best_move);
        best_move
    }
}
