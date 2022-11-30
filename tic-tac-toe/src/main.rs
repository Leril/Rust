use std::fmt::{Display, Formatter};
use std::io::{stdin, stdout, Write};
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Player{
    X,
    O
}

impl Display for Player{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Player::X => "X",
            Player::O => "O"
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum BoardTileOption{
    X,
    O,
}

impl Display for BoardTileOption{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            BoardTileOption::X => "X",
            BoardTileOption::O => "O"
        })
    }
}

#[derive(Clone, Copy)]
struct Board{
    current_player: Option<Player>,
    board_state: [Option<BoardTileOption>; 9]
}

fn main() {
    println!("tic tac toe!");
    println!("Board squares are numbered as follows:");
    println!(
        "------------\n\
        | 1 | 2 | 3 |\n\
        -------------\n\
        | 4 | 5 | 6 |\n\
        -------------\n\
        | 7 | 8 | 9 |\n\
        -------------"
    );


    let mut board = Board{
        current_player: Some(Player::X),
        board_state: [None, None, None, None, None, None, None, None, None] };

    loop {
        let next_move = process_player_input(board);
        update_board(&mut board, next_move);
        print_board(board);
        if check_for_winner(board){
            break;
        }
    }

    println!("Winner is player {}", board.current_player.unwrap());
}

fn process_player_input(board: Board) -> i32{
    loop {
        print!("Player {} enter next move>>", board.current_player.unwrap_or(Player::O));

        stdout().flush().expect("Panicked flushing ??");

        let mut turn = String::new();

        stdin().read_line(&mut turn).expect("Panic started taking in the new turn ");

        let input = turn.trim().parse::<i32>();

        match input {
            Ok(num) => {
                if num - 1 > 9 || num - 1 < 0{
                    println!("Please enter a number between 1 and 9")
                }else {
                    return num - 1;
                }
            },
            Err(err) => {
                println!("Error {}", err);
            }
        }
    }
}

fn update_board(board: &mut Board, pos: i32){
    if board.board_state[pos as usize].is_some(){
        println!("That tile is taken!!");
        return;
    }

    if board.current_player.unwrap() == Player::X{
        board.board_state[pos as usize] = Option::from(BoardTileOption::X);
        board.current_player = Option::from(Player::O);
    }else{
        board.board_state[pos as usize] = Option::from(BoardTileOption::O);
        board.current_player = Option::from(Player::X);
    }
}

fn print_board(board: Board){
    let mut i = 0;
    let mut j = 0;

    //print the board
    print!("\n-------\n");
    while i < 3 {
        print!("|");
        while j < 3 {
            match board.board_state[i * 3 + j] {
                Some(p) => print!(" {}", p),
                None => print!("  ")
            }
            print!(" |");

            j += 1;
        }
        print!("\n-------\n");

        j = 0;
        i += 1;
    }
}

fn check_for_winner(board: Board) -> bool{
    for i in 0..3_usize{
        if board.board_state[i * 3] == board.board_state[i * 3 + 1] &&
            board.board_state[i * 3] == board.board_state[i * 3 + 2] &&
            board.board_state[i * 3].is_some(){
            return true;
        }
    }

    for i in 0..3_usize{
        if board.board_state[i] == board.board_state[i + 3] &&
            board.board_state[i] == board.board_state[i + 6] &&
            board.board_state[i].is_some(){
            return true;
        }
    }

    if board.board_state[0] == board.board_state[4] &&
        board.board_state[0] == board.board_state[8] &&
        board.board_state[0].is_some(){
        return true;
    }

    if board.board_state[2] == board.board_state[4] &&
        board.board_state[2] == board.board_state[6] &&
        board.board_state[2].is_some(){
        return true;
    }

    return false;
}
