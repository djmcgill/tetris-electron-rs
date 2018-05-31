const CELLS_WIDE: usize = 8;
const CELLS_HIGH: usize = 20;
#[derive(Copy, Clone)]
pub enum Block { T, I}
impl Block {
    pub fn cells_taken(&self) -> [[bool; 3]; 3] {
        match *self {
            Block::T => [
                [false, false, false],
                [true , true , true ],
                [false, true , false],
            ],
            Block::I => [
                [true, false, false],
                [true, false, false],
                [true, false, false],
            ],
        }
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
            for y in 0..3 {
                if cells_taken[x][y] && cells[self.x + x][self.y + y] {
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
                cells[self.x + x][self.y + y] = cells_taken[x][y];
            }
        }
        cells
    }

    pub fn move_within_bounds(&self, block_command: BlockCommand) -> Option<Self> {
        match block_command {
            BlockCommand::L if self.x == 0 => None,
            BlockCommand::L => {
                let mut new_self = self.clone();
                new_self.x -= 1;
                Some(new_self)
            }
            BlockCommand::R => {
                let mut new_self = self.clone();
                new_self.x += 1;
                let cells_taken = self.block.cells_taken();
                for x in 0..3 {
                    for y in 0..3 {
                        if cells_taken[x][y] && self.x + x >= CELLS_WIDE {
                            return None
                        }
                    }
                }
                Some(new_self)
            },
        }
    }
}



pub type Cell = bool;
pub type Cells = [[Cell; CELLS_HIGH]; CELLS_WIDE];
#[derive(Copy, Clone)]
pub struct Board {
    pub cells: Cells,
    pub falling_block: Option<FallingBlock>,
}
type UserInput = BlockCommand;
pub enum BlockCommand { L, R } // todo: support rotate

impl Board {
    fn new_falling_block(mut self) -> Option<Board> {
        let new_falling_block = FallingBlock {
            block: Block::T, // todo: randomise
            x: CELLS_WIDE/2,
            y: CELLS_HIGH-1,
        };
        if new_falling_block.check_collision(&self.cells) {
            return None
        } else {
            self.falling_block = Some(new_falling_block);
            return Some(self);
        }
    }

    fn insert_block_into_cells(mut self) -> Self {
        if let &Some(ref falling_block) = &self.falling_block.clone() {
            let new_cells = falling_block.insert_into_cells(self.cells);
            self.cells = new_cells;
            self.falling_block = None;
            self
        } else {
            self // this should never happen
        }
    }

    pub fn step(self) -> Option<Board> {
        match &self.falling_block {
            &None => self.new_falling_block(),
            &Some(ref falling_block) if falling_block.y == 0 => {
                return Some(self.insert_block_into_cells())
            },
            &Some(ref falling_block) => { // falling_block.y != 0
                let new_falling_block = FallingBlock {
                    block: falling_block.block,
                    x: falling_block.x,
                    y: falling_block.y - 1,
                };
                if new_falling_block.check_collision(&self.cells) {
                    return Some(self.insert_block_into_cells())
                } else {
                    let mut new_board = self.clone();
                    new_board.falling_block = Some(new_falling_block);
                    return Some(new_board);
                }
            },
        }
    }


    pub fn input(self, input: UserInput) -> Self {
        match &self.falling_block {
            &None => self, // ignore user input
            &Some(ref falling_block) => { // if possible, move block left or right
                match falling_block.move_within_bounds(input) {
                    Some(new_falling_block) if !new_falling_block.check_collision(&self.cells) => {
                        let mut new_board = self.clone();
                        new_board.falling_block = Some(new_falling_block);
                        new_board
                    },
                    _ => self
                }
            },
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
