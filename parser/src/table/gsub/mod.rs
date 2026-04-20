use std::collections::HashMap;

use crate::{
    TableRecord,
    common::lookup::{self, Lookup, LookupList},
    cursor::Cursor,
    error::Error,
    gsub::{alternate::AlternateSubstitution, single::SingleSubsitution},
};

pub mod alternate;
pub mod chained;
pub mod contextual;
pub mod extension;
pub mod ligature;
pub mod multiple;
pub mod reverse;
pub mod single;
#[derive(Debug, Clone)]
pub enum Substitution {
    Single(SingleSubsitution),
    Multiple(MultipleSubstitution),
    Alternate(AlternateSubstitution),
    Ligature(LigatureSubstitution),
    Contextual(ContextualSubstitution),
    ChainedContextual(ChainedContextualSubstitution),
    Extension(ExtensionSubstitution),
    ReverseChainingContextSingle(ReverseChainingContextSingleSubstitution),
}
impl Substitution {
    pub fn parse(cursor: &mut Cursor, lookup_type: u16) -> Result<Self, Error> {
        let base = cursor.position();
        Ok(match lookup_type {
            1 => Self::Single(SingleSubsitution::parse(cursor, base)?);
            2 => Self::
            3 => {}
           4 => {}
            5 => {}
            6 => {}
            7 => {}
            8 => {}
            _ => {
                return Err(Error::Unknown);
            }
        })
    }
}
pub struct Gsub {
    lookup: Lookup,
    subsitutions: Vec<Substitution>,
}
impl Gsub {
    pub fn parse(data: &[u8], tables: &HashMap<[u8; 4], TableRecord>) -> Result<Self, Error> {
        let rec = tables.get(b"GSUB").ok_or(Error::MissingTable("GSUB"))?;
        let mut cursor = Cursor::set(data, rec.table_offset);
        let major = cursor.read_u16()?;
        let minor = cursor.read_u16()?;
        let script_list_offset = cursor.read_u16()?;
        let feature_list_offset = cursor.read_u16()?;
        let lookup_list_offset = cursor.read_u16()?;
        let feature_variation_offset = if minor == 1 {
            Some(cursor.read_u16()?)
        } else {
            None
        };
        cursor.seek(lookup_list_offset as usize)?;
        let lookup_list = LookupList::parse(&mut cursor)?;
        let sub_allocation = lookup_list.total_sub_tables();
        let mut subsitutions = Vec::with_capacity(sub_allocation);
        for lookup in &lookup_list.lookups {
            for sub_table_offset in lookup.sub_table_offsets{
                
            }
        }
        Ok(Self {
            lookup,
            subsitutions,
        })
    }
}
