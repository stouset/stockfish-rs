use crate::prelude::*;
use crate::hash::Key;
use crate::misc::Prng;

/// A table of Zobrist hashes.
///
/// Zobrist hashing can be used to turn various components of game state into
/// unique keys. These keys *should* be random enough and independent enough
/// that you can uniquely represent any state through combining together any
/// reasonable combination of keys through a bitwise XOR.
///
/// As the board state changes, it's fast and trivial to update an existing key
/// by doing a bitwise XOR against components that are no longer relevant new
/// new components which are.
#[allow(missing_debug_implementations)]
#[must_use]
pub struct Zobrist {
    psq:        [[Key; Square::COUNT]; Piece::COUNT],
    en_passant: [Key; File::COUNT],
    castling:   [Key; CastlingRights::COUNT],
    side:       Key,
    no_pawns:   Key,
}

impl Zobrist {
    pub(crate) const fn new(seed: u64) -> Self {
        let mut prng = Prng::from(seed);

        let mut psq        = [[Key::default(); Square::COUNT]; Piece::COUNT];
        let mut en_passant = [Key::default(); File::COUNT];
        let mut castling   = [Key::default(); CastlingRights::COUNT];

        let mut i;
        let mut j;

        i = 0;

        while i < Piece::COUNT {
            j = 0;

            while j < Square::COUNT {
                psq[i][j] = prng.next_u64().into();

                j += 1;
            }

            i += 1;
        }

        i = 0;

        while i < File::COUNT {
            en_passant[i] = prng.next_u64().into();

            i += 1;
        }

        i = 0;

        while i < CastlingRights::COUNT {
            castling[i] = prng.next_u64().into();

            i += 1;
        }

        Self {
            psq,
            en_passant,
            castling,
            side:     prng.next_u64().into(),
            no_pawns: prng.next_u64().into(),
        }
    }

    /// Returns the Zobrist hash of a piece on a given square.
    #[inline]
    pub const fn piece_square_key(&self, piece: Piece, square: Square) -> Key {
        self.psq[piece][square]
    }

    /// Returns the zobrist hash for a given en passant file.
    #[inline]
    pub const fn en_passant_key(&self, file: File) -> Key {
        self.en_passant[file]
    }

    /// Returns the zobrist hash for a particular set of castling rights.
    ///
    /// Note that the key for each set of rights is completely independent of
    /// all the others, and computing the XOR of the keys of two distinct sets
    /// of rights does not result in the same key as you would get from
    /// performing XOR against the rights themselves.
    ///
    /// # Example:
    ///
    /// ```rust
    /// # use stockfish_core::prelude::*;
    /// # use stockfish_core::hash::ZOBRIST;
    ///
    /// let white_oo  = CastlingRights::WHITE_OO;
    /// let white_ooo = CastlingRights::WHITE_OOO;
    /// let white     = CastlingRights::WHITE;
    ///
    /// assert_eq!(
    ///     white,
    ///     white_oo ^ white_ooo,
    /// );
    ///
    /// assert_ne!(
    ///     ZOBRIST.castling_key(white),
    ///     ZOBRIST.castling_key(white_oo) ^ ZOBRIST.castling_key(white_ooo),
    /// )
    /// ```
    #[inline]
    pub const fn castling_key(&self, castling: CastlingRights) -> Key {
        self.castling[castling.bits() as usize]
    }

    /// Returns a key which represents a change in the side to act.
    #[inline]
    pub const fn side_key(&self) -> Key {
        self.side
    }

    /// TODO: document me!
    #[inline]
    pub const fn no_pawns_key(&self) -> Key {
        self.no_pawns
    }
}

impl const Default for Zobrist {
    #[inline]
    fn default() -> Self {
        // The seed constant was lifted from Stockfish. Presumably it was closen as
        // having been verified to minimize (or eliminate) cuckoo hashing
        // collisions.
        Self::new(1_070_372)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;

    #[test]
    fn test_all_keys_unique() {
        let mut set     = HashSet::new();
        let     zobrist = Zobrist::default();

        for piece in Piece::iter() {
            for square in Square::iter() {
                assert!(set.insert(zobrist.piece_square_key(piece, square)));
            }
        }

        for file in File::iter() {
            assert!(set.insert(zobrist.en_passant_key(file)));
        }

        for castling in CastlingRights::iter() {
            assert!(set.insert(zobrist.castling_key(castling)));
        }

        assert!(set.insert(zobrist.side_key()));
        assert!(set.insert(zobrist.no_pawns_key()));
    }
}
