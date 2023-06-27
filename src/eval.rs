use crate::piece::Piece;

/// The Eval enum is used to keep track of DTM (distance to mate).
#[derive(Debug, Clone, Copy)]
pub enum Eval {
    Draw,
    Winning(usize),
    Losing(usize),
}

impl std::ops::Neg for Eval {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Eval::Draw => Eval::Draw,
            Eval::Winning(x) => Eval::Losing(x),
            Eval::Losing(x) => Eval::Winning(x),
        }
    }
}

impl std::cmp::PartialEq for Eval {
    fn eq(&self, other: &Self) -> bool {
        self._num() == other._num()
    }
    fn ne(&self, other: &Self) -> bool {
        self._num() != other._num()
    }
}

impl std::cmp::PartialOrd for Eval {
    fn ge(&self, other: &Self) -> bool {
        self._num() >= other._num()
    }
    fn gt(&self, other: &Self) -> bool {
        self._num() > other._num()
    }
    fn le(&self, other: &Self) -> bool {
        self._num() <= other._num()
    }
    fn lt(&self, other: &Self) -> bool {
        self._num() < other._num()
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self._num().partial_cmp(&other._num())
    }
}

impl Eval {
    pub fn increment(&self) -> Eval {
        match self {
            Eval::Draw => Eval::Draw,
            Eval::Winning(x) => Eval::Winning(x + 1),
            Eval::Losing(x) => Eval::Losing(x + 1),
        }
    }

    fn _num(&self) -> isize {
        match self {
            Eval::Draw => 0,
            Eval::Winning(x) => 10 - *x as isize,
            Eval::Losing(x) => *x as isize - 10,
        }
    }

    pub fn str(&self, p: &Piece) -> String {
        match self {
            Self::Draw => "The game is a draw.".to_string(),
            Self::Losing(m) => format!(
                "{} loses in {} move{}.",
                p.to_string().to_uppercase(),
                m / 2 + m % 2,
                if m > &2 { "s" } else { "" }
            ),
            Self::Winning(m) => format!(
                "{} wins in {} move{}.",
                p.to_string().to_uppercase(),
                m / 2 + m % 2,
                if m > &2 { "s" } else { "" }
            ),
        }
    }
}
