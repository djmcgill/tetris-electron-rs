extern crate rand;
use self::rand::{thread_rng, Rng};

pub const CELLS_WIDE: usize = 8;
pub const CELLS_HIGH: usize = 20;
#[derive(Copy, Clone)]
pub enum Block { T, I, O, S, Z, J, L }
impl Block {
    pub fn cells_taken(&self) -> [[bool; 4]; 4] {
        match *self {
            Block::T => [
                [false, false, false, false],
                [false, false, false, false],
                [true , true , true , false],
                [false, true , false, false],
            ],
            Block::I => [
                [true, false, false, false],
                [true, false, false, false],
                [true, false, false, false],
                [true, false, false, false],
            ],
            Block::O => [
                [false, false, false, false],
                [false, false, false, false],
                [true , true , false, false],
                [true , true , false, false],
            ],
            Block::S => [
                [false, false, false, false],
                [false, false, false, false],
                [false, true , true , false],
                [true , true , false, false],
            ],
            Block::Z => [
                [false, false, false, false],
                [false, false, false, false],
                [true , true , false, false],
                [false, true , true , false],
            ],
            Block::J => [
                [false, false, false, false],
                [false, false, false, false],
                [true , true , true , false],
                [false, false, true , false],
            ],
            Block::L => [
                [false, false, false, false],
                [false, false, false, false],
                [true , true , true , false],
                [true , false, false, false],
            ],
        }
    }

    fn random_block() -> Self {
        thread_rng().choose(&[
            Block::T,
            Block::I,
            Block::O,
            Block::S,
            Block::Z,
            Block::J,
            Block::L,
        ]).unwrap().clone()
    }
}
#[derive(Copy, Clone)]
pub struct FallingBlock {
    pub block: Block,
    pub x: usize, // 0,0 is the bottom left corner
    pub y: usize,
}
impl FallingBlock {
    pub fn check_collision(&self, cells: &Cells) -> bool {
        let cells_taken = self.block.cells_taken();
        for x in 0..3 {
            let new_x = self.x + x;
            for y in 0..3 {
                let new_y = self.y + y;
                if new_x >= CELLS_WIDE ||
                    new_y >= CELLS_HIGH ||
                    (cells_taken[x][y] && cells[self.x + x][self.y + y]) {
                        return true;
                }
            }
        }
        return false;
    }

    pub fn insert_into_cells(&self, mut cells: Cells) -> Cells {
        let cells_taken = self.block.cells_taken();
        for x in 0..3 {
            for y in 0..3 {
                cells[self.x + x][self.y + y] |= cells_taken[x][y];
            }
        }
        for y in 0..CELLS_HIGH {
            let mut full = true;
            for x in 0..CELLS_WIDE {
                if !cells[x][y] { full = false; break; }
            }
            if full {
                for new_y in y+1..CELLS_HIGH {
                    for x in 0..CELLS_WIDE {
                        cells[x][new_y] = cells[x][new_y-1];
                    }
                }
            }
        }
        cells
    }

    pub fn move_within_bounds(&self, block_command: BlockCommand) -> Option<Self> {
        match block_command {
            BlockCommand::MovL if self.x == 0 => None,
            BlockCommand::MovL => {
                let mut new_self = self.clone();
                new_self.x -= 1;
                Some(new_self)
            }
            BlockCommand::MovR => {
                let cells_taken = self.block.cells_taken();
                for x in 0..3 {
                    for y in 0..3 {
                        if cells_taken[x][y] && self.x + x + 1 >= CELLS_WIDE {
                            return None
                        }
                    }
                }
                let mut new_self = self.clone();
                new_self.x += 1;
                Some(new_self)
            },
            _ => Some(self.clone())
        }
    }
}

pub type Cell = bool;
pub type Cells = [[Cell; CELLS_HIGH]; CELLS_WIDE];
#[derive(Clone)]
pub struct Board {
    pub cells: Cells,
    pub falling_block: Option<FallingBlock>,
}
pub type UserInput = BlockCommand;
pub enum BlockCommand { MovL, MovR, RotL, RotR }

impl Board {
    fn new_falling_block(&mut self) -> bool {
        let new_falling_block = FallingBlock {
            block: Block::random_block(),
            x: CELLS_WIDE/2,
            y: CELLS_HIGH-4,
        };
        if new_falling_block.check_collision(&self.cells) {
            return false
        } else {
            self.falling_block = Some(new_falling_block);
            return true;
        }
    }

    fn insert_block_into_cells(&mut self) {
        if let Some(falling_block) = self.falling_block {
            let new_cells = falling_block.insert_into_cells(self.cells);
            self.cells = new_cells;
            self.falling_block = None;
        }
    }

    pub fn step(&mut self) -> bool {
        match self.falling_block.clone() {
            None => self.new_falling_block(),
            Some(falling_block) if falling_block.y > 0 => { // falling_block.y != 0
                let new_falling_block = FallingBlock {
                    block: falling_block.block,
                    x: falling_block.x,
                    y: falling_block.y - 1,
                };
                if new_falling_block.check_collision(&self.cells) {
                    self.insert_block_into_cells();
                    true
                } else {
                    self.falling_block = Some(new_falling_block);
                    true
                }
            },
            Some(_) => { // if falling_block.y == 0 => {
                self.insert_block_into_cells();
                true
            },
        }
    }


    pub fn input(&mut self, input: UserInput) {
        if let Some(falling_block) = self.falling_block {
            if let Some(new_falling_block) = falling_block.move_within_bounds(input) {
                if !new_falling_block.check_collision(&self.cells) {
                    self.falling_block = Some(new_falling_block);
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn step_move_down() { panic!() }

    #[test]
    fn step_join_cells() { panic!() }

    #[test]
    fn step_lose_game() { panic!() }

    #[test]
    fn step_new_falling() { panic!() }

    #[test]
    fn step_bottom() { panic!() }

    #[test]
    fn step_line_bottom() { panic!() }

    #[test]
    fn step_line_mid() { panic!() }

    #[test]
    fn step_two_lines() { panic!() }

    #[test]
    fn input_left() { panic!() }

    #[test]
    fn input_left_over_bounds() { panic!() }

    #[test]
    fn input_left_into_block() { panic!() }

    #[test]
    fn input_right() { panic!() }

    #[test]
    fn input_right_over_bounds() { panic!() }

    #[test]
    fn input_right_nearly_over_bounds() { panic!() }

    #[test]
    fn input_right_into_block() { panic!() }
}
