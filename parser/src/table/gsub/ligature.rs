use crate::{common::coverage::Coverage, cursor::Cursor, error::Error};

#[derive(Debug, Clone)]
pub struct LigatureSubstitution {
    format: u16,
    coverage: Coverage,
    ligature_sets: Vec<LigatureSet>,
}
impl LigatureSubstitution {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let base = cursor.position();
        let format = cursor.read_u16()?;
        let coverage_offset = cursor.read_u16()?;
        let saved = cursor.position();
        cursor.seek(coverage_offset as usize + base);
        let coverage = Coverage::parse(cursor)?;
        cursor.seek(saved)?;
        let ligature_set_count = cursor.read_u16()?;
        let mut ligature_set_offsets = Vec::with_capacity(ligature_set_count as usize);
        for _ in 0..ligature_set_count {
            ligature_set_offsets.push(cursor.read_u16()?);
        }
        let mut ligature_sets = Vec::with_capacity(ligature_set_count as usize);
        for offset in ligature_set_offsets {
            cursor.seek(offset as usize)?;
            ligature_sets.push(LigatureSet::parse(cursor, base)?);
        }
        Ok(Self {
            format,
            coverage,
            ligature_sets,
        })
    }
}
#[derive(Debug, Clone)]
pub struct LigatureSet {
    ligatures: Vec<Ligature>,
}
impl LigatureSet {
    pub fn parse(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let ligature_count = cursor.read_u16()?;
        let mut ligature_offsets = Vec::with_capacity(ligature_count as usize);
        for _ in 0..ligature_count {
            ligature_offsets.push(cursor.read_u16()?);
        }
        let mut ligatures = Vec::with_capacity(ligature_count as usize);
        for offset in ligature_offsets {
            cursor.seek(offset as usize + base)?;
            ligatures.push(Ligature::parse(cursor)?);
        }
        Ok(Self { ligatures })
    }
}

#[derive(Debug, Clone)]
pub struct Ligature {
    ligature_glyph: u16,
    component_glyph_ids: Vec<u16>,
}

impl Ligature {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let ligature_glyph = cursor.read_u16()?;
        let component_count = cursor.read_u16()?;
        if component_count == 0 {
            return Err(Error::Unknown);
        }
        let mut component_glyph_ids = Vec::with_capacity((component_count - 1) as usize);
        for _ in 0..(component_count - 1) {
            component_glyph_ids.push(cursor.read_u16()?);
        }
        Ok(Self {
            ligature_glyph,
            component_glyph_ids,
        })
    }
}
