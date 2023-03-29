use std::{fs::{ OpenOptions}, path::PathBuf};
use encoding::{Encoding, DecoderTrap};


#[derive(Default,Debug,Clone)]
enum DocumentationType{
    #[default]
    Struct,
    Enum,
    Define
}

#[derive(Default,Debug,Clone)]
struct DocumentationItemChild{
    datatype : String,
    note : String
}

#[derive(Default,Debug,Clone)]
struct DocumentationItem{
    item_type : DocumentationType,
    name : String,
    children : Vec<DocumentationItemChild>
}


struct DocumentationData{
    
}

fn parse_file(path: PathBuf,encoding : Box<dyn Encoding>) -> Result<DocumentationData,std::io::Error>{
    let file = OpenOptions::new().read(true).open(path)?;

    // encoding.decode(DecoderTrap::Ignore).unwrap();
    todo!();
}
