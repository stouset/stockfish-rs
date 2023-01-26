#![allow(clippy::missing_inline_in_public_items)]

use super::Bitboard;
use crate::prelude::*;

use core::iter::FusedIterator;

/// An [`Iterator`] that enumerates over every [`Square`] contained in a
/// [`Bitboard`].
#[derive(Debug, Eq)]
#[derive_const(Clone, PartialEq)]
#[must_use]
pub struct Iter {
    bb: Bitboard,
}

impl Iter {
    pub(crate) const fn new(bitboard: Bitboard) -> Self {
        Self {
            bb: bitboard,
        }
    }
}

impl Iterator for Iter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let lsb = self.bb.0.trailing_zeros() as usize;
        let s   = Square::VARIANTS.get(lsb).copied();

        if s.is_some() {
            self.bb &= Bitboard(self.bb.0 - 1);
        }

        s
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.bb.count(), Some(self.bb.count()))
    }
}

impl FusedIterator for Iter {}
