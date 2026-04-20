use crate::{common::coverage::Coverage, cursor::Cursor, error::Error};

#[derive(Debug, Clone)]
pub struct AlternateSubstitution {
    pub coverage: Coverage,
    pub alternate_sets: Vec<AlternateSet>, // each glyph → list of alternates
}
impl AlternateSubstitution {
    ///Takes a Cursor set to the current subtable offset + the start of the subsitution table offset
    pub fn parse(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let format = cursor.read_u16()?;
        let coverage_offset = cursor.read_u16()?;
        let saved_pos = cursor.position();
        cursor.seek(coverage_offset as usize + base)?;
        let coverage = Coverage::parse(cursor)?;
        cursor.seek(saved_pos)?;
        let alternate_set_count = cursor.read_u16()?;
        let mut alternate_sets = Vec::with_capacity(alternate_set_count as usize);
        for idx in 0..alternate_set_count {
            let offset = cursor.read_u16()?;
            let saved = cursor.position();
            cursor.seek(offset as usize + base)?;
            alternate_sets.push(AlternateSet::parse(cursor)?);
            cursor.seek(saved)?;
        }
        Ok(Self {
            coverage,
            alternate_sets,
        })
    }
}
#[derive(Debug, Clone)]
pub struct AlternateSet {
    alternate_glyph_ids: Vec<u16>,
}
impl AlternateSet {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let glyph_count = cursor.read_u16()?;
        let mut alternate_glyph_ids = Vec::with_capacity(glyph_count as usize);
        for _ in 0..glyph_count {
            alternate_glyph_ids.push(cursor.read_u16()?);
        }
        Ok(Self {
            alternate_glyph_ids,
        })
    }
}
