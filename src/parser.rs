use crate::regex::*;
use encoding::{DecoderTrap, Encoding};
use std::{fs::OpenOptions, io::Read, path::PathBuf};

enum ParsingState {
    None,
    Name,
    Fields,
}

#[derive(Default, Debug, Clone)]
pub enum DocumentationType {
    #[default]
    Struct,
    Enum,
}

#[derive(Default, Debug, Clone)]
pub struct DocumentationItemChild {
    pub datatype: String,
    pub note: String,
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
    items: Vec<DocumentationItem>,
}

impl Iterator for DocumentationData {
    type Item = DocumentationItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.items.pop()
    }
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

    for line in contents.lines() {
        if line.is_empty(){
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
                    curret_item.children.push(DocumentationItemChild {
                        datatype: captures.get(1).unwrap().as_str().to_owned(),
                        note: captures.get(2).unwrap().as_str().to_owned(),
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
