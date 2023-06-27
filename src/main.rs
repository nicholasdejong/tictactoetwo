mod board;
mod cli;
mod eval;
mod piece;

fn main() {
    cli::init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_full() {
        assert!(board!(X X X O O O X X X).full())
    }

    #[test]
    fn winner_x() {
        //rows
        assert_eq!(board!(X X X . . . . . .).winner(), Some(piece::Piece::X));
        assert_eq!(board!(. . . X X X . . .).winner(), Some(piece::Piece::X));
        assert_eq!(board!(. . . . . . X X X).winner(), Some(piece::Piece::X));
        //columns
        assert_eq!(board!(X . . X . . X . .).winner(), Some(piece::Piece::X));
        assert_eq!(board!(. X . . X . . X .).winner(), Some(piece::Piece::X));
        assert_eq!(board!(. . X . . X . . X).winner(), Some(piece::Piece::X));
        //diagonals
        assert_eq!(board!(X . . . X . . . X).winner(), Some(piece::Piece::X));
        assert_eq!(board!(. . X . X . X . .).winner(), Some(piece::Piece::X))
    }
}
