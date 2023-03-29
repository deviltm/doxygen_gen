use encoding::{DecoderTrap, Encoding};
use std::{fs::OpenOptions, io::Read, path::PathBuf};

#[derive(Default, Debug, Clone)]
pub enum DocumentationType {
    #[default]
    Struct,
    Enum,
}

#[derive(Default, Debug, Clone)]
pub struct DocumentationItemChild {
    datatype: String,
    note: String,
}

#[derive(Default, Debug, Clone)]
pub struct DocumentationItem {
    item_type: DocumentationType,
    name: String,
    children: Vec<DocumentationItemChild>,
}

#[derive(Default, Debug, Clone)]
pub struct DocumentationData {
    //Potentially add other data here
    items: Vec<DocumentationItem>,
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
    let mut curret_item: Option<DocumentationItem> = None;

    for line in contents.lines() {
        //Encountered a struct/enum definition
        if line.contains("//! ") {
            let mut i = DocumentationItem::default();
            i.name = line[4..].to_owned();
            curret_item = Some(i);
        }else if let Some(item) = curret_item.as_mut(){

        }
    }

    Ok(data)
}
