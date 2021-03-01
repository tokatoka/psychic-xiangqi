use crate::color::write_color;
use anyhow::Result;
use std::io::{stdin, BufRead /*BufReader*/};
use termcolor::Color;

#[derive(Copy, Clone)]
pub enum PieceType {
    None,
    Shuai,
    Shi,
    Xiang,
    Ma,
    Ju,
    Pao,
    Bing,
}

#[derive(Copy, Clone)]
pub enum MoveResult {
    Valid,
    Invalid,
    RedWin,
    GreenWin,
}

#[derive(Copy, Clone)]
pub enum Player {
    NoneRed,
    NoneGreen,
    Red,
    Green,
}

#[derive(Copy, Clone)]
pub struct Piece {
    piecetype: PieceType,
    player: Player,
}

impl Piece {
    pub fn new(piecetype: PieceType, player: Player) -> Piece {
        Piece {
            piecetype: piecetype,
            player: player,
        }
    }

    pub fn vacant(&self, board: &Board, row: usize, col: usize) -> bool {
        let a = (row <= 9) && (col <= 8);
        let b = match board.board[row][col].piecetype {
            PieceType::None => true,
            _ => false,
        };
        a && b
    }

    pub fn allowed(&self, board: &Board, row: usize, col: usize) -> bool {
        let a = (row <= 9) && (col <= 8);
        let b = match (board.board[row][col].player, self.player) {
            (Player::Red, Player::Red) | (Player::Green, Player::Green) => false,
            _ => true,
        };
        a && b
    }

    pub fn possible(&self, board: &Board, prow: usize, pcol: usize) -> Vec<[usize; 2]> {
        match self.piecetype {
            PieceType::Ju => {
                let mut v = vec![];
                for col in pcol..9 {
                    match (board.board[prow][col].player, self.player) {
                        (Player::NoneRed, _) | (Player::NoneGreen , _) => v.push([prow, col]),
                        (Player::Green, Player::Red) | (Player::Red, Player :: Green) => {
                            v.push([prow, col]);
                            break;
                        },
                        _ => break,
                    }
                }
                for col in (0..pcol).rev() {
                    match (board.board[prow][col].player, self.player) {
                        (Player::NoneRed, _) | (Player::NoneGreen , _) => v.push([prow, col]),
                        (Player::Green, Player::Red) | (Player::Red, Player :: Green) => {
                            v.push([prow, col]);
                            break;
                        },
                        _ => break,
                    }
                }
                for row in prow..8 {
                    match (board.board[row][pcol].player, self.player) {
                        (Player::NoneRed, _) | (Player::NoneGreen , _) => v.push([row, pcol]),
                        (Player::Green, Player::Red) | (Player::Red, Player :: Green) => {
                            v.push([row, pcol]);
                            break;
                        },
                        _ => break,
                    }
                }
                for row in (0..prow).rev() {
                    match (board.board[row][pcol].player, self.player) {
                        (Player::NoneRed, _) | (Player::NoneGreen, _) => v.push([row, pcol]),
                        (Player::Green, Player::Red) | (Player::Red, Player :: Green) => {
                            v.push([row, pcol]);
                            break;
                        },
                        _ => break,
                    }
                }
                v
            }
            PieceType::Ma => {
                let mut v = vec![];
                if self.vacant(board, prow + 1, pcol) {
                    if self.allowed(board, prow + 2, pcol + 1) {
                        v.push([prow + 2, pcol + 1])
                    }
                    if self.allowed(board, prow + 2, pcol - 1) {
                        v.push([prow + 2, pcol - 1])
                    }
                }
                if self.vacant(board, prow - 1, pcol) {
                    if self.allowed(board, prow - 2, pcol + 1) {
                        v.push([prow - 2, pcol + 1])
                    }
                    if self.allowed(board, prow - 2, pcol - 1) {
                        v.push([prow - 2, pcol - 1])
                    }
                }
                if self.vacant(board, prow, pcol + 1) {
                    if self.allowed(board, prow - 1, pcol + 2) {
                        v.push([prow - 1, pcol + 2])
                    }
                    if self.allowed(board, prow + 1, pcol + 2) {
                        v.push([prow + 1, pcol + 2])
                    }
                }
                if self.vacant(board, prow, pcol - 1) {
                    if self.allowed(board, prow - 1, pcol - 2) {
                        v.push([prow - 1, pcol - 2])
                    }
                    if self.allowed(board, prow + 1, pcol - 2) {
                        v.push([prow + 1, pcol - 2])
                    }
                }
                v
            }
            PieceType::Shi => {
                let mut v = vec![];
                match self.player {
                    Player::Red => {
                        if ((prow + 1 == 8 && pcol + 1 == 4)
                            || (prow + 1 == 8 && pcol - 1 == 4)
                            || (prow - 1 == 8 && pcol + 1 == 4)
                            || (prow - 1 == 8 && pcol - 1 == 4))
                            && self.allowed(board, 8, 4)
                        {
                            v.push([8, 4]);
                        }
                        if prow == 8 && pcol == 4 {
                            for [row, col] in vec![[9, 5], [9, 3], [7, 5], [7, 3]] {
                                if self.allowed(board, row, col) {
                                    v.push([row, col]);
                                }
                            }
                        }
                    }
                    Player::Green => {
                        if ((prow + 1 == 1 && pcol + 1 == 4)
                            || (prow + 1 == 1 && pcol - 1 == 4)
                            || (prow - 1 == 1 && pcol + 1 == 4)
                            || (prow - 1 == 1 && pcol - 1 == 4))
                            && self.allowed(board, 1, 4)
                        {
                            v.push([1, 4]);
                        }
                        if prow == 1 && pcol == 4 {
                            for [row, col] in vec![[2, 5], [2, 3], [0, 5], [0, 3]] {
                                if self.allowed(board, row, col) {
                                    v.push([row, col]);
                                }
                            }
                        }
                    }
                    _ => {}
                }
                v
            }
            PieceType::Xiang => {
                let mut v = vec![];
                match self.player {
                    Player::Red | Player::Green => {
                        if self.vacant(board, prow - 1, pcol - 1)
                            && self.allowed(board, prow - 2, pcol - 2)
                        {
                            v.push([prow - 2, pcol - 2]);
                        }
                        if self.vacant(board, prow + 1, pcol + 1)
                            && self.allowed(board, prow + 2, pcol + 2)
                        {
                            v.push([prow + 2, pcol + 2]);
                        }
                        if self.vacant(board, prow - 1, pcol + 1)
                            && self.allowed(board, prow - 2, pcol + 2)
                        {
                            v.push([prow - 2, pcol + 2]);
                        }
                        if self.vacant(board, prow + 1, pcol - 1)
                            && self.allowed(board, prow + 2, pcol - 2)
                        {
                            v.push([prow + 2, pcol - 2]);
                        }
                    }
                    _ => {}
                }
                v
            }
            PieceType::Bing => {
                let mut v = vec![];
                match self.player {
                    Player::Red => {
                        if prow <= 4 {
                            for [row, col] in
                                vec![[prow - 1, pcol], [prow, pcol - 1], [prow, pcol + 1]]
                            {
                                if self.allowed(&board, row, col) {
                                    v.push([row, col]);
                                }
                            }
                        } else if self.allowed(&board, prow - 1, pcol) {
                            v.push([prow - 1, pcol]);
                        }
                    }
                    Player::Green => {
                        if prow >= 5 {
                            for [row, col] in
                                vec![[prow + 1, pcol], [prow, pcol - 1], [prow, pcol + 1]]
                            {
                                if self.allowed(&board, row, col) {
                                    v.push([row, col]);
                                }
                            }
                        } else if self.allowed(&board, prow + 1, pcol) {
                            v.push([prow + 1, pcol]);
                        }
                    }
                    _ => {}
                }
                v
            }
            PieceType::Shuai => {
                let mut v = vec![];
                match self.player {
                    Player::Red => {
                        for [row, col] in vec![
                            [prow + 1, pcol],
                            [prow - 1, pcol],
                            [prow, pcol + 1],
                            [prow, pcol - 1],
                        ] {
                            if row >= 7
                                && row <= 9
                                && col >= 3
                                && col <= 5
                                && self.allowed(board, row, col)
                            {
                                v.push([row, col]);
                            }
                        }
                    }
                    Player::Green => {
                        for [row, col] in vec![
                            [prow + 1, pcol],
                            [prow - 1, pcol],
                            [prow, pcol + 1],
                            [prow, pcol - 1],
                        ] {
                            if row <= 2
                                && col >= 3
                                && col <= 5
                                && self.allowed(board, row, col)
                            {
                                v.push([row, col]);
                            }
                        }
                    }
                    _ => {}
                }
                v
            }
            PieceType::Pao => {
                let mut v = vec![];
                let mut flag = false;
                for col in pcol..9 {
                    if !flag {
                        match board.board[prow][col].player {
                            Player::NoneRed | Player::NoneGreen => v.push([prow, col]),
                            _ => {
                                flag = true;
                            }
                        }
                    } else {
                        match (board.board[prow][col].player, self.player) {
                            (Player::Red, Player::Green) | (Player::Green, Player::Red) =>  {
                                v.push([prow, col]);
                                break;
                            },
                            _  => break,
                        }
                    }
                }
                flag = false;
                for col in (0..pcol).rev() {
                    if !flag {
                        match board.board[prow][col].player {
                            Player::NoneRed | Player::NoneGreen => v.push([prow, col]),
                            _ => {
                                flag = true;
                            }
                        }
                    } else {
                        match (board.board[prow][col].player, self.player) {
                            (Player::Red, Player::Green) | (Player::Green, Player::Red) => { 
                                v.push([prow, col]);
                                break
                            },
                            _  => break,
                        }
                    }
                }
                flag = false;
                for row in prow..8 {
                    if !flag {
                        match board.board[row][pcol].player {
                            Player::NoneRed | Player::NoneGreen => v.push([row, pcol]),
                            _ => {
                                flag = true;
                            }
                        }
                    } else {
                        match (board.board[row][pcol].player, self.player) {
                            (Player::Red, Player::Green) | (Player::Green, Player::Red) => { 
                                v.push([row, pcol]);
                                break;
                            }
                            _  => break,
                        }
                    }
                }
                flag = false;
                for row in (0..prow).rev() {
                    if !flag {
                        match board.board[row][pcol].player {
                            Player::NoneRed | Player::NoneGreen => v.push([row, pcol]),
                            _ => {
                                flag = true;
                            }
                        }
                    } else {
                        match (board.board[row][pcol].player, self.player) {
                            (Player::Red, Player::Green) | (Player::Green, Player::Red) => {
                                v.push([row, pcol]);
                                break;
                            },
                            _  => break,
                        }
                    }
                }
                v
            }
            _ => {
                vec![]
            }
        }
    }

    pub fn show_piece(&self) {
        /*
        print out the piece on the board
        */
        match (self.piecetype, self.player) {
            (PieceType::Shuai, Player::Red) => write_color("帅", Some(Color::Red)).unwrap(),
            (PieceType::Shi, Player::Red) => write_color("仕", Some(Color::Red)).unwrap(),
            (PieceType::Xiang, Player::Red) => write_color("相", Some(Color::Red)).unwrap(),
            (PieceType::Ma, Player::Red) => write_color("傌", Some(Color::Red)).unwrap(),
            (PieceType::Ju, Player::Red) => write_color("俥", Some(Color::Red)).unwrap(),
            (PieceType::Pao, Player::Red) => write_color("炮", Some(Color::Red)).unwrap(),
            (PieceType::Bing, Player::Red) => write_color("兵", Some(Color::Red)).unwrap(),
            (PieceType::Shuai, Player::Green) => write_color("将", Some(Color::Green)).unwrap(),
            (PieceType::Shi, Player::Green) => write_color("士", Some(Color::Green)).unwrap(),
            (PieceType::Xiang, Player::Green) => write_color("象", Some(Color::Green)).unwrap(),
            (PieceType::Ma, Player::Green) => write_color("馬", Some(Color::Green)).unwrap(),
            (PieceType::Ju, Player::Green) => write_color("車", Some(Color::Green)).unwrap(),
            (PieceType::Pao, Player::Green) => write_color("砲", Some(Color::Green)).unwrap(),
            (PieceType::Bing, Player::Green) => write_color("卒", Some(Color::Green)).unwrap(),
            (_, Player::NoneRed) => write_color("..", Some(Color::White)).unwrap(),
            (_, Player::NoneGreen) => write_color("..", Some(Color::White)).unwrap(),
            _ => write_color("..", Some(Color::White)).unwrap(),
        }
    }
}

pub struct Board {
    pub board: [[Piece; 9]; 10],
}

impl Board {
    //const WIDTH: i32 = 30;
    //const HEIGHT: i32 = 26;

    pub fn new() -> Board {
        let board = Board {
            board: [
                [
                    Piece::new(PieceType::Ju, Player::Green),
                    Piece::new(PieceType::Ma, Player::Green),
                    Piece::new(PieceType::Xiang, Player::Green),
                    Piece::new(PieceType::Shi, Player::Green),
                    Piece::new(PieceType::Shuai, Player::Green),
                    Piece::new(PieceType::Shi, Player::Green),
                    Piece::new(PieceType::Xiang, Player::Green),
                    Piece::new(PieceType::Ma, Player::Green),
                    Piece::new(PieceType::Ju, Player::Green),
                ],
                [
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                ],
                [
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::Pao, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::Pao, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                ],
                [
                    Piece::new(PieceType::Bing, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::Bing, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::Bing, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::Bing, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::Bing, Player::Green),
                ],
                [
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                    Piece::new(PieceType::None, Player::Green),
                ],
                [
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                ],
                [
                    Piece::new(PieceType::Bing, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::Bing, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::Bing, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::Bing, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::Bing, Player::Red),
                ],
                [
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::Pao, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::Pao, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                ],
                [
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                    Piece::new(PieceType::None, Player::Red),
                ],
                [
                    Piece::new(PieceType::Ju, Player::Red),
                    Piece::new(PieceType::Ma, Player::Red),
                    Piece::new(PieceType::Xiang, Player::Red),
                    Piece::new(PieceType::Shi, Player::Red),
                    Piece::new(PieceType::Shuai, Player::Red),
                    Piece::new(PieceType::Shi, Player::Red),
                    Piece::new(PieceType::Xiang, Player::Red),
                    Piece::new(PieceType::Ma, Player::Red),
                    Piece::new(PieceType::Ju, Player::Red),
                ],
            ],
        };
        board
    }

    pub fn run(&mut self) -> Result<()> {
        let mut turn = Player::Red;
        let stdin = stdin();
        let reader = stdin.lock();
        for line in reader.lines() {
            let line = line?;
            let tokens = line.split_whitespace().collect::<Vec<_>>();
            match tokens.len() {
                5 => {
                    if tokens[0] == "move" {
                        let f1: usize = tokens[1].parse()?;
                        let f1 = f1 - 1;
                        let f2: usize = tokens[2].parse()?;
                        let f2 = f2 - 1;
                        let t1: usize = tokens[3].parse()?;
                        let t1 = t1 - 1;
                        let t2: usize = tokens[4].parse()?;
                        let t2 = t2 - 1;
                        match self.move_piece(turn, [f1, f2], [t1, t2]) {
                            MoveResult::Valid => {
                                match turn {
                                    Player::Red => {
                                        turn = Player::Green;
                                    }
                                    Player::Green => {
                                        turn = Player::Red;
                                    }
                                    _ => {

                                    }
                                }
                                self.show()
                            }
                            MoveResult::Invalid => {
                            }
                            MoveResult::RedWin => {
                                println!("Red has won");
                                break;
                            }
                            MoveResult::GreenWin => {
                                println!("Green has won");
                                break;
                            }
                        }
                    } else {
                        println!("Invalid Command");
                    }
                }
                _ => {
                    println!("Invalid Command");
                }
            }
        }
        Ok(())
    }

    pub fn show(&self) {
        println!("xxxxx 1  2  3  4  5  6  7  8  9xx");
        println!("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
        for row in 0..5 {
            print!("{:02} ", row + 1);
            print!("xx");
            for col in 0..9 {
                self.board[row][col].show_piece();
                if col != 8 {
                    print!(" ");
                }
            }
            println!("xx")
        }
        for row in 5..10 {
            print!("{:02} ", row + 1);
            print!("xx");
            for col in 0..9 {
                self.board[row][col].show_piece();
                if col != 8 {
                    print!(" ");
                }
            }
            println!("xx")
        }
        println!("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    }


    pub fn verify(&self, turn: Player, from: [usize; 2], to: [usize; 2]) -> MoveResult {
        let piece = self.board[from[0]][from[1]];
        match piece.piecetype {
            PieceType::None => {
                match write_color("Invalid Command: There's no piece!", Some(Color::White)) {
                    Ok(..) => {},
                    Err(..) => {},
                };
                return MoveResult::Invalid;
            }
            _ => {}
        };
        match (piece.player, turn) {
            (Player::Red, Player::Green) => {
                match write_color(
                    "Invalid Command: You cannot move the opponent's piece!",
                    Some(Color::White),
                ) {
                    Ok(..) => {},
                    Err(..) => {},
                };
                return MoveResult::Invalid;
            }
            (Player::Green, Player::Red) => {
                match write_color(
                    "Invalid Command: You cannot move the opponent's piece!",
                    Some(Color::White),
                ) {
                    Ok(..) => {},
                    Err(..) => {},
                };
                return MoveResult::Invalid;
            }
            _ => {}
        }
        let possible_poses = piece.possible(&self, from[0], from[1]);
        match (piece.piecetype, piece.player) {
            (PieceType::Shuai, Player::Red) => {
                if possible_poses.len() == 0 {
                    match write_color("Green Win!!!", Some(Color::White)) {
                        Ok(..) => {},
                        Err(..) => {},
                    };
                    return MoveResult::GreenWin;
                }
            }
            (PieceType::Shuai, Player::Green) => {
                if possible_poses.len() == 0 {
                    match write_color("Red Win!!!", Some(Color::White)) {
                        Ok(..) => {},
                        Err(..) => {},
                    };
                    return MoveResult::RedWin;
                }
            }
            _ => {}
        }
        if !possible_poses.contains(&to) {
            println!(
                "Invalid Command: You cannot move the piece to ({}, {}).",
                to[0] + 1, to[1] + 1
            );
            return MoveResult::Invalid;
        }
        return MoveResult::Valid;
    }
    // 記譜を戻すための関数
    pub fn reverse_piece(&self) -> MoveResult {
        return MoveResult::Valid;
    }

    pub fn move_piece(&mut self, turn: Player, from: [usize; 2], to: [usize; 2]) -> MoveResult {
        let isvalid = self.verify(turn, from, to);
        match isvalid {
            MoveResult::Valid => {
                self.board[to[0]][to[1]] = self.board[from[0]][from[1]];
                self.board[from[0]][from[1]] = Piece::new(PieceType::None, Player::NoneGreen);
                return MoveResult::Valid;
            }
            MoveResult::Invalid => return MoveResult::Invalid,
            MoveResult::RedWin => return MoveResult::RedWin,
            MoveResult::GreenWin => return MoveResult::GreenWin,
        }
    }
}
