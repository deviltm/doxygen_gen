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
    pub note: String,
    pub additional_data: String,
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
    let additional_data_regex = additional_data_regex();

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
                    let mut data = "-";
                    if note.contains(" //") {
                        if let Some(captures) = additional_data_regex.captures(note) {
                            note = captures.get(1).unwrap().as_str();
                            data = captures.get(2).unwrap().as_str();
                        }
                    }
                    curret_item.children.push(DocumentationItemChild {
                        datatype: captures.get(1).unwrap().as_str().to_owned(),
                        note: note.to_owned(),
                        additional_data: data.to_owned(),
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
    };
    let child = data.children[0].clone();
    assert_eq!(child.datatype, expected.datatype);
    assert_eq!(child.note, expected.note);

    let expected = DocumentationItemChild {
        datatype: "int b;".to_owned(),
        note: "This is B".to_owned(),
        additional_data: "".to_owned(),
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
    };
    let child = data.children[0].clone();
    assert_eq!(child.datatype, expected.datatype);
    assert_eq!(child.note, expected.note);

    let expected = DocumentationItemChild {
        datatype: "int b;".to_owned(),
        note: "This is B".to_owned(),
        additional_data: "".to_owned(),
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
    };
    let child = data.children[0].clone();
    assert_eq!(child.datatype, expected.datatype);
    assert_eq!(child.note, expected.note);

    let expected = DocumentationItemChild {
        datatype: "b,".to_owned(),
        note: "This is B".to_owned(),
        additional_data: "".to_owned(),
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
    };
    let child = data.children[0].clone();
    assert_eq!(child.datatype, expected.datatype);
    assert_eq!(child.note, expected.note);

    let expected = DocumentationItemChild {
        datatype: "int b;".to_owned(),
        note: "This is B".to_owned(),
        additional_data: "".to_owned(),
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
    };
    assert_eq!(data.r#type, expected.r#type);
    assert_eq!(data.note, expected.note);
    assert_eq!(data.name, expected.name);
    let expected = DocumentationItemChild {
        datatype: "int a;".to_owned(),
        note: "This is A".to_owned(),
        additional_data: "".to_owned(),
    };
    let child = data.children[0].clone();
    assert_eq!(child.datatype, expected.datatype);
    assert_eq!(child.note, expected.note);

    let expected = DocumentationItemChild {
        datatype: "int b;".to_owned(),
        note: "This is B".to_owned(),
        additional_data: "".to_owned(),
    };
    let child = data.children[1].clone();
    assert_eq!(child.datatype, expected.datatype);
    assert_eq!(child.note, expected.note);
}
