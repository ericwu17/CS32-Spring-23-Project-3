use super::board::{Board, Side};
pub trait Player {
    fn new_from_name(name: &str) -> Self;
    fn get_name(&self) -> String;
    fn is_interactive(&self) -> bool;
    fn choose_move(&self, b: &Board, s: Side) -> i32;
}
