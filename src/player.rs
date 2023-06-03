use super::board::{Board, Side};
pub trait Player {
    fn get_name(&self) -> String;
    fn is_interactive(&self) -> bool;
    fn choose_move(&self, b: &Board, s: Side) -> i32;
}
