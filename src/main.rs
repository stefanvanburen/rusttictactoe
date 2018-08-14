extern crate rand;
use rand::distributions::{IndependentSample, Range};
use std::io;

static COMPUTER_CHAR: char = 'O';
static USER_CHAR: char = 'X';

fn main() {
    play_game();
}

fn play_game() {
    println!("Tic Tac Toe!!");
    let mut board = reset_board();
    let mut computer_won = false;

    print_game_board(&board);
    loop {
        get_user_input(&mut board);
        print_game_board(&board);
        if check_for_winner(&board) {
            break;
        }
        make_computer_move(&mut board);
        print_game_board(&board);
        if check_for_winner(&board) {
            computer_won = true;
            break;
        }
    }

    println!("Game over!");

    if computer_won {
        println!("The computer won.");
    } else {
        println!("You won.");
    }

    loop {
        println!("Play again? Y/N");
        let mut user_move_str = String::new();
        io::stdin()
            .read_line(&mut user_move_str)
            .expect("failed to read line");
        if user_move_str.trim() == "Y" {
            play_game();
        } else if user_move_str.trim() == "N" {
            break;
        }
    }
}

fn reset_board() -> [char; 9] {
    ['1', '2', '3', '4', '5', '6', '7', '8', '9']
}

fn get_user_input(v: &mut [char; 9]) {
    println!("Please type a number 1-9 for your move");
    let mut user_move_str = String::new();
    io::stdin()
        .read_line(&mut user_move_str)
        .expect("failed to read line");
    let user_move: i32 = user_move_str.trim().parse().expect("Please type a number!");
    if v[(user_move - 1) as usize] == COMPUTER_CHAR || v[(user_move - 1) as usize] == USER_CHAR {
        println!("Please select a cell that hasn't been selected yet.");
        get_user_input(v);
    } else {
        v[(user_move - 1) as usize] = USER_CHAR;
    }
}

fn print_game_board(v: &[char; 9]) {
    println!("----------------------------------------");
    println!("\t{}|{}|{}", v[0], v[1], v[2]);
    println!("\t{}|{}|{}", v[3], v[4], v[5]);
    println!("\t{}|{}|{}", v[6], v[7], v[8]);
    println!("----------------------------------------");
    println!();
}

fn make_computer_move(v: &mut [char; 9]) {
    let mut made_a_move = false;
    while !made_a_move {
        let mut rng = rand::thread_rng();
        let range = Range::new(0, 9);
        let rand = range.ind_sample(&mut rng);
        if v[(rand) as usize] != COMPUTER_CHAR && v[(rand) as usize] != USER_CHAR {
            v[(rand) as usize] = COMPUTER_CHAR;
            println!("Computer put an {} at {}.", COMPUTER_CHAR, rand + 1);
            println!();
            made_a_move = true;
        }
    }
}

fn check_for_winner(v: &[char; 9]) -> bool {
    (v[0] == v[1] && v[1] == v[2])
        || (v[0] == v[3] && v[3] == v[6])
        || (v[0] == v[4] && v[4] == v[8])
        || (v[1] == v[4] && v[4] == v[7])
        || (v[2] == v[4] && v[4] == v[6])
        || (v[3] == v[4] && v[4] == v[5])
        || (v[2] == v[5] && v[5] == v[8])
        || (v[6] == v[7] && v[7] == v[8])
}
