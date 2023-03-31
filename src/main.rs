#![allow(dead_code)]
use exporter::{export_pdf, export_doc};
use parser::parse_file;
use std::path::PathBuf;

//Module to define all the needed regex
mod regex;
//Header parser module
mod parser;
//Data exporter module
mod exporter;
//module for the iced window
mod main_window;

fn main() {
    let data = parse_file(
        PathBuf::from("/home/luna/Projects/doxygen_gen/testing/mfci_io_70.h"),
        encoding::all::WINDOWS_1251,
    )
    .unwrap();
    export_doc(
        data,
        PathBuf::from("/home/luna/Projects/doxygen_gen/testing/out.docx"),
    )
    .unwrap();
}
