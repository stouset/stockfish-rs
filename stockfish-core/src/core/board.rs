use crate::prelude::*;

use core::iter::IntoIterator;
use core::ops::{Index, IndexMut};

/// A [`Board`] is an array of [`Square`]s (A1, B1, C1, ..., F8, G8, H8) that
/// may optionally contain a [`Piece`].
#[derive(Copy, Eq, PartialEq)]
#[derive_const(Clone)]
#[must_use]
pub struct Board([Option<Piece>; Square::COUNT]);

impl Board {
    /// An empty chess board containing no [`Piece`]s.
    pub const EMPTY: Self = Self([None; Square::COUNT]);

    /// Returns an iterator over all occupied squares on the board.
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn iter(&self) -> impl Iterator<Item = (Square, Piece)> + '_ {
        Square::iter().filter_map(|s| self[s].map(|t| (s, t)))
    }

    /// If the given [`Piece`] is on the [`Board`], returns the [`Square`] it
    /// resides on. If no such token exists, returns [`None`].
    ///
    /// Note that this function by necessity iterates over every [`Square`]
    /// until the token is found. It should be used judiciously (or not at all)
    /// in performance-sensitive situations.
    #[allow(clippy::missing_inline_in_public_items)]
    #[must_use]
    pub fn search<I: IntoIterator<Item = Square>>(&self, squares: I, piece: Piece) -> Option<Square> {
        squares.into_iter().find(|s| self[*s] == Some(piece))
    }
}

impl const Default for Board {
    #[inline]
    fn default() -> Self {
        Self::EMPTY
    }
}

// TODO: this is an annoying detail to expose and breaks the abstraction, but it
// allows for a convenient implementation of parsing a chess board from FEN
impl const Index<usize> for Board {
    type Output = Option<Piece>;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

// TODO: this is an annoying detail to expose and breaks the abstraction, but it
// allows for a convenient implementation of parsing a chess board from FEN
impl const IndexMut<usize> for Board {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl const Index<Square> for Board {
    type Output = Option<Piece>;

    #[inline]
    fn index(&self, index: Square) -> &Self::Output {
        self.index(index.as_usize())
    }
}

impl const IndexMut<Square> for Board {
    #[inline]
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        self.index_mut(index.as_usize())
    }
}

impl core::fmt::Debug for Board {
    #[allow(clippy::missing_inline_in_public_items)]
    #[cfg_attr(coverage, no_coverage)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f)?;
        writeln!(f, "  +---+---+---+---+---+---+---+---+")?;

        for rank in Rank::iter().rev() {
            write!(f, "{} |", char::from(rank))?;

            for file in File::iter() {
                let square = Square::new(file, rank);
                let piece  = self[square];

                match piece {
                    Some(t) => write!(f, " {} |", char::from(t))?,
                    None    => write!(f, "   |")?,
                }
            }

            writeln!(f)?;

            writeln!(f, "  +---+---+---+---+---+---+---+---+")?;
        }

        writeln!(f, "    A   B   C   D   E   F   G   H")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives() {
        let board = board!(
            r n b q k b n r
            p p p p p p p p
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            P P P P P P P P
            R N B Q K B N R
        );

        assert_eq!(board, board.clone());
    }

    #[test]
    fn debug() {
        let board = board!(
            r n b q k b n r
            p p p p p p p p
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            P P P P P P P P
            R N B Q K B N R
        );

        assert_eq!(
            concat!(
                "\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "8 | r | n | b | q | k | b | n | r |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "7 | p | p | p | p | p | p | p | p |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "6 |   |   |   |   |   |   |   |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "5 |   |   |   |   |   |   |   |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "4 |   |   |   |   |   |   |   |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "3 |   |   |   |   |   |   |   |   |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "2 | P | P | P | P | P | P | P | P |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "1 | R | N | B | Q | K | B | N | R |\n",
                "  +---+---+---+---+---+---+---+---+\n",
                "    A   B   C   D   E   F   G   H\n",
            ),

            format!("{board:?}"),
        );
    }

    #[test]
    fn iter() {
        let board = board!(
            _ _ b _ _ _ k r
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            p _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ K _ _ _
        );

        assert_eq!(
            vec![
                (Square::E1, Piece::WhiteKing),
                (Square::A5, Piece::BlackPawn),
                (Square::C8, Piece::BlackBishop),
                (Square::G8, Piece::BlackKing),
                (Square::H8, Piece::BlackRook),
            ],

            board.iter().collect::<Vec<(Square, Piece)>>()
        );
    }

    #[test]
    fn search() {
        let board = board!(
            r n b q k b n r
            p p p p p p p p
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            P P P P P P P P
            R N B Q K B N R
        );

        assert_eq!(Some(Square::B2), board.search([Square::B2],         Piece::WhitePawn));
        assert_eq!(Some(Square::A7), board.search(Square::iter(),       Piece::BlackPawn));
        assert_eq!(None,             board.search(Rank::_2.into_iter(), Piece::BlackKing));
    }
}

/// Allows constructing a [`Board`] from a human-readable format.
///
/// # Example:
///
/// ```rust
/// use stockfish_core::prelude::*;
/// use stockfish_core::board;
///
/// board!(
///     r n b q k b n r
///     p p p p p p p p
///     _ _ _ _ _ _ _ _
///     _ _ _ _ _ _ _ _
///     _ _ _ _ _ _ _ _
///     _ _ _ _ _ _ _ _
///     P P P P P P P P
///     R N B Q K B N R
/// );
///
/// board!(
///     _ _ _ _ _ _ K _
///     _ _ _ _ _ _ q _
///     _ _ _ _ _ k _ _
///     _ _ _ _ _ _ _ _
///     _ _ _ _ _ _ _ _
///     _ _ _ _ _ _ _ _
///     _ _ _ _ _ _ _ _
///     _ _ _ _ _ _ _ _
/// );
/// ```
#[macro_export]
macro_rules! board {
    (
        $a8:tt $b8:tt $c8:tt $d8:tt $e8:tt $f8:tt $g8:tt $h8:tt
        $a7:tt $b7:tt $c7:tt $d7:tt $e7:tt $f7:tt $g7:tt $h7:tt
        $a6:tt $b6:tt $c6:tt $d6:tt $e6:tt $f6:tt $g6:tt $h6:tt
        $a5:tt $b5:tt $c5:tt $d5:tt $e5:tt $f5:tt $g5:tt $h5:tt
        $a4:tt $b4:tt $c4:tt $d4:tt $e4:tt $f4:tt $g4:tt $h4:tt
        $a3:tt $b3:tt $c3:tt $d3:tt $e3:tt $f3:tt $g3:tt $h3:tt
        $a2:tt $b2:tt $c2:tt $d2:tt $e2:tt $f2:tt $g2:tt $h2:tt
        $a1:tt $b1:tt $c1:tt $d1:tt $e1:tt $f1:tt $g1:tt $h1:tt
    ) => ( {
        let mut board = Board::EMPTY;
        let mut iter  = Square::iter();

        board_pieces!(board, iter,
            $a1 $b1 $c1 $d1 $e1 $f1 $g1 $h1
            $a2 $b2 $c2 $d2 $e2 $f2 $g2 $h2
            $a3 $b3 $c3 $d3 $e3 $f3 $g3 $h3
            $a4 $b4 $c4 $d4 $e4 $f4 $g4 $h4
            $a5 $b5 $c5 $d5 $e5 $f5 $g5 $h5
            $a6 $b6 $c6 $d6 $e6 $f6 $g6 $h6
            $a7 $b7 $c7 $d7 $e7 $f7 $g7 $h7
            $a8 $b8 $c8 $d8 $e8 $f8 $g8 $h8
        );

        board
    } )
}

#[allow(clippy::module_name_repetitions)]
#[doc(hidden)]
#[macro_export]
macro_rules! board_pieces {
    ( $board:ident, $iter:expr, _ $($pieces:tt)* ) => (
        $board[$iter.next().unwrap()] = None;
        board_pieces!($board, $iter, $($pieces)*);
    );

    ( $board:ident, $iter:expr, $piece:tt $($pieces:tt)* ) => (
        $board[$iter.next().unwrap()] = Piece::from_fen(stringify!($piece).as_bytes()[0]);
        board_pieces!($board, $iter, $($pieces)*);
    );

    ( $board:ident, $index:expr, ) => ();
}
