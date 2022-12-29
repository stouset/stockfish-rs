use stockfish_core::prelude::*;
use stockfish_core::accelerate::computed;
use stockfish_core::bitboard::magic::Magic;

use std::io::prelude::Write;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

fn main() -> std::io::Result<()> {
    let output_root = PathBuf::from("./stockfish-core/share/cached");

    fs::create_dir_all(&output_root)?;

    accelerate("square_distance", &output_root, &generate_square_distance());
    accelerate("pseudo_attacks",  &output_root, &generate_pseudo_attacks());
    accelerate("pawn_attacks",    &output_root, &generate_pawn_attacks());

    let bishop_magics = Magic::new_bishop();
    accelerate("bishop_magic_numbers", &output_root, &bishop_magics.magics);
    accelerate("bishop_magic_attacks", &output_root, &bishop_magics.attacks);

    let rook_magics = Magic::new_rook();
    accelerate("rook_magic_numbers", &output_root, &rook_magics.magics);
    accelerate("rook_magic_attacks", &output_root, &rook_magics.attacks);

    Ok(())
}

fn accelerate<P: AsRef<Path>, T: bytemuck::Pod>(name: &str, root: P, data: &T) {
    let path  = root.as_ref().join(name.to_lowercase() + ".bin");
    let bytes = bytemuck::bytes_of(data);

    File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path).unwrap()
        .write_all(bytes).unwrap();
}

fn generate_square_distance() -> [[u8; Square::COUNT]; Square::COUNT] {
    let mut distances = [[0; Square::COUNT]; Square::COUNT];

    for s1 in Square::into_iter() {
        for s2 in Square::into_iter() {
            distances[s1][s2] = computed::square_distance(s1, s2);
        }
    }

    distances
}

fn generate_pseudo_attacks() -> [[Bitboard; Square::COUNT]; Piece::COUNT] {
    let mut pseudo_attacks = [[Bitboard::EMPTY; Square::COUNT]; Piece::COUNT];

    for piece in Piece::into_iter().skip(1) {
        for square in Square::into_iter() {
            pseudo_attacks[piece][square] = computed::pseudo_attacks(piece, square);
        }
    }

    pseudo_attacks
}

fn generate_pawn_attacks() -> [[Bitboard; Square::COUNT]; Color::COUNT] {
    let mut pawn_attacks = [[Bitboard::EMPTY; Square::COUNT]; Color::COUNT];

    for color in Color::into_iter() {
        for square in Square::into_iter() {
            pawn_attacks[color][square] = computed::pawn_attacks(color, square);
        }
    }

    pawn_attacks
}
