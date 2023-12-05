mod d11;
mod d12;
use std::fs;

fn main() {
    let input =
        fs::read_to_string("./input/d1.txt").expect("Should have been able to read the file");

    d12::exec(input);
}
