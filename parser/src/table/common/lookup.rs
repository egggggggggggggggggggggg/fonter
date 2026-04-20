use crate::{cursor::Cursor, error::Error};
use bitflags::bitflags;

bitflags! {
    pub struct LookupFlag: u16 {
        const RIGHT_TO_LEFT =  0x0001;
        const IGNORE_BASE_GLYPHS = 0x0002;
        const IGNORE_LIGATURES = 0x0004;
        const IGNORE_MARKS = 0x0008;
        const USE_MARK_FILTERING_SET = 0x0010;
        const RESERVED = 0x00E0;
        const MARK_ATTACHMENT_CLASS_FILTER = 0xFF00;
    }
}
pub struct Lookup {
    lookup_type: u16,
    lookup_flag: LookupFlag,
    sub_table_count: u16,
    sub_table_offsets: Vec<u16>,
    mark_filtering_set: Option<u16>,
}
impl Lookup {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let lookup_type = cursor.read_u16()?;
        let lookup_flag = LookupFlag::from_bits_truncate(cursor.read_u16()?);
        let sub_table_count = cursor.read_u16()?;
        let mut sub_table_offsets = Vec::new();
        for _ in 0..sub_table_count {
            sub_table_offsets.push(cursor.read_u16()?);
        }
        let mark_filtering_set = if lookup_flag.contains(LookupFlag::USE_MARK_FILTERING_SET) {
            Some(cursor.read_u16()?)
        } else {
            None
        };
        Ok(Self {
            lookup_type,
            lookup_flag,
            sub_table_count,
            sub_table_offsets,
            mark_filtering_set,
        })
    }
}
