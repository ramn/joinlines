use std::io::prelude::*;
use std::io::stdin;
use std::error::Error;
use std::env;


const ARG_MISSING: &'static str = "argument <lines_per_block> is missing";


/// Joins two blocks of text, such that row 1 of the first block will be
/// appended to row 1 of the second block and so on.
///
/// Takes an argument <lines_per_block> which is the number of lines in a block.
/// Both blocks must be of equal size.
///
/// Input is read from stdin, output to stdout.
fn main() {
    run().unwrap();
}

fn run() -> Result<(), Box<Error>> {
    let lines_per_block: usize =
        try!(try!(env::args().nth(1).ok_or(ARG_MISSING)).parse());
    let separator: String = env::args().nth(2).unwrap_or("".to_string());

    let mut buffer = String::new();
    try!(stdin().read_to_string(&mut buffer));
    let lines: Vec<&str> = buffer.lines().collect();
    assert!(lines_per_block * 2 == lines.len());

    let first = &lines[0..lines_per_block];
    let second = &lines[lines_per_block..];
    for (&left, &right) in first.iter().zip(second.iter()) {
        println!("{}{}{}", left, separator, right);
    }
    Ok(())
}
