use std::fmt::{self, Display, Formatter, Write};
use std::io::stdin;

const OFFSETS: [(i32, i32); 8] =
    [(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)];

struct Board<const WIDTH: usize, const HEIGHT: usize>(pub [[bool; WIDTH]; HEIGHT]);
impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT> {
    /// Create a new, empty board
    fn new() -> Self {
        Self([[false; WIDTH]; HEIGHT])
    }

    /// Gets whether the given cell is alive
    /// Returns Some(val) if cell was on board, else None
    fn get(&self, (x, y): (i32, i32)) -> Option<bool> {
        // Get row
        self.0.get(y as usize)
            // If on board, get cell
            .and_then(|row| row.get(x as usize))
            // Copy it out of the array
            .map(|v| *v)
    }

    /// Sets whether a cell is alive
    /// Returns Some(()) if cell was on board, else None
    fn set(&mut self, (x, y): (i32, i32), val: bool) -> Option<()> {
        // Get row mut
        self.0.get_mut(y as usize)
            // If on board, get cell mut
            .and_then(|row| row.get_mut(x as usize))
            // Set cell to input
            .map(|v| *v = val)
    }

    /// Computes the next generation of the board
    fn next(self) -> Self {
        // Create new destination board
        let mut new = Self::new();
        // Iter over every cell in the destination board, mutably
        for (y, row) in new.0.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                // Count neighbors
                // For each offset
                let neighbors = OFFSETS.iter()
                    // Get the living status of the neighboring cell, or false if it's off the board
                    .map(|(a, b)| self.get(((x as i32)+a, (y as i32)+b)).unwrap_or(false))
                    // Count the number of alive cells
                    .fold(0u8, |acc, v| acc+(v as u8));
                // Set the current cell to its new state based on the current state and neighbors
                *cell = match (self.get((x as i32, y as i32)).unwrap(), neighbors) {
                    // If alive and 2 neighbors, or any state and 3 neighbors, then alive
                    (true, 2) | (_, 3) => true,
                    // Else, dead
                    _ => false
                };
            }
        }

        new
    }

    /// Copies a given board into a specific position of the current board
    fn blit<const OTHER_WIDTH: usize, const OTHER_HEIGHT: usize>(&mut self,  (x, y): (i32, i32), other: &Board<OTHER_WIDTH, OTHER_HEIGHT>) {
        // Iter over every cell in the given board
        for (y_offset, row) in other.0.iter().enumerate() {
            for (x_offset, &cell) in row.iter().enumerate() {
                // Write that cell's state to the corresponding cell in the current board
                self.set((x + (x_offset as i32), y+(y_offset as i32)), cell);
            }
        }
    }
}
impl<const WIDTH: usize, const HEIGHT: usize> Display for Board<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Normally you'd use the `write!` macro in here to write to the Formatter, but in this
        // case, it's more efficient to write char-by-char.

        // Iter cells
        for row in self.0.iter() {
            for &cell in row.iter() {
                // Write cell
                f.write_char(if cell {'X'} else {'_'})?;
            }
            // Write newline
            f.write_char('\n')?;
        }

        Ok(())
    }
}

// The Acorn: a common starting position
const ACORN: Board<7, 3> = Board([
    [false, true, false, false, false, false, false],
    [false, false, false, true, false, false, false],
    [true, true, false, false, true, true, true]
]);

fn main() {
    // Create a big board
    let mut board: Board<40, 40> = Board::new();
    // Put the Acorn seed in the middle
    board.blit((17, 19), &ACORN);

    // Run
    loop {
        println!("{}", board);
        stdin().read_line(&mut String::new()).unwrap();
        board = board.next();
    }
}
