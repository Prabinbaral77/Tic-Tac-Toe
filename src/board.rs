pub struct Board {
    cells: [[char; 3]; 3],
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [[' '; 3]; 3],
        }
    }

    pub fn print_board(&self) {
        println!("************************Score board!************************");
        for row in &self.cells {
            println!("| {} | {} | {} |", row[0], row[1], row[2]);
            println!("--------------");
        }
    }

    pub fn mark(&mut self, row_index: usize, col_index: usize, symbol: char) -> bool {
        println!("Marking on the game board");
        if self.cells[row_index][col_index] == ' ' {
            self.cells[row_index][col_index] = symbol;
            return true;
        }
        return false;
    }

    pub fn check_winner(&self, mark: char) -> bool {
        for i in 0..3 {
            // for row winner
            if self.cells[i][0] == mark && self.cells[i][1] == mark && self.cells[i][2] == mark {
                return true;
            }
            // for column winner
            if self.cells[0][i] == mark && self.cells[1][i] == mark && self.cells[2][i] == mark {
                return true;
            }
        }
        //for diagonal winner
        if (self.cells[0][0] == mark && self.cells[1][1] == mark && self.cells[2][2] == mark)
            || (self.cells[2][0] == mark && self.cells[1][1] == mark && self.cells[0][2] == mark)
        {
            return true;
        }

        return false;
    }

    pub fn is_team_full(&self) -> bool {
        for row in &self.cells {
            for &cell in row {
                if cell == ' ' {
                    return false;
                }
            }
        }
        return true;
    }
}
