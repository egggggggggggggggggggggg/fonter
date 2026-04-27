se crate::{cursor::Cursor, error::Error};
#[derive(Debug, Clone)]
pub enum Coverage {
    Format1 {
        glyph_count: u16,
        glyph_array: Vec<u16>,
    },
    Format2 {
        range_count: u16,
        range_records: Vec<RangeRecord>,
    },
}
impl Coverage {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let format = cursor.read_u16()?;
        Ok(match format {
            1 => {
                let glyph_count = cursor.read_u16()?;
                let mut glyph_array = Vec::new();
                for _ in 0..glyph_count {
                    glyph_array.push(cursor.read_u16()?);
                }
                Coverage::Format1 {
                    glyph_count,
                    glyph_array,
                }
            }
            2 => {
                let range_count = cursor.read_u16()?;
                let mut range_records = Vec::new();
                for _ in 0..range_count {
                    range_records.push(RangeRecord::parse(cursor)?);
                }
                Coverage::Format2 {
                    range_count,
                    range_records,
                }
            }
            _ => {
                return Err(Error::Unknown);
            }
        })
    }
    ///For a given glyph_id returns the coverage_index associated with it if it can be found.
    pub fn coverage_index(&self, glyph_id: u16) -> Option<u16> {
        match self {
            Coverage::Format1 { glyph_array, .. } => glyph_array
                .binary_search(&glyph_id)
                .ok()
                .map(|idx| idx as u16),
            Coverage::Format2 { range_records, .. } => {
                let mut left = 0;
                let mut right = range_records.len();
                while left < right {
                    let mid = (left + right) / 2;
                    let range = &range_records[mid];
                    if glyph_id < range.start_glyph_id {
                        right = mid;
                    } else if glyph_id > range.end_glyph_id {
                        left = mid + 1;
                    } else {
                        return Some(
                            range.start_coverage_index + (glyph_id - range.start_glyph_id),
                        );
                    }
                }
                None
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct RangeRecord {
    start_glyph_id: u16,
    end_glyph_id: u16,
    start_coverage_index: u16,
}
impl RangeRecord {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        Ok(Self {
            start_glyph_id: cursor.read_u16()?,
            end_glyph_id: cursor.read_u16()?,
            start_coverage_index: cursor.read_u16()?,
        })
    }
}
