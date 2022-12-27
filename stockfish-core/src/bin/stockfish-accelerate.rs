use stockfish_core::prelude::*;
use stockfish_core::accelerate::computed;

use std::io::prelude::Write;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

fn main() -> std::io::Result<()> {
    let output_root = PathBuf::from("./share/cached");

    fs::create_dir_all(&output_root)?;

    accelerate("square_distance", &output_root, &generate_square_distance());

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
