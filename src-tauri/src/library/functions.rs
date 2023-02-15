/*      Common functions I use

        2023.02.13       Sven Ponelat

*/

use std::fs::*;
use std::io::{self, prelude::*, BufReader};





// Returns an iterator to the lines
pub fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}
