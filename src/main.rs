#![allow(dead_code)]

use std::path::PathBuf;

use parser::parse_file;
//Header parser module
mod parser;
//module for the iced window
mod main_window;

fn main() {
    let data = parse_file(
        PathBuf::from("/home/luna/Projects/doxygen_gen/testing/mfci_io_70.h"),
        encoding::all::WINDOWS_1251,
    )
    .unwrap();
    println!("{:?}", data)
}
