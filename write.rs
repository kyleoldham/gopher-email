use std::fs::File;
use std::io::prelude::*;
use std::io::Write;

fn main() {
    let mut buffer = File::create("wild.txt");
    buffer.write("yo this isn't a cli tool but i wish it were");
    file.write_all(b"Hello, world!");
}
