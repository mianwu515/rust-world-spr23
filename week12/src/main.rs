// This program generates a random Sudoku puzzle, then allows the user to solve it by entering numbers into the grid. The user can move the cursor around the grid using the arrow keys, and can delete a number by pressing backspace. When the user has entered a valid solution, the program congratulates them and exits.

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{cursor, event, execute, queue, style, terminal};
use crossterm::cursor::MoveTo;
use rand::Rng;
use std::fmt;
use std::io::{self, Write};

const BOARD_SIZE: usize = 9;

struct Board {
    board: [[u8; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn new() -> Self {
        Self {
            board: [[0; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    fn print(&self, cursor_pos: (u16, u16)) {
        for (i, row) in self.board.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let ch = if cell != 0 {
                    format!("{}", cell)
                } else {
                    " ".to_owned()
                };
                let color = if self.is_conflict(i, j) {
                    style::Color::Red
                } else {
                    style::Color::White
                };
                execute!(
                    io::stdout(),
                    MoveTo(cursor_pos.0 + j as u16 * 2, cursor_pos.1 + i as u16),
                    style::PrintStyledContent(style::StyledContent::new(ch).with(color)),
                )
                .unwrap();
            }
        }
        io::stdout().flush().unwrap();
    }

    fn is_valid(&self, row: usize, col: usize, num: u8) -> bool {
        for i in 0..BOARD_SIZE {
            if self.board[row][i] == num {
                return false;
            }
            if self.board[i][col] == num {
                return false;
            }
            let box_row = row / 3 * 3 + i / 3;
            let box_col = col / 3 * 3 + i % 3;
            if self.board[box_row][box_col] == num {
                return false;
            }
        }
        true
    }

    fn is_conflict(&self, row: usize, col: usize) -> bool {
        let num = self.board[row][col];
        if num == 0 {
            return false;
        }
        for i in 0..BOARD_SIZE {
            if i != col && self.board[row][i] == num {
                return true;
            }
            if i != row && self.board[i][col] == num {
                return true;
            }
            let box_row = row / 3 * 3 + i / 3;
            let box_col = col / 3 * 3 + i % 3;
            if box_row != row && box_col != col && self.board[box_row][box_col] == num {
                return true;
            }
        }
        false
    }

    fn is_solved(&self) -> bool {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.board[i][j] == 0 || self.is_conflict(i, j) {
                    return false;
                }
            }
        }
        true
    }

    fn set_cell(&mut self, row: usize, col: usize, num: u8) {
        self.board[row][col] = num;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.board.iter().enumerate() {
            if i % 3 == 0 {
                writeln!(f, "+-------+-------+-------+").unwrap();
            }
            for (j, &cell) in row.iter().enumerate() {
                if j % 3 == 0 {
                    write!(f, "| ")?;
                }
                if cell != 0 {
                    write!(f, "{} ", cell)?;
                } else {
                    write!(f, "  ")?;
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+-------+-------+-------+")
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut board = Board::new();
    // Generate a random puzzle
    for i in 0..BOARD_SIZE {
        let mut nums: Vec<u8> = (1..=9).collect();
        rng.shuffle(&mut nums);
        for j in 0..BOARD_SIZE {
            let num = nums.pop().unwrap();
            if !board.is_valid(i, j, num) {
                nums.insert(0, num);
                continue;
            }
            board.set_cell(i, j, num);
        }
    }

    let mut cursor_pos = (0, 0);
    let mut selected_cell = (0, 0);

    execute!(
        io::stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
    )
    .unwrap();

    loop {
        board.print(cursor_pos);
        queue!(
            io::stdout(),
            MoveTo(
                cursor_pos.0 + selected_cell.1 as u16 * 2,
                cursor_pos.1 + selected_cell.0 as u16
            ),
            style::SetForegroundColor(style::Color::Black),
            style::SetBackgroundColor(style::Color::White),
            style::Print("  "),
            style::ResetColor,
        )
        .unwrap();
        io::stdout().flush().unwrap();

        if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
            match code {
                KeyCode::Char('1')
                | KeyCode::Char('2')
                | KeyCode::Char('3')
                | KeyCode::Char('4')
                | KeyCode::Char('5')
                | KeyCode::Char('6')
                | KeyCode::Char('7')
                | KeyCode::Char('8')
                | KeyCode::Char('9') => {
                    let num = code as u8 - b'0';
                    if board.is_valid(selected_cell.0, selected_cell.1, num) {
                        board.set_cell(selected_cell.0, selected_cell.1, num);
                        selected_cell.1 += 1;
                        if selected_cell.1 == BOARD_SIZE {
                            selected_cell.0 += 1;
                            selected_cell.1 = 0;
                            if selected_cell.0 == BOARD_SIZE {
                                selected_cell.0 = 0;
                            }
                        }
                    }
                }
                KeyCode::Backspace => {
                    board.set_cell(selected_cell.0, selected_cell.1, 0);
                    if selected_cell.1 > 0 {
                        selected_cell.1 -= 1;
                    } else {
                        selected_cell.0 -= 1;
                        if selected_cell.0 == usize::max_value() {
                            selected_cell.0 = BOARD_SIZE - 1;
                        }
                        selected_cell.1 = BOARD_SIZE - 1;
                    }
                }
                KeyCode::Enter => {
                    if board.is_solved() {
                        execute!(
                            io::stdout(),
                            cursor::Show,
                            terminal::Clear(terminal::ClearType::All),
                            MoveTo(0, 0),
                            style::Print("Congratulations, you solved the puzzle!"),
                        )
                        .unwrap();
                        break;
                    } else {
                        execute!(
                            io::stdout(),
                            cursor::Hide,
                            MoveTo(0, 0),
                            style::Print("The puzzle is not solved yet. Keep trying!"),
                        )
                        .unwrap();
                    }
                }
                KeyCode::Up => {
                    if selected_cell.0 > 0 {
                        selected_cell.0 -= 1;
                    } else {
                        selected_cell.0 = BOARD_SIZE - 1;
                    }
                }
                KeyCode::Down => {
                    selected_cell.0 += 1;
                    if selected_cell.0 == BOARD_SIZE {
                        selected_cell.0 = 0;
                    }
                }
                KeyCode::Left => {
                    if selected_cell.1 > 0 {
                        selected_cell.1 -= 1;
                    } else {
                        selected_cell.1 = BOARD_SIZE - 1;
                    }
                }
                KeyCode::Right => {
                    selected_cell.1 += 1;
                    if selected_cell.1 == BOARD_SIZE {
                        selected_cell.1 = 0;
                    }
                }
                _ => {}
            }
        }
    }
}
