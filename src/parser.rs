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

use crate::regex::*;
use encoding::{DecoderTrap, Encoding};
use std::{fs::OpenOptions, io::Read, path::PathBuf};

enum ParsingState {
    None,
    Name,
    Fields,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum DocumentationType {
    #[default]
    Struct,
    Enum,
}

#[derive(Default, Debug, Clone)]
pub struct DocumentationItemChild {
    pub datatype: String,
    pub code: String,
    pub note: String,
    pub additional_data: String,
    pub signed: String,
    pub bits: String,
    pub msb: String,
}

#[derive(Default, Debug, Clone)]
pub struct DocumentationItem {
    pub r#type: DocumentationType,
    pub note: String,
    pub name: String,
    pub children: Vec<DocumentationItemChild>,
}

#[derive(Default, Debug, Clone)]
pub struct DocumentationData {
    //Potentially add other data here
    pub items: Vec<DocumentationItem>,
}

pub fn parse_file(
    path: PathBuf,
    encoding: &dyn Encoding,
) -> Result<DocumentationData, std::io::Error> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let contents = &mut Vec::<u8>::new();
    file.read_to_end(contents)?;

    //Decode the file before processing it
    let contents = encoding.decode(contents, DecoderTrap::Ignore).unwrap();

    let mut data = DocumentationData::default();
    //Have to asign the default value, even tho it's not used
    let mut curret_item = DocumentationItem::default();
    let mut parsing_state = ParsingState::None;

    //Precompile the regex objects
    let def_regex = name_regex();
    let field_regex = field_regex();
    let field_code_regex = field_code_regex();
    let additional_data_regex = additional_data_regex();
    let signed_regex = signed_data_regex();

    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }
        match parsing_state {
            ParsingState::None => {
                //Encountered a struct/enum definition
                if line.contains("//! ") {
                    curret_item = DocumentationItem {
                        note: line[4..].to_owned(),
                        ..Default::default()
                    };
                    parsing_state = ParsingState::Name;
                }
            }
            ParsingState::Name => {
                let captures = def_regex.captures(line);
                if let Some(captures) = captures {
                    if captures.get(1).unwrap().as_str() == "enum" {
                        curret_item.r#type = DocumentationType::Enum;
                    }
                    curret_item.name = captures.get(2).unwrap().as_str().to_owned();
                    parsing_state = ParsingState::Fields;
                } else if line.contains("#define ") {
                    parsing_state = ParsingState::None;
                }
            }
            ParsingState::Fields => {
                let captures = field_regex.captures(line);
                if let Some(captures) = captures {
                    let mut note = captures.get(2).unwrap().as_str();
                    let mut code = "-";
                    let mut data = "-";
                    let mut signed = "-";
                    let mut bits = "-";
                    let mut msb = "-";
                    if note.contains(" //") {
                        if let Some(captures) = additional_data_regex.captures(note) {
                            note = captures.get(1).unwrap().as_str();
                            data = captures.get(2).unwrap().as_str();
                            if let Some(captures) = signed_regex.captures(data) {
                                data = captures.get(1).unwrap().as_str();
                                signed = captures.get(2).unwrap().as_str();
                                bits = captures.get(3).unwrap().as_str();
                                msb = captures.get(4).unwrap().as_str();
                            }
                        }
                    }
                    if note.contains("[") {
                        if let Some(captures) = field_code_regex.captures(note) {
                            code = captures.get(1).unwrap().as_str();
                            note = captures.get(2).unwrap().as_str();
                        }
                    }
                    curret_item.children.push(DocumentationItemChild {
                        datatype: captures.get(1).unwrap().as_str().to_owned(),
                        code: code.to_owned(),
                        note: note.to_owned(),
                        additional_data: data.to_owned(),
                        signed: signed.to_owned(),
                        bits: bits.to_owned(),
                        msb: msb.to_owned(),
                    });
                }
                //The name check may not be necesarry, but gonna leave it here just in case
                else if line.contains(curret_item.name.as_str()) && line.contains('}') {
                    data.items.push(curret_item.clone());
                    parsing_state = ParsingState::None;
                }
            }
        }
    }

    Ok(data)
}

//A whole bunch of tests, which I didn't really need, I was trying to find a bug I had, but it
//turned out that it's in exproter.rs
#[test]
fn parse_simple_file_test() {
    let data = parse_file(PathBuf::from("test_data/test1.h"), encoding::all::UTF_8).unwrap();
    assert_eq!(data.items.len(), 1);
    let data = data.items[0].clone();
    assert_eq!(data.children.len(), 2);
    let expected = DocumentationItem {
        r#type: DocumentationType::Struct,
        note: "Test struct".to_owned(),
        name: "test".to_owned(),
        children: Vec::default(),
    };
    assert_eq!(data.r#type, expected.r#type);
    assert_eq!(data.note, expected.note);
    assert_eq!(data.name, expected.name);
    let expected = DocumentationItemChild {
        datatype: "int a;".to_owned(),
        note: "This is A".to_owned(),
        additional_data: "".to_owned(),
        ..Default::default()
    };
    let child = data.children[0].clone();
    assert_eq!(child.datatype, expected.datatype);
    assert_eq!(child.note, expected.note);

    let expected = DocumentationItemChild {
        datatype: "int b;".to_owned(),
        note: "This is B".to_owned(),
        additional_data: "".to_owned(),
        ..Default::default()
    };
    let child = data.children[1].clone();
    assert_eq!(child.datatype, expected.datatype);
    assert_eq!(child.note, expected.note);
}

#[test]
fn parse_file_with_multiple_stucts_test() {
    let data = parse_file(PathBuf::from("test_data/test3.h"), encoding::all::UTF_8).unwrap();
    assert_eq!(data.items.len(), 2);
    let data = data.items[1].clone();
    assert_eq!(data.children.len(), 2);
    let expected = DocumentationItem {
        r#type: DocumentationType::Struct,
        note: "Test struct".to_owned(),
        name: "test".to_owned(),
        children: Vec::default(),
    };
    assert_eq!(data.r#type, expected.r#type);
    assert_eq!(data.note, expected.note);
    assert_eq!(data.name, expected.name);
    let expected = DocumentationItemChild {
        datatype: "int a;".to_owned(),
        note: "This is A".to_owned(),
        additional_data: "".to_owned(),
        ..Default::default()
    };
    let child = data.children[0].clone();
    assert_eq!(child.datatype, expected.datatype);
    assert_eq!(child.note, expected.note);

    let expected = DocumentationItemChild {
        datatype: "int b;".to_owned(),
        note: "This is B".to_owned(),
        additional_data: "".to_owned(),
        ..Default::default()
    };
    let child = data.children[1].clone();
    assert_eq!(child.datatype, expected.datatype);
    assert_eq!(child.note, expected.note);
}

#[test]
fn parse_file_with_enum_test() {
    let data = parse_file(PathBuf::from("test_data/test4.h"), encoding::all::UTF_8).unwrap();
    assert_eq!(data.items.len(), 2);
    let data = data.items[1].clone();
    assert_eq!(data.children.len(), 2);
    let expected = DocumentationItem {
        r#type: DocumentationType::Enum,
        note: "Test struct".to_owned(),
        name: "test".to_owned(),
        children: Vec::default(),
    };
    assert_eq!(data.r#type, expected.r#type);
    assert_eq!(data.note, expected.note);
    assert_eq!(data.name, expected.name);
    let expected = DocumentationItemChild {
        datatype: "a,".to_owned(),
        note: "This is A".to_owned(),
        additional_data: "".to_owned(),
        ..Default::default()
    };
    let child = data.children[0].clone();
    assert_eq!(child.datatype, expected.datatype);
    assert_eq!(child.note, expected.note);

    let expected = DocumentationItemChild {
        datatype: "b,".to_owned(),
        note: "This is B".to_owned(),
        additional_data: "".to_owned(),
        ..Default::default()
    };
    let child = data.children[1].clone();
    assert_eq!(child.datatype, expected.datatype);
    assert_eq!(child.note, expected.note);
}

#[test]
fn parse_file_with_defines_test() {
    let data = parse_file(PathBuf::from("test_data/test2.h"), encoding::all::UTF_8).unwrap();
    assert_eq!(data.items.len(), 1);
    let data = data.items[0].clone();
    assert_eq!(data.children.len(), 2);
    let expected = DocumentationItem {
        r#type: DocumentationType::Struct,
        note: "Test struct".to_owned(),
        name: "test".to_owned(),
        children: Vec::default(),
    };
    assert_eq!(data.r#type, expected.r#type);
    assert_eq!(data.note, expected.note);
    assert_eq!(data.name, expected.name);
    let expected = DocumentationItemChild {
        datatype: "int a;".to_owned(),
        note: "This is A".to_owned(),
        additional_data: "".to_owned(),
        ..Default::default()
    };
    let child = data.children[0].clone();
    assert_eq!(child.datatype, expected.datatype);
    assert_eq!(child.note, expected.note);

    let expected = DocumentationItemChild {
        datatype: "int b;".to_owned(),
        note: "This is B".to_owned(),
        additional_data: "".to_owned(),
        ..Default::default()
    };
    let child = data.children[1].clone();
    assert_eq!(child.datatype, expected.datatype);
    assert_eq!(child.note, expected.note);
}

#[test]
fn parse_file_with_additional_note_test() {
    let data = parse_file(PathBuf::from("test_data/test5.h"), encoding::all::UTF_8).unwrap();
    assert_eq!(data.items.len(), 2);
    let data = data.items[1].clone();
    assert_eq!(data.children.len(), 2);
    let expected = DocumentationItem {
        r#type: DocumentationType::Struct,
        note: "Test struct".to_owned(),
        name: "test".to_owned(),
        children: Vec::default(),
        ..Default::default()
    };
    assert_eq!(data.r#type, expected.r#type);
    assert_eq!(data.note, expected.note);
    assert_eq!(data.name, expected.name);
    let expected = DocumentationItemChild {
        datatype: "int a;".to_owned(),
        note: "This is A".to_owned(),
        additional_data: "".to_owned(),
        ..Default::default()
    };
    let child = data.children[0].clone();
    assert_eq!(child.datatype, expected.datatype);
    assert_eq!(child.note, expected.note);

    let expected = DocumentationItemChild {
        datatype: "int b;".to_owned(),
        note: "This is B".to_owned(),
        additional_data: "".to_owned(),
        ..Default::default()
    };
    let child = data.children[1].clone();
    assert_eq!(child.datatype, expected.datatype);
    assert_eq!(child.note, expected.note);
}
