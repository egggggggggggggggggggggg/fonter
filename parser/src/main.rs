use std::io;

use parser::{
    TtfFont,
    tags::{self, Tag},
};

fn main() -> io::Result<()> {
    let mut font = TtfFont::new("../jet.ttf").unwrap();
    let mut gsub = font.parse_gsub().unwrap();
    gsub.get_substitution(Tag::from(*tags::script::LATIN), None, Tag::from(*b"liga"))
        .unwrap();
    Ok(())
}
