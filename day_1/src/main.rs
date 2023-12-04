use std::fs;

use day_1::{part1, part2};

fn main() {
  let input = fs::read_to_string("input.txt")
    .expect("Unable to read input file");

  part1::main(input.clone());
  part2::main(input.clone());
}