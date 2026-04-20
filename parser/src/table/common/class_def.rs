use crate::{cursor::Cursor, error::Error};
pub enum ClassDefinition {
    Format1 {
        start_glyph_id: u16,
        glyph_count: u16,
        class_values: Vec<u16>,
    },
    Format2 {
        class_range_count: u16,
        class_range_records: Vec<ClassRange>,
    },
}
impl ClassDefinition {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let format = cursor.read_u16()?;
        Ok(match format {
            1 => {
                let start_glyph_id = cursor.read_u16()?;
                let glyph_count = cursor.read_u16()?;
                let mut class_values = Vec::new();
                for _ in 0..glyph_count {
                    class_values.push(cursor.read_u16()?);
                }
                Self::Format1 {
                    start_glyph_id,
                    glyph_count,
                    class_values,
                }
            }
            2 => {
                let class_range_count = cursor.read_u16()?;
                let mut class_range_records = Vec::new();
                for _ in 0..class_range_count {
                    class_range_records.push(ClassRange::parse(cursor)?);
                }
                Self::Format2 {
                    class_range_count,
                    class_range_records,
                }
            }
            _ => {
                return Err(Error::Unknown);
            }
        })
    }
}
pub struct ClassRange {
    start_glyph_id: u16,
    end_glyph_id: u16,
    class: u16,
}
impl ClassRange {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        Ok(Self {
            start_glyph_id: cursor.read_u16()?,
            end_glyph_id: cursor.read_u16()?,
            class: cursor.read_u16()?,
        })
    }
}
