/// The type of rules a game will occur under.
#[derive(Copy, Debug, Eq)]
#[derive_const(Clone, PartialEq)]
#[repr(u8)]
pub enum Ruleset {
    /// A standard game of chess.
    Standard,

    /// A game of Fischer random chess, where the starting state is randomized.
    Chess960,
}
