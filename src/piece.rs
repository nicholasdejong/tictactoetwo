#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    X = 1,
    O = -1,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::O => write!(f, "o"),
            Piece::X => write!(f, "x"),
        }
    }
}

impl std::ops::Not for Piece {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::O => Self::X,
            Self::X => Self::O,
        }
    }
}

impl<'a> std::ops::Not for &'a Piece {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Piece::O => &Piece::X,
            Piece::X => &Piece::O,
        }
    }
}
