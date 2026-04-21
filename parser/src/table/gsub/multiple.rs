use crate::{common::coverage::Coverage, cursor::Cursor};
#[derive(Debug, Clone)]
pub struct MultipleSubsitution {
    coverage: Coverage,
    sequences: Vec<Sequence>,
}
impl MultipleSubsitution {
    pub fn parse(cursor: &mut Cursor, base: usize) {}
}
#[derive(Debug, Clone)]
pub struct Sequence {}
