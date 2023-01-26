//! Components for hash tables and the algorithms that generate the keys used to
//! index into them.

mod zobrist;

pub use zobrist::Zobrist;

use core::ops::{BitXor, BitXorAssign};

/// A precomputed table of Zobrist hashes.
///
/// This table can be used to turn various components of game state into unique
/// keys. These keys *should* be random enough and independent enough that you
/// can uniquely represent any state through combining together any reasonable
/// combination of keys through a bitwise XOR.
///
/// As the board state changes, it's fast and trivial to update an existing key
/// by doing a bitwise XOR against components that are no longer relevant new
/// new components which are.
pub const ZOBRIST: Zobrist = Zobrist::default();

/// A computed lookup key for indexing into hash tables.
#[derive(Copy, Debug, Eq, Hash)]
#[derive_const(Clone, Default, PartialEq)]
#[must_use]
pub struct Key(u64);

impl const From<u64> for Key {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl const BitXor for Key {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0.bitxor(rhs.0))
    }
}

impl const BitXorAssign for Key {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0.bitxor_assign(rhs.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_clone() {
        assert_eq!(Key::default(), Key::default().clone());
    }

    #[test]
    fn key_debug() {
        assert_ne!("", format!("{:?}", Key::from(0)));
    }

    #[test]
    fn key_bitxor() {
        let k1 = Key::from(0b1010_1111_u64);
        let k2 = Key::from(0b1111_1010_u64);

        assert_eq!(Key::from(0b0101_0101_u64), k1 ^ k2);
    }

    #[test]
    fn key_bitxor_assign() {
        let mut k1 = Key::from(0b1010_1111_u64);
        let     k2 = Key::from(0b1111_1010_u64);

        k1 ^= k2;

        assert_eq!(Key::from(0b0101_0101_u64), k1);
    }
}
