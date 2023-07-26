fn main() {
    // Build the board
    #[rustfmt::skip]
    let str_board = [
        "QPPP    ",
        "P    P  ",
        "P      P",
        " P    P ",
        "P     P ",
        "P     P ",
        "  P     ",
        "P   P   ",
    ];

    let board = Board::new(&str_board);

    // Calculate solutions
    let mut solutions = Vec::new();

    recurse(board.clone(), Vec::new(), 16, &mut solutions);

    // Print solutions
    for (i, sol) in solutions.iter().enumerate() {
        println!("=== Solution {} ===", i + 1);

        let mut board = board.clone();

        for (j, &(row, col)) in sol.iter().enumerate() {
            println!("--- Move {} ---", j + 1);

            board.move_to(row, col);
            board.print();
        }
    }
}

fn recurse(board: Board, moves: Vec<(u8, u8)>, left: u8, solutions: &mut Vec<Vec<(u8, u8)>>) {
    // Get all possible next moves
    let available_moves = board.next_moves();

    // Iterate next moves
    for (row, col) in available_moves {
        // Build next board
        let mut next_board = board.clone();
        next_board.move_to(row, col);

        // Add this move to new moves vector
        let mut next_moves = moves.clone();
        next_moves.push((row, col));

        if left == 1 {
            // Taking the last pawn
            solutions.push(next_moves);
        } else {
            // Recurse
            recurse(next_board, next_moves, left - 1, solutions);
        }
    }
}

#[derive(Clone)]
struct Board {
    board: u64, // Bitmask of pawn positions
    q_col: u8,  // Queen column
    q_row: u8,  // Queen row
}

impl Board {
    // Convert string array to a board
    fn new(str_board: &[&str; 8]) -> Self {
        // Convert string array to board bits
        let board = str_board
            .iter()
            .enumerate()
            .fold(0u64, |cur, (row_num, row)| {
                cur | row.chars().enumerate().fold(0u64, |cur, (col_num, c)| {
                    if c == 'P' {
                        cur | Self::pos_bit(row_num as u8, col_num as u8)
                    } else {
                        cur
                    }
                })
            });

        // Get queen position
        let q_row = str_board
            .iter()
            .position(|row| row.contains('Q'))
            .expect("Queen row not found");

        let q_col = str_board[q_row]
            .chars()
            .position(|c| c == 'Q')
            .expect("Queen column not found");

        // Create
        Self {
            board,
            q_col: q_col as u8,
            q_row: q_row as u8,
        }
    }

    /// Get bit mask for board position
    #[inline]
    fn pos_bit(row: u8, col: u8) -> u64 {
        1 << (col + (row * 8))
    }

    /// Tests if a board position contains a pawn
    #[inline]
    fn occupied(&self, row: u8, col: u8) -> bool {
        (self.board & Self::pos_bit(row, col)) != 0
    }

    /// Moves the queen to a new position and removes the pawn
    fn move_to(&mut self, row: u8, col: u8) {
        // Move the queen
        self.q_col = col;
        self.q_row = row;

        // Remove the pawn
        self.board &= !Self::pos_bit(row, col);
    }

    /// Calculate all next valid moves for the queen
    fn next_moves(&self) -> Vec<(u8, u8)> {
        let mut moves = Vec::with_capacity(8);

        // West
        if let Some(col) = (0..self.q_col)
            .rev()
            .find(|col| self.occupied(self.q_row, *col))
        {
            moves.push((self.q_row, col))
        }

        // East
        if let Some(col) = ((self.q_col + 1)..8).find(|col| self.occupied(self.q_row, *col)) {
            moves.push((self.q_row, col))
        }

        // North
        if let Some(row) = (0..self.q_row)
            .rev()
            .find(|row| self.occupied(*row, self.q_col))
        {
            moves.push((row, self.q_col))
        }

        // South
        if let Some(row) = ((self.q_row + 1)..8).find(|row| self.occupied(*row, self.q_col)) {
            moves.push((row, self.q_col))
        }

        // NE
        let mut col = self.q_col;
        let mut row = self.q_row;

        loop {
            if row == 0 || col == 7 {
                break;
            }

            col += 1;
            row -= 1;

            if self.occupied(row, col) {
                moves.push((row, col));
                break;
            }
        }

        // SE
        let mut col = self.q_col;
        let mut row = self.q_row;

        loop {
            if row == 7 || col == 7 {
                break;
            }

            col += 1;
            row += 1;

            if self.occupied(row, col) {
                moves.push((row, col));
                break;
            }
        }

        // SW
        let mut col = self.q_col;
        let mut row = self.q_row;

        loop {
            if row == 7 || col == 0 {
                break;
            }

            col -= 1;
            row += 1;

            if self.occupied(row, col) {
                moves.push((row, col));
                break;
            }
        }

        // NW
        let mut col = self.q_col;
        let mut row = self.q_row;

        loop {
            if row == 0 || col == 0 {
                break;
            }

            col -= 1;
            row -= 1;

            if self.occupied(row, col) {
                moves.push((row, col));
                break;
            }
        }

        moves
    }

    /// Prints the board
    fn print(&self) {
        for row in 0..8 {
            let rowstr: String = (0..8)
                .map(|col| {
                    if row == self.q_row && col == self.q_col {
                        '♛'
                    } else if self.occupied(row, col) {
                        '♟'
                    } else {
                        '·'
                    }
                })
                .collect();

            println!("{rowstr}");
        }
    }
}
