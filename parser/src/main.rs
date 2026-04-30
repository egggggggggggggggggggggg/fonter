use std::io;

use parser::{
    TtfFont,
    tags::{self, Tag},
};

fn main() -> io::Result<()> {
    let mut font = TtfFont::new("../jet.ttf").unwrap();
    println!("Tables: {:?}", font.tables);
    let mut gsub = font.parse_gsub().unwrap();
    gsub.get_substitution(Tag::from(*tags::script::DEFAULT), None, Tag::from(*b"calt"))
        .unwrap();
    Ok(())
}
