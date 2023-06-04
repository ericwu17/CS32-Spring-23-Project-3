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

// evaluates a board recursively using the minimax algorithm defined in the spec
// b is a copy of the board, and evaluation_depth represents how much further we would like to go.
// returns a positive integer if the board is good for South, and a negative integer if the board
// is good for North.
fn evaluate_board(b: Board, evaluation_depth: i32, side_to_play: Side) -> BoardEval {
    if evaluation_depth == 0 {
        return BoardEval {
            eval: b.beans(Side::South, 0) - b.beans(Side::North, 0),
            best_move: 1,
        };
    }

    // a vec of boards that represent what the boards may look like after side_to_play
    // has finished their turn
    let mut possible_move_boards: Vec<(Board, i32)> = Vec::new();

    // a queue of boards that may be extended if there are intermediate positions that can
    // be reached within a single turn (intermediate positions are reached after a player ends a sowing
    // in their own pot)
    let mut reachable_boards: Vec<(Board, Option<i32>)> = Vec::new();
    reachable_boards.push((b.clone(), None));

    while reachable_boards.len() > 0 {
        let (original_board, source) = reachable_boards.pop().unwrap();
        for hole in 1..=original_board.holes() {
            // if the hole is a legal move (has > 0 beans):
            if original_board.beans(side_to_play, hole) > 0 {
                let mut clone_board = original_board.clone();

                let mut end_side: Side = Side::North;
                let mut end_hole: i32 = 0;
                clone_board.sow(side_to_play, hole, &mut end_side, &mut end_hole);

                // execute a capture if the move ends in a player's own empty hole, and the opponent's hole
                // opposite the ending hole is nonempty:
                #[allow(clippy::collapsible_if)]
                if end_hole != 0 && end_side == side_to_play {
                    if clone_board.beans(side_to_play, end_hole) == 1 {
                        // if the hole used to be empty
                        if clone_board.beans(side_to_play.opponent(), end_hole) != 0 {
                            // if the opponent's corresponding hole is nonempty, execute capture
                            clone_board.move_to_pot(side_to_play, end_hole, side_to_play);
                            clone_board.move_to_pot(
                                side_to_play.opponent(),
                                end_hole,
                                side_to_play,
                            );
                        }
                    }
                }

                if end_hole == 0 {
                    // if further move is needed because move ends at pot:
                    let source = match source {
                        None => Some(hole),
                        Some(x) => Some(x),
                    };
                    reachable_boards.push((clone_board, source));
                } else {
                    // no further move needed:
                    let source = source.unwrap_or(hole);
                    possible_move_boards.push((clone_board, source));
                }
            }
        }
    }

    let mut possible_board_evals = Vec::new();
    for (b, source) in possible_move_boards.into_iter() {
        possible_board_evals.push(BoardEval {
            eval: evaluate_board(b, evaluation_depth - 1, side_to_play.opponent()).eval,
            best_move: source,
        });
    }

    if possible_board_evals.len() == 0 {
        // no legal moves, the game is over. Returns i32::MAX if south has won, and i32::MIN if north has won. return 0 if it is a tie.

        let beans_i_have = b.beans(side_to_play, 0);
        let total_beans = b.total_beans();

        let eval = match beans_i_have.cmp(&(&total_beans / 2)) {
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

    // if South is playing, we return the max. If North is playing, we return the min.
    match side_to_play {
        Side::South => {
            let mut best_eval = possible_board_evals.pop().unwrap();

            for curr_eval in possible_board_evals.into_iter() {
                if curr_eval.eval > best_eval.eval {
                    best_eval = curr_eval;
                }
            }

            return best_eval;
        }
        Side::North => {
            let mut best_eval = possible_board_evals.pop().unwrap();

            for curr_eval in possible_board_evals.into_iter() {
                if curr_eval.eval < best_eval.eval {
                    best_eval = curr_eval;
                }
            }

            return best_eval;
        }
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

        let BoardEval { eval, best_move } = evaluate_board(b.clone(), 5, s);
        println!("{} thinks the evaluation is currently {}", self.name, eval);
        println!("{} chooses hole {}", self.name, best_move);
        best_move
    }
}
