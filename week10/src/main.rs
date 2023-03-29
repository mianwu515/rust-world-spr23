use std::io::{self, Write};

const BOARD_SIZE: usize = 9;

struct Board {
    board: [[u8; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn new() -> Self {
        Board {
            board: [[0; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    fn print(&self) {
        for i in 0..BOARD_SIZE {
            if i % 3 == 0 && i != 0 {
                println!("------+-------+------");
            }
            for j in 0..BOARD_SIZE {
                if j % 3 == 0 && j != 0 {
                    print!("| ");
                }
                print!("{} ", self.board[i][j]);
            }
            println!("");
        }
    }

    fn is_valid(&self, row: usize, col: usize, num: u8) -> bool {
        // Check row
        for j in 0..BOARD_SIZE {
            if self.board[row][j] == num {
                return false;
            }
        }

        // Check column
        for i in 0..BOARD_SIZE {
            if self.board[i][col] == num {
                return false;
            }
        }

        // Check box
        let box_row = (row / 3) * 3;
        let box_col = (col / 3) * 3;
        for i in 0..3 {
            for j in 0..3 {
                if self.board[box_row + i][box_col + j] == num {
                    return false;
                }
            }
        }

        true
    }

    fn solve(&mut self) -> bool {
        let mut empty_pos = None;
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.board[i][j] == 0 {
                    empty_pos = Some((i, j));
                    break;
                }
            }
            if empty_pos.is_some() {
                break;
            }
        }

        if let Some((row, col)) = empty_pos {
            for num in 1..=9 {
                if self.is_valid(row, col, num) {
                    self.board[row][col] = num;
                    if self.solve() {
                        return true;
                    }
                    self.board[row][col] = 0;
                }
            }
            false
        } else {
            true
        }
    }
}

fn main() {
    let mut board = Board::new();

    // Get input
    println!("Enter the initial board (0 for empty cells):");
    for i in 0..BOARD_SIZE {
        print!("Row {}: ", i + 1);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<u8> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for j in 0..BOARD_SIZE {
            board.board[i][j] = nums[j];
        }
    }

    // Solve and print
    if board.solve() {
        println!("Solution:");
        board.print();
    } else {
        println!("No solution found");
    }
}
