use crate::{cursor::Cursor, error::Error};
pub struct LangSysRecord {
    pub lang_sys_tag: [u8; 4],
    pub lang_sys: LangSys,
}
impl LangSysRecord {
    ///Base is the offset of the ScriptTable.
    pub fn parse(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let lang_sys_tag = cursor.read_u32()?.to_be_bytes();
        let lang_sys_offset = cursor.read_u16()?;
        cursor.seek(base + lang_sys_offset as usize)?;
        let lang_sys = LangSys::parse(cursor)?;
        Ok(Self {
            lang_sys_tag,
            lang_sys,
        })
    }
}
#[derive(Debug, Clone)]
pub struct LangSys {
    pub lookup_order_offset: u16,
    pub required_feature_index: u16,
    pub feature_indices: Vec<u16>,
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
            feature_indices,
        })
    }
}
