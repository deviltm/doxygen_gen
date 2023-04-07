//The GPLv3 License (GPLv3)
//
//Copyright (c) 2023 Ciubix8513
//
//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU General Public License as published by
//the Free Software Foundation, either version 3 of the License, or
//any later version.
//
//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU General Public License for more details.
//
//You should have received a copy of the GNU General Public License
//along with this program.  If not, see <http://www.gnu.org/licenses/>.

use crate::parser::DocumentationData;
use docx_rs::*;
use std::{fs::File, path::PathBuf};

//Some macros to make my life easier
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

pub fn export_doc(data: DocumentationData, file: PathBuf) -> Result<(), std::io::Error> {
    //Open file first, just so that we don't have to do the pdf generation if the path is incorrect
    let file = File::create(file)?;
    //29700;21000 = 52.39;37.04
    //page_orient isn't working rn, so I had to do it manually
    let mut doc = Docx::new().page_size(16837, 11905);
    let mut table_count = 1;
    //While I can make a system for defining the tables, it'd be quite a pain for this project
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
                        cell!(l.signed.to_owned()),
                        cell!(l.bits.to_owned()),
                        cell!(if l.val.is_empty() {
                            l.additional_data.to_owned()
                        } else {
                            l.additional_data.to_owned() + " " + &l.val
                        }),
                    ])
                }))
                .collect(),
        ));
    }
    doc.build().pack(file)?;
    Ok(())
}
