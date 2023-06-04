use bad_player::BadPlayer;
use board::Board;
use game::Game;
use human_player::HumanPlayer;

mod bad_player;
mod board;
mod game;
mod human_player;
mod player;

fn main() {
    let player1 = BadPlayer::new_from_name("Homer");
    let player2 = HumanPlayer::new_from_name("Marge");

    let board = Board::new(3, 2);

    let mut game = Game::new(board, Box::new(player2), Box::new(player1));

    game.play();
}
