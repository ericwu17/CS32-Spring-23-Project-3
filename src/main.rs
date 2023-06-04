use bad_player::BadPlayer;
use board::Board;
use game::Game;
use human_player::HumanPlayer;
use smart_player::SmartPlayer;

mod bad_player;
mod board;
mod game;
mod human_player;
mod player;
mod smart_player;

fn main() {
    let player1 = SmartPlayer::new_from_name("Homer");
    let player2 = HumanPlayer::new_from_name("Eric");

    let board = Board::new(6, 4);

    let mut game = Game::new(board, Box::new(player2), Box::new(player1));

    game.play();
}
