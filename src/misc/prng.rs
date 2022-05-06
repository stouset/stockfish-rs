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
pub(crate) struct Prng {
    seed: u64,
}

impl Prng {
    pub(crate) fn from_seed(seed: u64) -> Self {
        debug_assert_ne!(seed, 0);

        Self { seed }
    }

    pub(crate) fn next_u64(&mut self) -> u64 {
        self.seed ^= self.seed >> 12;
        self.seed ^= self.seed << 25;
        self.seed ^= self.seed >> 27;

        self.seed.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    /// Special generator used to fast init magic numbers. Output values
    /// only have 1/8th of their bits set on average.
    pub(crate) fn next_sparse_u64(&mut self) -> u64 {
        self.next_u64() & self.next_u64() & self.next_u64()
    }
}
