use std::{collections::HashMap, fmt::Display};

use crate::{cursor::Cursor, error::Error, tags::Tag};
#[derive(Debug, Clone)]
pub struct FeatureList {
    pub features: Vec<Feature>,
}
impl FeatureList {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let base = cursor.position();
        let feature_count = cursor.read_u16()?;
        let mut features = Vec::with_capacity(feature_count as usize);
        for _ in 0..feature_count {
            let tag = Tag::from(cursor.read_u32()?);
            let feature_offset = cursor.read_u16()?;
            let saved = cursor.position();
            let feature_start = feature_offset as usize + base;
            cursor.seek(feature_start)?;
            let feature_params_offset = cursor.read_u16()?;
            let lookup_index_count = cursor.read_u16()?;
            let mut lookup_list_indices = Vec::with_capacity(lookup_index_count as usize);
            for _ in 0..lookup_index_count {
                lookup_list_indices.push(cursor.read_u16()?);
            }
            let feature_params = if feature_params_offset == 0 {
                None
            } else {
                cursor.seek(feature_start + feature_params_offset as usize)?;
                Some(0)
            };
            features.push(Feature {
                tag,
                feature_params,
                lookup_list_indices,
            });
            cursor.seek(saved)?;
        }
        Ok(Self { features })
    }
}
#[derive(Debug, Clone)]
pub struct Feature {
    ///Placeholder for now. Unsure how to implement this.
    pub feature_params: Option<i32>,
    pub lookup_list_indices: Vec<u16>,
    pub tag: Tag,
}
impl Display for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "lookup_indices: {:?} tag: {}",
            self.lookup_list_indices, self.tag
        )
    }
}
