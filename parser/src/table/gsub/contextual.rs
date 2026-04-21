use crate::{cursor::Cursor, error::Error};

#[derive(Debug, Clone)]
pub struct ContextualSubstitution {}
impl ContextualSubstitution {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        todo!()
    }
}
