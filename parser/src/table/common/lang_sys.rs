use crate::{cursor::Cursor, error::Error};
pub struct LangSysRecord {
    lang_sys_tag: [u8; 4],
    lang_sys_offset: u16,
}
impl LangSysRecord {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let lang_sys_tag = cursor.read_u32()?.to_be_bytes();
        let lang_sys_offset = cursor.read_u16()?;
        Ok(Self {
            lang_sys_tag,
            lang_sys_offset,
        })
    }
}
pub struct LangSys {
    lookup_order_offset: u16,
    required_feature_index: u16,
    feature_index_count: u16,
    feature_indices: Vec<u16>,
}
impl LangSys {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        //Reserved. Maybe skip instead of reading it.
        let lookup_order_offset = cursor.read_u16()?;
        let required_feature_index = cursor.read_u16()?;
        let feature_index_count = cursor.read_u16()?;
        let mut feature_indices = Vec::new();
        for _ in 0..feature_index_count {
            feature_indices.push(cursor.read_u16()?);
        }
        Ok(Self {
            lookup_order_offset,
            required_feature_index,
            feature_index_count,
            feature_indices,
        })
    }
}
