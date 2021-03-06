// not all code paths will be exercised by the build script
#![allow(dead_code)]

#![feature(const_convert)]
#![feature(const_intoiterator_identity)]
#![feature(const_mut_refs)]
#![feature(const_ops)]
#![feature(const_option)]
#![feature(const_slice_index)]
#![feature(const_trait_impl)]
#![feature(const_try)]
#![feature(macro_metavar_expr)]
#![feature(mixed_integer_ops)]
#![feature(new_uninit)]
#![feature(strict_provenance)]

use std::env;
use std::path::{Path, PathBuf};

#[path = "src/macros.rs"]
#[macro_use]
mod macros;

#[path = "src/bitboard/mod.rs"]
mod bitboard;

#[path = "src/misc/mod.rs"]
mod misc;

#[path = "src/types/mod.rs"]
mod types;

fn main() {
    println!("cargo:rustc-cfg=use_computed_bitboards");

    detect_hardware_features();
    generate_bitboards();
}

fn detect_hardware_features() {
    // TODO: actually detect hardware features :(
    // println!("cargo:rustc-cfg=use_pext");
    println!("cargo:rustc-cfg=use_popcnt");
}

fn generate_bitboards() {
    let dir = PathBuf::from(env::var("OUT_DIR").unwrap())
    .join("bitboards");

    generate("BB_POPCNT_16",       &dir, &computed::popcnt16());
    generate("BB_SQUARE_DISTANCE", &dir, &computed::square_distance());
    generate("BB_SQUARE",          &dir, &computed::square());
    generate("BB_LINE" ,           &dir, &computed::line());
    generate("BB_BETWEEN",         &dir, &computed::between());
    generate("BB_PSEUDO_ATTACKS",  &dir, &computed::pseudo_attacks());
    generate("BB_PAWN_ATTACKS",    &dir, &computed::pawn_attacks());

    let bishop_magics = computed::bishop_magics();
    generate("BB_BISHOP_MAGIC_MAGICS",  &dir, &bishop_magics.magics);
    generate("BB_BISHOP_MAGIC_ATTACKS", &dir, &bishop_magics.attacks);

    let rook_magics = computed::rook_magics();
    generate("BB_ROOK_MAGIC_MAGICS",  &dir, &rook_magics.magics);
    generate("BB_ROOK_MAGIC_ATTACKS", &dir, &rook_magics.attacks);
}

fn generate<T: bytemuck::Pod>(name: &str, dir: &Path, data: &T) {
    use std::fs::{self, File};
    use std::io::prelude::Write;

    fs::create_dir_all(dir).unwrap();

    let mut path: PathBuf = dir.to_path_buf();
    path.push(name.to_lowercase());
    path.set_extension("bin");

    let bytes = bytemuck::bytes_of(data);

    File::options()
        .create(true)
        .write(true)
        .open(&path).unwrap()
        .write_all(bytes).unwrap();

    println!("cargo:rustc-env=STOCKFISH_RS_{}={}", name, &path.display());
}

mod computed {
    use crate::bitboard::{self, Bitboard, Magic};
    use crate::types::{Color, PieceType, Square};

    pub fn popcnt16() -> [u8; 1 << 16] {
        let mut popcnt16 = [0; 1 << 16];

        for i in 0..u16::MAX {
            popcnt16[i as usize] = bitboard::popcnt64(i as _);
        }

        popcnt16
    }

    pub fn square_distance() -> [[u8; Square::COUNT]; Square::COUNT] {
        let mut square_distance = [[0; 64]; 64];

        for s1 in Square::iter() {
            for s2 in Square::iter() {
                square_distance[s1][s2] = bitboard::square_distance(s1, s2);
            }
        }

        square_distance
    }

    pub fn square() -> [Bitboard; Square::COUNT] {
        let mut square = [Bitboard::EMPTY; 64];

        for s in Square::iter() {
            square[s] = bitboard::square(s);
        }

        square
    }

    pub fn line() -> [[Bitboard; Square::COUNT]; Square::COUNT] {
        let mut line = [[Bitboard::EMPTY; 64]; 64];

        for s1 in Square::iter() {
            for s2 in Square::iter() {
                line[s1][s2] = bitboard::line(s1, s2);
            }
        }

        line
    }

    pub fn between() -> [[Bitboard; Square::COUNT]; Square::COUNT] {
        let mut between = [[Bitboard::EMPTY; 64]; 64];

        for s1 in Square::iter() {
            for s2 in Square::iter() {
                between[s1][s2] = bitboard::between(s1, s2);
            }
        }

        between
    }

    pub fn bishop_magics() -> Box<bitboard::Magic<0x1480>> {
        Magic::new_bishop()
    }

    pub fn rook_magics() -> Box<bitboard::Magic<0x19000>> {
        Magic::new_rook()
    }

    pub fn pawn_attacks() -> [[Bitboard; Square::COUNT]; Color::COUNT] {
        let mut pawn_attacks = [[Bitboard::EMPTY; 64]; 2];

        for c in Color::iter() {
            for s in Square::iter() {
                pawn_attacks[c][s] = bitboard::pawn_attacks(c, s);
            }
        }

        pawn_attacks
    }

    pub fn pseudo_attacks() -> [[Bitboard; Square::COUNT]; PieceType::COUNT] {
        let mut pseudo_attacks = [[Bitboard::EMPTY; 64]; 6];

        for pt in [
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Rook,
            PieceType::Queen,
            PieceType::King,
        ] {
            for s in Square::iter() {
                pseudo_attacks[pt][s] = bitboard::pseudo_attacks(pt, s);
            }
        }

        pseudo_attacks
    }
}
