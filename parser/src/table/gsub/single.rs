use crate::{common::coverage::Coverage, cursor::Cursor, error::Error};
#[derive(Debug, Clone)]
pub enum SingleSubsitution {
    Format1 {
        coverage: Coverage,
        delta_glyph_id: u16,
    },
    Format2 {
        coverage: Coverage,
        subsitute_glyph_ids: Vec<u16>,
    },
}
impl SingleSubsitution {
    pub fn parse(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let format = cursor.read_u16()?;
        let coverage_offset = cursor.read_u16()?;
        let saved_pos = cursor.position();
        cursor.seek(coverage_offset as usize + base)?;
        let coverage = Coverage::parse(cursor)?;
        cursor.seek(saved_pos)?;
        Ok(match format {
            1 => {
                let delta_glyph_id = cursor.read_u16()?;
                Self::Format1 {
                    coverage,
                    delta_glyph_id,
                }
            }
            2 => {
                let glyph_count = cursor.read_u16()?;
                let mut subsitute_glyph_ids = Vec::with_capacity(glyph_count as usize);
                for _ in 0..glyph_count {
                    subsitute_glyph_ids.push(cursor.read_u16()?);
                }
                Self::Format2 {
                    coverage,
                    subsitute_glyph_ids,
                }
            }
            _ => {
                return Err(Error::Unknown);
            }
        })
    }
}
