mod d11;
mod d12;
mod d21;
use std::fs;

fn main() {
    let input =
        fs::read_to_string("./input/test.txt").expect("Should have been able to read the file");

    d21::exec(input);
}
