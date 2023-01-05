use bitflags::bitflags;

bitflags! {
    #[must_use]
    pub struct Castling: u8 {
        const NONE = 0;

        const WHITE_OO  = 1 << 0;
        const WHITE_OOO = 1 << 1;
        const BLACK_OO  = 1 << 2;
        const BLACK_OOO = 1 << 3;

        const WHITE      = Self::WHITE_OO .bits | Self::WHITE_OOO.bits;
        const BLACK      = Self::BLACK_OO .bits | Self::BLACK_OOO.bits;
        const KING_SIDE  = Self::WHITE_OO .bits | Self::BLACK_OO .bits;
        const QUEEN_SIDE = Self::WHITE_OOO.bits | Self::BLACK_OOO.bits;
        const ANY        = Self::WHITE    .bits | Self::BLACK    .bits;
    }
}

impl Castling {
    pub const COUNT: usize = Self::ANY.bits as usize + 1;
}
