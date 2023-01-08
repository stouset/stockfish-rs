use crate::prelude::*;

#[derive(Copy, Debug, Eq, PartialEq)]
#[derive_const(Clone)]
#[must_use]
pub struct Board([Option<Token>; Square::COUNT]);

impl Board {
    pub const EMPTY: Self = Self([None; Square::COUNT]);
}

// TODO: this is an annoying detail to expose and breaks the abstraction, but it
// allows for a convenient implementation of parsing a chess board from FEN
impl const std::ops::Index<usize> for Board {
    type Output = Option<Token>;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

// TODO: this is an annoying detail to expose and breaks the abstraction, but it
// allows for a convenient implementation of parsing a chess board from FEN
impl const std::ops::IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl const std::ops::Index<Square> for Board {
    type Output = Option<Token>;

    fn index(&self, index: Square) -> &Self::Output {
        self.index(index.as_usize())
    }
}

impl const std::ops::IndexMut<Square> for Board {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        self.index_mut(index.as_usize())
    }
}

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
        let mut iter  = Square::A1..=Square::H8;

        board_tokens!(board, iter,
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
macro_rules! board_tokens {
    ( $board:ident, $iter:expr, _ $($tokens:tt)* ) => (
        $board[$iter.next().unwrap()] = None;
        board_tokens!($board, $iter, $($tokens)*);
    );

    ( $board:ident, $iter:expr, $token:tt $($tokens:tt)* ) => (
        $board[$iter.next().unwrap()] = Token::from_fen(stringify!($token).as_bytes()[0]);
        board_tokens!($board, $iter, $($tokens)*);
    );

    ( $board:ident, $index:expr, ) => ();
}
