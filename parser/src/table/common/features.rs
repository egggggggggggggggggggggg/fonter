use crate::{cursor::Cursor, error::Error};

pub struct FeatureList {
    feature_count: u16,
    feature_records: Vec<FeatureRecord>,
}
impl FeatureList {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let feature_count = cursor.read_u16()?;
        let mut feature_records = Vec::with_capacity(feature_count as usize);
        for _ in 0..feature_count {
            feature_records.push(FeatureRecord::parse(cursor)?);
        }
        Ok(Self {
            feature_count,
            feature_records,
        })
    }
}
pub struct FeatureRecord {
    feature_tag: [u8; 4],
    feature_offset: u16,
}
impl FeatureRecord {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let feature_tag = cursor.read_u32()?.to_be_bytes();
        let feature_offset = cursor.read_u16()?;
        Ok(Self {
            feature_tag,
            feature_offset,
        })
    }
}
pub struct Feature {
    feature_params_offset: u16,
    lookup_index_count: u16,
    lookup_list_indices: Vec<u16>,
}
impl Feature {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let feature_params_offset = cursor.read_u16()?;
        let lookup_index_count = cursor.read_u16()?;
        let mut lookup_list_indices = Vec::with_capacity(lookup_index_count as usize);
        for _ in 0..lookup_index_count {
            lookup_list_indices.push(cursor.read_u16()?);
        }
        Ok(Self {
            feature_params_offset,
            lookup_index_count,
            lookup_list_indices,
        })
    }
}
