#[derive(Copy, Debug, Eq)]
#[derive_const(Clone, PartialEq)]
#[repr(u8)]
pub enum Ruleset {
    Standard,
    Chess960,
}
