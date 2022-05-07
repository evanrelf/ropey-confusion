#![allow(dead_code)]

use ropey::Rope;
use std::{fs::File, io, path::Path};

#[derive(Debug)]
struct RopeWc {
    bytes: usize,
    chars: usize,
    lines: usize,
}

impl RopeWc {
    fn count(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let rope = Rope::from_reader(File::open(path)?)?;

        Ok(Self {
            bytes: rope.len_bytes(),
            chars: rope.len_chars(),
            lines: rope.len_lines(),
        })
    }
}

fn main() {
    for path in std::env::args().skip(1) {
        println!("{:#?}", RopeWc::count(path).unwrap());
    }
}
