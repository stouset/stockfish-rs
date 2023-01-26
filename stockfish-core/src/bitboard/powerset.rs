#![allow(clippy::missing_inline_in_public_items)]

use super::Bitboard;

use core::iter::FusedIterator;

/// An [`Iterator`] that enumerates over every subset of enabled squares
/// contained in a [`Bitboard`].
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use]
pub struct Powerset {
    source: Bitboard,
    next:   Option<Bitboard>,
}

impl Powerset {
    pub(crate) const fn new(bitboard: Bitboard) -> Self {
        Self {
            source: bitboard,
            next:   Some(Bitboard::EMPTY),
        }
    }
}

impl Iterator for Powerset {
    type Item = Bitboard;

    fn next(&mut self) -> Option<Self::Item> {
        // use Carry-Ripler trick to enumerate all subsets of the source
        // bitboard
        let next  = self.next;
        self.next = self.next
            .map(|bb| bb.0.wrapping_sub(self.source.0) & self.source.0)
            .map(Bitboard::from)
            .filter(|bb| bb.is_any());

        next
    }

    // TODO: more accurately estimate the bounds
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(2_usize.pow(self.source.0.count_ones())))
    }
}

impl FusedIterator for Powerset {}
