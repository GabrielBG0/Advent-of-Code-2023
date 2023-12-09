mod d11;
mod d12;
mod d21;
mod d22;
use std::fs;

fn main() {
    let input =
        fs::read_to_string("./input/d2.txt").expect("Should have been able to read the file");

    d22::exec(input);
}
