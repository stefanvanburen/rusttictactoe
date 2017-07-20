extern crate rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use std::io;

static computerChar: char = 'O';
static userChar: char = 'X';

fn main() {
    playGame();
}

fn playGame()
{
    println!("Tic Tac Toe!!");
    let mut board = resetBoard();
    let mut gameOver = false;
    let mut computerWon = false;
    let mut userWon = false;

    printGameBoard(&board);
    while(!gameOver)
    {
      getUserInput(&mut board);
      printGameBoard(&board);
      userWon = checkForWinner(&board);
      if(userWon)
      {
          gameOver = true;
          break;
      }
      makeComputerMove(&mut board);
      printGameBoard(&board);
      computerWon = checkForWinner(&board);
      if(computerWon)
      {
          gameOver = true;
          break;
      }
    }


    println!("Game over!");

    if(computerWon)
    {
      println!("The computer won.");
    }
    else
    {
      println!("You won.");
    }
    let mut done = false;
    while(!done)
    {
      println!("Play again? Y/N");
      let mut userMoveStr = String::new();
      io::stdin().read_line(&mut userMoveStr)
          .expect("failed to read line");
      if(userMoveStr.trim() == "Y")
      {
        done = true;
        playGame();
      }
      else if(userMoveStr.trim() == "N")
      {
        done = true;
        break;
      }
    }

}
fn resetBoard() -> [char; 9]
{
    return ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
}

fn getUserInput(v:&mut [char; 9])
{
    println!("Please type a number 1-9 for your move");
    let mut userMoveStr = String::new();
    io::stdin().read_line(&mut userMoveStr)
        .expect("failed to read line");
    let userMove: i32 = userMoveStr.trim().parse()
    .expect("Please type a number!");
    if(v[(userMove - 1) as usize] == computerChar || v[(userMove - 1) as usize] == userChar)
    {
        println!("Please select a cell that hasn't been selected yet.");
        getUserInput(v);
    }
    else
    {
        v[(userMove - 1) as usize] = userChar;
    }
}

fn printGameBoard(v: &[char; 9])
{
    println!("----------------------------------------");
    println!("\t{}|{}|{}", v[0], v[1], v[2]);
    println!("\t{}|{}|{}", v[3], v[4], v[5]);
    println!("\t{}|{}|{}", v[6], v[7], v[8]);
    println!("----------------------------------------");
    println!("");
}

fn makeComputerMove(v: &mut [char; 9])
{
    let mut madeAMove = false;
    while(!madeAMove)
    {
      let mut rng = rand::thread_rng();
      let range = Range::new(0, 9);
      let mut rand = range.ind_sample(&mut rng);
      if(v[(rand) as usize] != computerChar && v[(rand) as usize] != userChar)
      {
        v[(rand) as usize] = computerChar;
        println!("Computer put an {} at {}.", computerChar, rand + 1);
        println!("");
        madeAMove = true;
      }
    }
}

fn checkForWinner(v: &[char; 9]) -> bool
{
    return ((v[0] == v[1] && v[1] == v[2]) ||
            (v[0] == v[3] && v[3] == v[6]) ||
            (v[0] == v[4] && v[4] == v[8]) ||
            (v[1] == v[4] && v[4] == v[7]) ||
            (v[2] == v[4] && v[4] == v[6]) ||
            (v[3] == v[4] && v[4] == v[5]) ||
            (v[2] == v[5] && v[5] == v[8]) ||
            (v[6] == v[7] && v[7] == v[8]))

}
