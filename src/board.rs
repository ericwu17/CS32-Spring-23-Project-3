#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Side {
    North,
    South,
}

impl Side {
    pub fn opponent(&self) -> Side {
        match self {
            Side::North => Side::South,
            Side::South => Side::North,
        }
    }
}

pub struct Board {
    north_holes: Vec<i32>,
    south_holes: Vec<i32>,
    north_pot: i32,
    south_pot: i32,
    num_holes: i32,
}

impl Board {
    pub fn new(n_holes: i32, num_initial_beans_per_hole: i32) -> Self {
        let n_holes = if n_holes <= 0 { 1 } else { n_holes };

        Board {
            north_holes: vec![num_initial_beans_per_hole; n_holes as usize],
            south_holes: vec![num_initial_beans_per_hole; n_holes as usize],
            north_pot: 0,
            south_pot: 0,
            num_holes: n_holes,
        }
    }

    pub fn holes(&self) -> i32 {
        self.num_holes
    }
    pub fn beans(&self, s: Side, hole: i32) -> i32 {
        if hole < 0 || hole > self.holes() {
            return -1;
        }
        if hole == 0 {
            match s {
                Side::North => return self.north_pot,
                Side::South => return self.south_pot,
            }
        }
        match s {
            Side::North => {
                return *self.north_holes.get((hole - 1) as usize).unwrap();
            }
            Side::South => {
                return *self.south_holes.get((hole - 1) as usize).unwrap();
            }
        }
    }
    pub fn beans_in_play(&self, s: Side) -> i32 {
        let v = match s {
            Side::North => self.north_holes.iter(),
            Side::South => self.south_holes.iter(),
        };

        let mut acc = 0;
        for hole in v {
            acc += hole;
        }
        acc
    }
    pub fn total_beans(&self) -> i32 {
        let mut acc = 0;
        for hole in self.north_holes.iter() {
            acc += hole;
        }
        for hole in self.south_holes.iter() {
            acc += hole;
        }
        acc += self.north_pot + self.south_pot;
        acc
    }
    pub fn move_to_pot(&mut self, s: Side, hole: i32, pot_owner: Side) -> bool {
        if hole <= 0 || hole > self.num_holes {
            return false;
        }
        let num_beans_removed: i32;
        let hole_index = (hole - 1) as usize;
        match s {
            Side::North => {
                num_beans_removed = self.north_holes[hole_index];
                self.north_holes[hole_index] = 0;
            }
            Side::South => {
                num_beans_removed = self.south_holes[hole_index];
                self.south_holes[hole_index] = 0;
            }
        }
        match pot_owner {
            Side::North => {
                self.north_pot += num_beans_removed;
            }
            Side::South => {
                self.south_pot += num_beans_removed;
            }
        }

        true
    }
    pub fn set_beans(&mut self, s: Side, hole: i32, beans: i32) -> bool {
        if hole < 0 || hole > self.num_holes {
            return false;
        }
        if beans < 0 {
            return false;
        }
        if hole == 0 {
            match s {
                Side::North => {
                    self.north_pot = beans;
                }
                Side::South => {
                    self.south_pot = beans;
                }
            }
        } else {
            match s {
                Side::North => {
                    self.north_holes[(hole - 1) as usize] = beans;
                }
                Side::South => {
                    self.south_holes[(hole - 1) as usize] = beans;
                }
            }
        }

        true
    }

    pub fn sow(&mut self, side: Side, hole: i32, end_side: &mut Side, end_hole: &mut i32) -> bool {
        if hole <= 0 || hole > self.num_holes {
            return false;
        }

        let player_sowing: Side = side;

        let mut num_beans_in_hand = self.beans(side, hole);
        self.set_beans(side, hole, 0);

        let mut side = side;
        let mut hole = hole;

        while num_beans_in_hand > 0 {
            (side, hole) = Board::get_next_location(side, hole, player_sowing, self.num_holes);
            self.set_beans(side, hole, self.beans(side, hole) + 1);
            num_beans_in_hand -= 1;
        }

        *end_side = side;
        *end_hole = hole;

        true
    }

    pub fn get_next_location(
        side: Side,
        hole: i32,
        player_sowing: Side,
        num_holes: i32,
    ) -> (Side, i32) {
        assert!(hole >= 0 && hole <= num_holes);
        if hole == 0 {
            // if the hole is zero, that means we are at a players pot.
            // we know it is only possible if the player sowing is the same as the pot's side.
            match side {
                Side::North => return (Side::South, 1),
                Side::South => return (Side::North, num_holes),
            }
        }
        match side {
            Side::North => {
                if hole == 1 {
                    match player_sowing {
                        Side::North => (Side::North, 0),
                        Side::South => (Side::South, 1),
                    }
                } else {
                    (Side::North, hole - 1)
                }
            }
            Side::South => {
                if hole == num_holes {
                    match player_sowing {
                        Side::North => (Side::North, num_holes),
                        Side::South => (Side::South, 0),
                    }
                } else {
                    (Side::South, hole + 1)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use Side::*;

    #[test]
    fn sow_beans() {
        let mut b: Board = Board::new(4, 4);

        let mut end_side = North;
        let mut end_hole = -1;
        assert!(!b.sow(South, 0, &mut end_side, &mut end_hole));
        assert!(b.sow(South, 1, &mut end_side, &mut end_hole));
        assert!(end_hole == 0 && end_side == South);
        assert!(b.beans(South, 0) == 1);
        assert!(b.beans(South, 1) == 0);
        assert!(b.beans(South, 2) == 5);
        assert!(b.beans(South, 3) == 5);
        assert!(b.beans(South, 4) == 5);

        assert!(b.beans_in_play(North) == 16);
        assert!(b.beans_in_play(South) == 15);
        assert!(b.total_beans() == 32);
    }

    #[test]
    fn sow_with_many_beans_per_pot() {
        let mut b: Board = Board::new(2, 61);
        let mut end_side = North;
        let mut end_hole = -1;
        assert!(b.sow(South, 1, &mut end_side, &mut end_hole));

        assert!(end_side == South && end_hole == 2);

        assert_eq!(b.beans(South, 0), 12);
        assert_eq!(b.beans(South, 1), 12);
        assert_eq!(b.beans(South, 2), 74);
        assert_eq!(b.beans(North, 0), 0);
        assert_eq!(b.beans(North, 1), 73);
        assert_eq!(b.beans(North, 2), 73);
        assert_eq!(b.beans_in_play(South), 86);
        assert_eq!(b.beans_in_play(North), 146);
        assert_eq!(b.total_beans(), 244);
    }
}
