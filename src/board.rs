use crate::cli::Coord;
use crate::eval::Eval;
use crate::piece::Piece;

#[derive(Debug, Clone, Copy)]
pub struct Move(usize, Eval);

impl Move {
    pub fn index(&self) -> usize {
        self.0
    }

    pub fn eval(&self) -> Eval {
        self.1
    }
}

#[derive(Default, PartialEq, Debug, Clone, Copy)]
pub struct Board(pub [Option<Piece>; 9]);

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from("  ┌───┬───┬───┐\na │");
        for (index, piece) in self.0.iter().enumerate() {
            let piece = match piece {
                Some(p) => format!("{p}"),
                None => String::from(" "),
            };
            if (index + 1) % 3 == 0 && index != self.0.len() - 1 {
                result = format!(
                    "{result} {piece} │\n  ├───┼───┼───┤\n{} │",
                    Coord::from(index + 1).row()
                );
            } else {
                result = format!("{result} {piece} │");
            }
        }
        write!(f, "{result}\n  └───┴───┴───┘\n    1   2   3")
    }
}

impl Board {
    pub fn full(&self) -> bool {
        !self.0.contains(&None)
    }

    pub fn winner(&self) -> Option<Piece> {
        match self.0 {
            [Some(a), Some(b), Some(c), _, _, _, _, _, _] if a == b && b == c => Some(a),
            [_, _, _, Some(a), Some(b), Some(c), _, _, _] if a == b && b == c => Some(a),
            [_, _, _, _, _, _, Some(a), Some(b), Some(c)] if a == b && b == c => Some(a),
            [Some(a), _, _, Some(b), _, _, Some(c), _, _] if a == b && b == c => Some(a),
            [_, Some(a), _, _, Some(b), _, _, Some(c), _] if a == b && b == c => Some(a),
            [_, _, Some(a), _, _, Some(b), _, _, Some(c)] if a == b && b == c => Some(a),
            [Some(a), _, _, _, Some(b), _, _, _, Some(c)] if a == b && b == c => Some(a),
            [_, _, Some(a), _, Some(b), _, Some(c), _, _] if a == b && b == c => Some(a),
            _ => None,
        }
    }

    pub fn open(&self) -> Vec<usize> {
        let mut open: Vec<usize> = vec![];
        for (index, sq) in self.0.iter().enumerate() {
            if sq.is_none() {
                open.push(index);
            }
        }
        open
    }

    pub fn turn(&self) -> Piece {
        if self.open().len() % 2 == 0 {
            return Piece::O;
        }
        Piece::X
    }

    pub fn moves(&self) -> Vec<(usize, Self)> {
        let mut moves: Vec<(usize, Self)> = vec![];
        for sq in self.open() {
            let mut new = Board(self.0);
            new.0[sq] = Some(self.turn());
            moves.push((sq, new));
        }
        moves
    }

    pub fn eval(&self) -> Eval {
        let score = match self.winner() {
            Some(Piece::X) => Eval::Winning(0),
            Some(Piece::O) => Eval::Losing(0),
            None => Eval::Draw,
        };
        return match self.turn() {
            Piece::X => score,
            Piece::O => -score,
        };
    }

    fn negamax(&self) -> Eval {
        let mut best = None::<Eval>;
        if self.full() || self.winner().is_some() {
            return self.eval();
        }
        for (_, position) in self.moves() {
            let score = -position.negamax().increment();
            if let Some(e) = best {
                if score > e {
                    best = Some(score);
                }
            } else {
                best = Some(score);
            }
        }
        return best.expect("Can't call negamax on full board.");
    }

    pub fn search(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        for (index, position) in self.moves() {
            moves.push(Move(index, -position.negamax().increment()));
        }
        moves.sort_unstable_by(|a, b| {
            b.eval()
                .partial_cmp(&a.eval())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        moves
    }
}

/// Initializes a TicTacToe board.
/// ```
/// let b = board!(. . . . . . . . .);
/// assert_eq!(b, Board::default());
///
/// let b = board!(X O X O X O X O X);
/// assert!(b.full() && b.winner() == Some(Piece::X));
///
/// ```
/// Crosses are represented with X or x.
/// Naughts are represented with O or o.
/// ```
/// assert!(board!(X . O . X . O . X), board!(x . o . x . o . x));
/// ```
#[macro_export]
macro_rules! board {
    (@piece X) => {
        Some($crate::piece::Piece::X)
    };
    (@piece x) => {
        Some($crate::piece::Piece::X)
    };
    (@piece O) => {
        Some($crate::piece::Piece::O)
    };
    (@piece o) => {
        Some($crate::piece::Piece::O)
    };
    (@piece $t:tt) => {
        None::<$crate::piece::Piece>
    };
    ($($t:tt)*) => {{
        const BOARD: $crate::board::Board = {
            let mut b = [None; 9];
            let mut index = 0;
            $(
                let piece = board!(@piece $t);
                if index < 9 {
                    b[index] = piece;
                }
                index += 1;
            )*
            let _ = index;
            $crate::board::Board(b)
        };
        BOARD
    }};
}
