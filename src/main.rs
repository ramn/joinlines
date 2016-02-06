//! Joins two blocks of text, such that row 1 of the first block will be
//! appended to row 1 of the second block and so on.
//!
//! Input is read from stdin until EOF, output to stdout. The the first half of
//! the input lines will become the first block.

use std::io::prelude::*;
use std::io::stdin;
use std::error::Error;
use std::env;

fn main() {
    run().unwrap();
}

fn run() -> Result<(), Box<Error>> {
    let separator: String = env::args().nth(1).unwrap_or("".to_string());
    let mut buffer = String::new();
    try!(stdin().read_to_string(&mut buffer));
    let lines: Vec<&str> = buffer.lines().collect();
    let lines_per_block = lines.len() / 2;
    let first = &lines[0..lines_per_block];
    let second = &lines[lines_per_block..];
    for (&left, &right) in first.iter().zip(second.iter()) {
        println!("{}{}{}", left, separator, right);
    }
    Ok(())
}
