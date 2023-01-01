/// xorshift64star Pseudo-Random Number Generator
/// This class is based on original code written and dedicated
/// to the public domain by Sebastiano Vigna (2014).
/// It has the following characteristics:
///
///  -  Outputs 64-bit numbers
///  -  Passes `Dieharder` and `SmallCrush` test batteries
///  -  Does not require warm-up, no zeroland to escape
///  -  Internal state is a single 64-bit integer
///  -  Period is 2^64 - 1
///  -  Speed: 1.60 ns/call (Core i7 @3.40GHz)
///
/// For further analysis see
///   <http://vigna.di.unimi.it/ftp/papers/xorshift.pdf>
#[must_use]
pub(crate) struct Prng {
    /// The internal seed from which the next pseudorandom number is generated.
    seed: u64,
}

impl Prng {
    /// Returns a new pseudorandom [`u64`]. Updates the seed internally so the
    /// next value will be different (with overwhelming probability).
    #[must_use]
    pub(crate) const fn next_u64(&mut self) -> u64 {
        self.seed ^= self.seed >> 12;
        self.seed ^= self.seed << 25;
        self.seed ^= self.seed >> 27;

        self.seed.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    /// Special generator used to fast init magic numbers. Output values
    /// only have 1/8th of their bits set on average.
    #[must_use]
    pub(crate) const fn next_sparse_u64(&mut self) -> u64 {
        self.next_u64() & self.next_u64() & self.next_u64()
    }
}

impl const From<u64> for Prng {
    fn from(seed: u64) -> Self {
        assert!(seed != 0, "seed must not be zero");

        Self { seed }
    }
}

impl Iterator for Prng {
    type Item = u64;

    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_u64())
    }

    #[must_use]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prng_next_u64() {
        let mut prng = Prng::from(1);

        assert_eq!(0x47E4_CE4B_896C_DD1D, prng.next_u64());
        assert_eq!(0xABCF_A6A8_E079_651D, prng.next_u64());
        assert_eq!(0xB9D1_0D8F_EB73_1F57, prng.next_u64());
    }

    #[test]
    fn prng_next_sparse_u64() {
        let mut prng = Prng::from(1);

        assert_eq!(
            0x47E4_CE4B_896C_DD1D & 0xABCF_A6A8_E079_651D & 0xB9D1_0D8F_EB73_1F57,
            prng.next_sparse_u64()
        );
    }

    #[test]
    #[should_panic(expected = "must not be zero")]
    fn prng_with_zero_seed() {
        let _ = Prng::from(0);
    }
}
