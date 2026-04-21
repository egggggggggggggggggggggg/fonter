use crate::{cursor::Cursor, error::Error};

pub struct FeatureVariations {
    major: u16,
    minor: u16,
}
impl FeatureVariations {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        todo!()
    }
}
