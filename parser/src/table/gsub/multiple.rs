use std::net::UdpSocket;

use crate::{common::coverage::Coverage, cursor::Cursor, error::Error};
#[derive(Debug, Clone)]
pub struct MultipleSubsitution {
    pub format: u16,
    pub coverage: Coverage,
    pub sequences: Vec<Sequence>,
}

impl MultipleSubsitution {
    pub fn parse(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let format = cursor.read_u16()?;
        let coverage_offset = cursor.read_u16()?;
        let saved = cursor.position();
        cursor.seek(coverage_offset as usize + base)?;
        let coverage = Coverage::parse(cursor)?;
        cursor.seek(saved)?;
        let sequence_count = cursor.read_u16()?;
        let mut sequence_offsets = Vec::with_capacity(sequence_count as usize);
        for _ in 0..sequence_count {
            sequence_offsets.push(cursor.read_u16()?);
        }
        let mut sequences = Vec::with_capacity(sequence_count as usize);
        for offset in sequence_offsets {
            cursor.seek(offset as usize)?;
            sequences.push(Sequence::parse(cursor)?);
        }
        Ok(Self {
            format,
            coverage,
            sequences,
        })
    }
}
#[derive(Debug, Clone)]
pub struct Sequence {
    subsitute_glyph_ids: Vec<u16>,
}
impl Sequence {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let glyph_count = cursor.read_u16()?;
        let mut subsitute_glyph_ids = Vec::with_capacity(glyph_count as usize);
        for _ in 0..glyph_count {
            subsitute_glyph_ids.push(cursor.read_u16()?);
        }
        Ok(Self {
            subsitute_glyph_ids,
        })
    }
}
