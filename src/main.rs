use board::{Board};

fn main() {
    //println!("Hello, world!");
    let mut board = Board::new();
    board.show();
    match board.run() {
        Ok(..) => {
            println!("Finished correctly!!")
        },
        Err(..) => {
            println!("Error occured!!")
        }
    }
}

#[test]
fn test(){


}

pub mod board;
pub mod color;