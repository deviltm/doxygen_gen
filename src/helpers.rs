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

use std::path::{PathBuf, Path};

use encoding::Encoding;

use crate::{parser::parse_file, exporter::export_doc};



pub fn process_file(r#in: PathBuf, out: &Path, encoding: &dyn Encoding) {
    //I don't think I need all this error checking, but i'm just gonna leave it 
    let data = parse_file(r#in.clone(), encoding.to_owned());
    if data.is_err() {
        println!("Could not parse {}", r#in.display());
    }
    let data = data.unwrap();
    let mut out = out.join(r#in.file_name().unwrap());
    out.set_extension("docx");
    if let Err(e) = export_doc(data, out.clone()) {
        println!("{:#?}", out);
        println!("{:#?}", e);
    }
}
