#![allow(dead_code)]

use ropey::Rope;
use std::io::Read;

#[derive(Debug)]
struct RopeWc {
    bytes: usize,
    chars: usize,
    lines: usize,
}

impl RopeWc {
    fn count(str: &str) -> Self {
        let rope = Rope::from_str(str);

        Self {
            bytes: rope.len_bytes(),
            chars: rope.len_chars(),
            lines: rope.len_lines(),
        }
    }
}

fn main() {
    let input = {
        let mut buffer = String::new();
        let mut stdin = std::io::stdin();
        stdin.read_to_string(&mut buffer).unwrap();
        buffer
    };

    println!("{:#?}", RopeWc::count(&input));
}
