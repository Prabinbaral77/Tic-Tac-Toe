use std::io::{self, Read};
mod board;

use crate::board::Board;

fn main() {
    let mut board = Board::new();
    let mut current_player = 'X';
    println!("Welcome to the Tic Tac Toe Game");
    board.print_board();

    loop {
        println!(
            "Player {}, Place your mark on board: (row)(col)",
            current_player
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let inputs: Vec<usize> = input
            .trim()
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        let row = inputs[0];
        let column = inputs[1];
        if inputs.len() != 2 && row > 2 && column > 2 {
            println!("Invalid input. Please enter row and column as two numbers between 0 and 2.");
            continue;
        }

        if !board.mark(row, column, current_player) {
            println!("Cell is already occupied! Try again.");
            continue;
        }

        board.print_board();

        if board.check_winner(current_player) {
            println!("Player {} won game.", current_player);
            break;
        }

        if board.is_team_full() {
            println!("Draw!");
            break;
        }

        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }
}
