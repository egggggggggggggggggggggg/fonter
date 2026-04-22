use parser::{TtfFont, error::Error};

fn main() -> Result<(), Error> {
    let path = "jet.ttf";
    let mut font = TtfFont::new(path)?;
    let gsub = font.parse_gsub()?;
    Ok(())
}
