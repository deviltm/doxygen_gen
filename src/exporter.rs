#![allow(dead_code)]
use crate::parser::DocumentationData;
use docx_rs::*;
use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

#[macro_export]
macro_rules! paragraph {
    ($text:expr) => {
        paragraph!($text, AlignmentType::Center)
    };
    ($text:expr,$align:expr) => {
        Paragraph::new()
            .add_run(Run::new().add_text($text))
            .align($align)
    };
}
#[macro_export]
macro_rules! cell {
    ($text:expr) => {
        TableCell::new().add_paragraph(paragraph!($text))
    };
    ($text:expr,$align:expr) => {
        TableCell::new().add_paragraph(paragraph!($text, $align))
    };
}

pub fn export_pdf(_data: DocumentationData, file: PathBuf) -> Result<(), std::io::Error> {
    //Open file first, just so that we don't have to do the pdf generation if the path is incorrect
    let mut _file = OpenOptions::new().write(true).open(file)?;
    Ok(())
}

pub fn export_doc(data: DocumentationData, file: PathBuf) -> Result<(), std::io::Error> {
    //Open file first, just so that we don't have to do the pdf generation if the path is incorrect
    let file = File::create(file)?;
    //29700;21000 = 52.39;37.04
    //page_orient isn't working rn, so I had to do it manually
    let mut doc = Docx::new().page_size(16837, 11905);
    let mut table_count = 1;
    let header = vec![TableRow::new(vec![
        cell!("Название элемента структуры"),
        cell!("Код параметра"),
        cell!("Наименование параметра (сигнала)"),
        cell!("ЦСР (ЦМР)"),
        cell!("Знак"),
        cell!("Размещение в разряде"),
        cell!("Примечание"),
    ])];

    for item in data.items {
        //Table name
        doc = doc
            .add_paragraph(paragraph!(
                format!("Таблица {} - {} ({})", table_count, item.note, item.name),
                AlignmentType::Right
            ))
            .page_orient(PageOrientationType::Landscape);
        table_count += 1;

        //The actual table
        doc = doc.add_table(Table::new(
            header
                .iter()
                .cloned()
                .chain(item.children.iter().map(|l| {
                    TableRow::new(vec![
                        cell!(l.datatype.to_owned()),
                        cell!("-"),
                        cell!(l.note.to_owned()),
                        cell!("-"),
                        cell!("-"),
                        cell!("-"),
                        cell!("-"),
                    ])
                }))
                .collect(),
        ));
    }
    doc.build().pack(file)?;
    Ok(())
}
