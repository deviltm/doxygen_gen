#![allow(dead_code)]
use std::{fs::{OpenOptions, File}, path::PathBuf};
use docx_rs::*;
use crate::parser::DocumentationData;

pub fn export_pdf(data: DocumentationData, file: PathBuf) -> Result<(), std::io::Error> {
    //Open file first, just so that we don't have to do the pdf generation if the path is incorrect
    let mut file = OpenOptions::new().write(true).open(file)?;
    Ok(())
}

pub fn export_doc(data: DocumentationData, file: PathBuf) -> Result<(), std::io::Error> {
    //Open file first, just so that we don't have to do the pdf generation if the path is incorrect
    let file = File::create(file)?;
    let mut doc = Docx::new();
    for item in data{
        doc = doc.add_paragraph(Paragraph::new().add_run(Run::new().add_text(item.name)));
    }
    doc.build().pack(file)?;
    Ok(())
}
