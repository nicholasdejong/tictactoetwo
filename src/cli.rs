use crate::{
    board::{Board, Move},
    eval::Eval,
    piece::Piece,
};
use colored::{ColoredString, Colorize};
use rand::prelude::*;

pub struct Coord(String);

impl From<usize> for Coord {
    fn from(index: usize) -> Self {
        Self(format!(
            "{}{}",
            (index as u8 / 3 + 97) as char,
            index % 3 + 1
        ))
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Coord {
    pub fn row(&self) -> char {
        self.0.as_str().chars().next().unwrap()
    }

    pub fn col(&self) -> usize {
        self.0
            .as_str()
            .chars()
            .nth(1)
            .unwrap()
            .to_digit(10)
            .expect("Invalid user input.") as usize
    }

    pub fn index(&self) -> usize {
        return ((self.row() as usize - 1) % 3) * 3 + self.col() - 1;
    }
}

pub fn color_move(m: &Move) -> ColoredString {
    let (index, eval) = (m.index(), m.eval());
    let coord = Coord::from(index);
    let colored_string = match eval {
        Eval::Winning(_) => coord.to_string().green(),
        Eval::Losing(_) => coord.to_string().red(),
        Eval::Draw => coord.to_string().normal(),
    };
    colored_string
}

pub fn print_state(b: &Board) {
    println!("   Tic Tac Toe");
    println!("{b}");
    let search = b.search();
    println!("\nEvaluation: {}", search[0].eval().str(&b.turn()));
    let mut moves = search
        .iter()
        .map(|m| format!("{}, ", color_move(m)))
        .collect::<String>();
    moves.pop();
    moves.pop();
    println!("Available moves: ({moves})");
}

pub fn init() {
    let mut rng = thread_rng();
    let mut b = Board::default();
    let mut ln = String::new();
    println!("Tic Tac Toe. Choose Player or Computer to go first: ");
    std::io::stdin()
        .read_line(&mut ln)
        .expect("Failed to read user input.");

    let is_maximizing = match ln.to_lowercase().chars().next() {
        Some(ch) => ch == 'p',
        None => panic!("Invalid user input"),
    };
    let computer = if is_maximizing { Piece::O } else { Piece::X };
    while !b.full() {
        if b.winner().is_some() {
            println!("{b}");
            println!("{} wins.", b.winner().unwrap().to_string().to_uppercase());
            break;
        }
        if b.turn() == computer {
            let tryhard = rng.gen_bool(0.75); // Tryhard 75% of the time
            let moves = b.search();
            let mv = if tryhard {
                moves[0].index()
            } else {
                moves[rng.gen_range(0..moves.len())].index()
            };
            b.0[mv] = Some(b.turn());
        } else {
            println!("{esc}c", esc = 27 as char);
            print_state(&b);
            let mut ln = String::new();
            println!("Choose a move from the list: ");
            std::io::stdin()
                .read_line(&mut ln)
                .expect("Failed to read user input.");
            let index = Coord(ln).index();
            if b.0[index].is_some() {
                panic!("Invalid move.");
            } else {
                b.0[index] = Some(b.turn());
            }
        }
    }
    if b.winner().is_none() {
        println!("{b}");
        println!("The game is a draw.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        for i in 0..9 {
            assert_eq!(Coord::from(i).index(), i);
        }
    }
}
