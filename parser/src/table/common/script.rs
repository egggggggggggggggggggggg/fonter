use crate::{cursor::Cursor, error::Error};

pub struct ScriptList {
    script_count: u16,
    script_records: Vec<ScriptRecords>,
}
pub struct ScriptRecords {
    script_tag: [u8; 4],
    script_offset: u16,
}
impl ScriptRecords {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let script_tag = cursor.read_u32()?.to_be_bytes();
        let script_offset = cursor.read_u16()?;
        Ok(Self {
            script_tag,
            script_offset,
        })
    }
}
impl ScriptList {
    ///Prior to calling this, cursor must be alligned to the offset specified for the table.
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let script_count = cursor.read_u16()?;
        let mut script_records = Vec::with_capacity(script_count as usize);
        for _ in 1..script_count {
            script_records.push(ScriptRecords::parse(cursor)?);
        }
        Ok(Self {
            script_count,
            script_records,
        })
    }
}
pub struct Script {}
pub struct LookupList {
    lookup_count: u16,
    lookup_offsets: Vec<u16>,
}
