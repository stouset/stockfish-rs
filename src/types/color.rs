c_style_enum! {
    /// A file, A through H, on a chess board. The variants for this enum are
    /// prefixed an underscore to mimic those of [`Rank`].
    pub Color, u8, 2; [
        White, Black
    ]
}
