use crate::{cursor::Cursor, error::Error, gsub::AlternateSubstitution};

pub struct SequenceLookup {
    sequence_index: u16,
    lookup_list_index: u16,
}
impl SequenceLookup {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        Ok(Self {
            sequence_index: cursor.read_u16()?,
            lookup_list_index: cursor.read_u16()?,
        })
    }
}
pub enum SequenceContext {
    Format1 {
        coverage_offset: u16,
        seq_rule_set_count: u16,
        seq_rule_set_offsets: Vec<u16>,
    },
    Format2 {
        coverage_offset: u16,
        class_def_offset: u16,
        class_seq_rule_count: u16,
        class_seq_rule_offsets: Vec<u16>,
    },
}
impl SequenceContext {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let format = cursor.read_u16()?;
        let coverage_offset = cursor.read_u16()?;
        Ok(match format {
            1 => {
                let seq_rule_set_count = cursor.read_u16()?;
                let mut seq_rule_set_offsets = Vec::with_capacity(seq_rule_set_count as usize);
                for _ in 0..seq_rule_set_count {
                    seq_rule_set_offsets.push(cursor.read_u16()?);
                }
                Self::Format1 {
                    coverage_offset,
                    seq_rule_set_count,
                    seq_rule_set_offsets,
                }
            }
            2 => {
                let class_def_offset = cursor.read_u16()?;
                let class_seq_rule_count = cursor.read_u16()?;
                let mut class_seq_rule_offsets = Vec::with_capacity(class_seq_rule_count as usize);
                for _ in 0..class_seq_rule_count {
                    class_seq_rule_offsets.push(cursor.read_u16()?);
                }
                Self::Format2 {
                    coverage_offset,
                    class_def_offset,
                    class_seq_rule_count,
                    class_seq_rule_offsets,
                }
            }
            _ => return Err(Error::Unknown),
        })
    }
}
pub struct SequenceRuleSet {
    seq_rule_count: u16,
    seq_rule_offsets: Vec<u16>,
}
impl SequenceRuleSet {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let seq_rule_count = cursor.read_u16()?;
        let mut seq_rule_offsets = Vec::with_capacity(seq_rule_count as usize);
        for _ in 0..seq_rule_count {
            seq_rule_offsets.push(cursor.read_u16()?);
        }
        Ok(Self {
            seq_rule_count,
            seq_rule_offsets,
        })
    }
}
