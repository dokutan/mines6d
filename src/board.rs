use ndarray::prelude::*;
use rand::Rng;
use std::{collections::HashSet, vec::Vec};

#[cfg(test)]
#[path = "board_tests.rs"]
mod board_tests;

/// Stores the state of the board and handles the game logic.
pub struct Board {
    pub board: Array6<u16>,
    pub mines_total: u32,
    pub mines_flagged: u32,
    pub mines_marked: u32,
    pub cheats_total: u32,
    pub cheats_remaining: u32,
}

impl Board {
    /*
    The values of the cells are defined as follows:

    bits 15-14 : 00 → covered, 01 → flagged, 10 → marked, 11 → uncovered
    bit  13    : 0 → empty, 1 → mine
    bits 12-0  : number of mines in the cell or in the neighbouring cells
    */

    pub fn new(size: (usize, usize, usize, usize, usize, usize), mines: u32, cheats: u32) -> Self {
        let board = Array6::<u16>::zeros(size);

        let (x6, x5, x4, x3, x2, x1) = size;
        let cells_total = (x6 * x5 * x4 * x3 * x2 * x1) as u32;

        let mines = if mines > cells_total {
            cells_total
        } else {
            mines
        };

        let mut b = Board {
            board,
            mines_total: mines,
            mines_flagged: 0,
            mines_marked: 0,
            cheats_total: cheats,
            cheats_remaining: cheats,
        };
        b.place_mines(b.mines_total);

        b
    }

    /// Checks if a cell having value is covered.
    pub fn is_covered(value: u16) -> bool {
        value & 0xc000 == 0x0000
    }

    /// Checks if a cell having value is uncovered.
    pub fn is_uncovered(value: u16) -> bool {
        value & 0xc000 == 0xc000
    }

    /// Checks if a cell having value is flagged.
    pub fn is_flagged(value: u16) -> bool {
        value & 0xc000 == 0x4000
    }

    /// Checks if a cell having value is marked.
    pub fn is_marked(value: u16) -> bool {
        value & 0xc000 == 0x8000
    }

    /// Checks if a cell having value is empty.
    pub fn is_empty(value: u16) -> bool {
        value & 0x2000 == 0x0000
    }

    /// Returns the number of mines in a cell, or the number of mines in the neighboring cells if it is empty.
    pub fn mines(value: u16) -> u16 {
        value & 0x1fff
    }

    // Returns all neighbors of the given cell
    pub fn neighbors(
        &self,
        cell: (usize, usize, usize, usize, usize, usize),
    ) -> Vec<(usize, usize, usize, usize, usize, usize)> {
        let mut result = Vec::new();
        let (x6, x5, x4, x3, x2, x1) = cell;
        let (s6, s5, s4, s3, s2, s1) = self.board.dim();

        if x6 > 0 {
            result.push((x6 - 1, x5, x4, x3, x2, x1));
        }
        if x6 < s6 - 1 {
            result.push((x6 + 1, x5, x4, x3, x2, x1));
        }
        if x5 > 0 {
            result.push((x6, x5 - 1, x4, x3, x2, x1));
        }
        if x5 < s5 - 1 {
            result.push((x6, x5 + 1, x4, x3, x2, x1));
        }
        if x4 > 0 {
            result.push((x6, x5, x4 - 1, x3, x2, x1));
        }
        if x4 < s4 - 1 {
            result.push((x6, x5, x4 + 1, x3, x2, x1));
        }
        if x3 > 0 {
            result.push((x6, x5, x4, x3 - 1, x2, x1));
        }
        if x3 < s3 - 1 {
            result.push((x6, x5, x4, x3 + 1, x2, x1));
        }
        if x2 > 0 {
            result.push((x6, x5, x4, x3, x2 - 1, x1));
        }
        if x2 < s2 - 1 {
            result.push((x6, x5, x4, x3, x2 + 1, x1));
        }
        if x1 > 0 {
            result.push((x6, x5, x4, x3, x2, x1 - 1));
        }
        if x1 < s1 - 1 {
            result.push((x6, x5, x4, x3, x2, x1 + 1));
        }

        result
    }

    /// Decrements self.cheats_remaining and reveals the contents of a covered cell if self.cheats_remaining > 0.
    /// Returns true if all mines have been correctly identified.
    pub fn cheat_cell(&mut self, cell: (usize, usize, usize, usize, usize, usize)) -> bool {
        if self.cheats_remaining == 0 {
            return false;
        }

        let (x6, x5, x4, x3, x2, x1) = cell;

        if Board::is_empty(self.board[[x6, x5, x4, x3, x2, x1]])
            && !Board::is_uncovered(self.board[[x6, x5, x4, x3, x2, x1]])
        {
            self.cheats_remaining -= 1;
            self.uncover_recursively(cell);
            false
        } else if !Board::is_uncovered(self.board[[x6, x5, x4, x3, x2, x1]]) {
            self.cheats_remaining -= 1;
            self.flag_cell(cell)
        } else {
            false
        }
    }

    /// Flags a cell as containing a mine, returns true if all mines have been correctly identified.
    pub fn flag_cell(&mut self, cell: (usize, usize, usize, usize, usize, usize)) -> bool {
        let (x6, x5, x4, x3, x2, x1) = cell;

        if !Board::is_flagged(self.board[[x6, x5, x4, x3, x2, x1]])
            && Board::is_covered(self.board[[x6, x5, x4, x3, x2, x1]])
        {
            self.board[[x6, x5, x4, x3, x2, x1]] =
                (self.board[[x6, x5, x4, x3, x2, x1]] | 0x4000) & 0x7fff;
            self.mines_flagged += 1;
        } else if Board::is_flagged(self.board[[x6, x5, x4, x3, x2, x1]]) {
            self.board[[x6, x5, x4, x3, x2, x1]] &= 0x3fff;
            self.mines_flagged -= 1;
        }

        // are all mines flagged correctly ?
        if self.mines_flagged == self.mines_total {
            for cell in self.board.iter() {
                if Board::is_flagged(*cell) && Board::is_empty(*cell) {
                    return false;
                }
            }

            true
        } else {
            false
        }
    }

    /// Marks a cell as maybe containing a mine.
    pub fn mark_cell(&mut self, cell: (usize, usize, usize, usize, usize, usize)) {
        let (x6, x5, x4, x3, x2, x1) = cell;

        if Board::is_covered(self.board[[x6, x5, x4, x3, x2, x1]]) {
            self.board[[x6, x5, x4, x3, x2, x1]] =
                (self.board[[x6, x5, x4, x3, x2, x1]] | 0x8000) & 0xbfff;
            self.mines_marked += 1;
        } else if Board::is_marked(self.board[[x6, x5, x4, x3, x2, x1]]) {
            self.board[[x6, x5, x4, x3, x2, x1]] &= 0x3fff;
            self.mines_marked -= 1;
        }
    }

    /// Marks a cell as uncovered, returns true if this results in the game being lost.
    pub fn uncover_cell(&mut self, cell: (usize, usize, usize, usize, usize, usize)) -> bool {
        let (x6, x5, x4, x3, x2, x1) = cell;

        if !Board::is_empty(self.board[[x6, x5, x4, x3, x2, x1]]) {
            true
        } else if Board::is_covered(self.board[[x6, x5, x4, x3, x2, x1]]) {
            self.uncover_recursively(cell);
            false
        } else {
            false
        }
    }

    /// Recursively uncovers empty cells.
    fn uncover_recursively(&mut self, cell: (usize, usize, usize, usize, usize, usize)) {
        let mut set = HashSet::new();
        set.insert(cell);

        // repeat until the set is empty
        loop {
            // using these sets is required to avoid a mutable iterator
            let mut remove_set = HashSet::new(); // holds the elements that should be removed from set
            let mut insert_set = HashSet::new(); // holds the elements that should be inserted into set

            for c in set.iter() {
                let (x6, x5, x4, x3, x2, x1) = *c;

                // remove cell from set
                remove_set.insert(*c);

                // uncover cell
                self.board[[x6, x5, x4, x3, x2, x1]] |= 0xc000;

                // store covered neighbors if the cell has no mines as neighbors
                if Board::mines(self.board[[x6, x5, x4, x3, x2, x1]]) == 0 {
                    for n in self.neighbors(*c) {
                        let (x6, x5, x4, x3, x2, x1) = n;

                        if Board::is_covered(self.board[[x6, x5, x4, x3, x2, x1]]) {
                            insert_set.insert(n);
                        }
                    }
                }
            }

            // update set
            set.extend(insert_set);
            for c in remove_set {
                set.remove(&c);
            }

            if set.is_empty() {
                break;
            }
        }
    }

    /// Randomly places the given number of mines on the board.
    fn place_mines(&mut self, number: u32) {
        let mut rng = rand::thread_rng();
        let (s6, s5, s4, s3, s2, s1) = self.board.dim();
        let mut number = number;

        while number > 0 {
            let x6: usize = rng.gen_range(0..s6);
            let x5: usize = rng.gen_range(0..s5);
            let x4: usize = rng.gen_range(0..s4);
            let x3: usize = rng.gen_range(0..s3);
            let x2: usize = rng.gen_range(0..s2);
            let x1: usize = rng.gen_range(0..s1);

            if Board::is_empty(self.board[[x6, x5, x4, x3, x2, x1]]) {
                self.board[[x6, x5, x4, x3, x2, x1]] = 0x2001;
                number -= 1;

                // increment the values of the neighbouring cells
                for n in self.neighbors((x6, x5, x4, x3, x2, x1)) {
                    let (x6, x5, x4, x3, x2, x1) = n;

                    if Board::is_empty(self.board[[x6, x5, x4, x3, x2, x1]]) {
                        self.board[[x6, x5, x4, x3, x2, x1]] += 1;
                    }
                }
            }
        }
    }
}
