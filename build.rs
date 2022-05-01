use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let bitboard_dir = PathBuf::from(env::var("OUT_DIR").unwrap())
        .join("bitboards");

    generate_bitboards(&bitboard_dir);
}

fn generate_bitboards(dir: &Path) {
    generate("BB_POPCNT_16",       dir, &bitboard::popcnt16()[..]);
    generate("BB_SQUARE_DISTANCE", dir, &bitboard::square_distance()[..]);
    generate("BB_SQUARE",          dir, &bitboard::square());
}

fn generate<T: bytemuck::Pod>(name: &str, dir: &Path, data: &[T]) {
    use std::fs::{self, File};
    use std::io::prelude::Write;

    let _ = fs::create_dir_all(dir).unwrap();

    let mut path : PathBuf = dir.to_path_buf();
    path.push(name.to_lowercase());
    path.set_extension("bin");

    let bytes = bytemuck::cast_slice(data);

    File::options()
        .create(true)
        .write(true)
        .open(&path).unwrap()
        .write_all(bytes).unwrap();

    println!("cargo:rustc-env=STOCKFISH_RS_{}={}", name, &path.display());
}

mod bitboard {
    pub fn popcnt16() -> [u8; 1 << 16] {
        let mut popcnt16 = [0; 1 << 16];

        for (i, v) in popcnt16.iter_mut().enumerate() {
            *v = i.count_ones().try_into().unwrap();
        }

        popcnt16
    }

    pub fn square_distance() -> [[u8; 64]; 64] {
        let mut square_distance = [[0; 64]; 64];

        for i in 0..64 {
            for j in 0..64 {
                let i_file: u8 = i &  7;
                let i_rank: u8 = i >> 3;
                let j_file: u8 = j &  7;
                let j_rank: u8 = j >> 3;

                square_distance[i as usize][j as usize] = std::cmp::max(
                    i_file.abs_diff(j_file),
                    i_rank.abs_diff(j_rank),
                );
            }
        }

        square_distance
    }

    pub fn square() -> [u64; 64] {
        let mut square = [0; 64];

        for (i, v) in square.iter_mut().enumerate() {
            *v = 1 << i;
        }

        square
    }
}
