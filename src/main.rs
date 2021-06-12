mod karatsuba;
mod power;
mod sort;
mod strassen;

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error, ErrorKind, Read},
};

fn read_to_ints(filename: &str) -> Result<Vec<i64>, Error> {
    let file = File::open(filename).expect("File not found!");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() {
    println!("Hello world!");
    let filename = "./data/integers.txt";
    let mut vec = read_to_ints(filename).ok().unwrap();
    let inversions = sort::count_inversions(&mut vec);
    println!("INVERSIONS: {}", inversions);
}
