use std::collections::HashMap;

use crate::{
    TableRecord,
    common::{
        features::FeatureList, lookup::LookupList, script::ScriptList, variation::FeatureVariations,
    },
    cursor::Cursor,
    error::Error,
    gsub::{
        alternate::AlternateSubstitution, chained::ChainedContextualSubstitution,
        contextual::ContextualSubstitution, extension::ExtensionSubstitution,
        ligature::LigatureSubstitution, multiple::MultipleSubsitution,
        reverse::ReverseSubstitution, single::SingleSubsitution,
    },
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
    Multiple(MultipleSubsitution),
    Alternate(AlternateSubstitution),
    Ligature(LigatureSubstitution),
    Contextual(ContextualSubstitution),
    ChainedContextual(ChainedContextualSubstitution),
    Extension(ExtensionSubstitution),
    Reverse(ReverseSubstitution),
}
impl Substitution {
    pub fn parse(cursor: &mut Cursor, lookup_type: u16) -> Result<Self, Error> {
        let base = cursor.position();
        Ok(match lookup_type {
            1 => Self::Single(SingleSubsitution::parse(cursor, base)?),
            2 => Self::Multiple(MultipleSubsitution::parse(cursor, base)?),
            3 => Self::Alternate(AlternateSubstitution::parse(cursor, base)?),
            4 => Self::Ligature(LigatureSubstitution::parse(cursor)?),
            5 => Self::Contextual(ContextualSubstitution::parse(cursor)?),
            6 => Self::ChainedContextual(ChainedContextualSubstitution::parse(cursor)?),
            7 => Self::Extension(ExtensionSubstitution::parse(cursor)?),
            8 => Self::Reverse(ReverseSubstitution::parse(cursor)?),
            _ => {
                return Err(Error::Unknown);
            }
        })
    }
}
pub struct Gsub<'a> {
    segment: &'a [u8],
    lookup_list: LookupList,
    script_list: ScriptList,
    feature_list: FeatureList,
    feature_variation_list: Option<FeatureVariations>,
    loaded_subsitutions: HashMap<&'static [u8; 4], Substitution>,
}
impl<'a> Gsub<'a> {
    pub fn parse(data: &'a [u8], tables: &HashMap<[u8; 4], TableRecord>) -> Result<Self, Error> {
        let rec = tables.get(b"GSUB").ok_or(Error::MissingTable("GSUB"))?;
        let segment =
            &data[rec.table_offset as usize..rec.table_offset as usize + rec.length as usize];
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
        cursor.seek(feature_list_offset as usize)?;
        let feature_list = FeatureList::parse(&mut cursor)?;
        cursor.seek(script_list_offset as usize)?;
        let script_list = ScriptList::parse(&mut cursor)?;
        let feature_variation_list = if let Some(offset) = feature_variation_offset {
            cursor.seek(offset as usize)?;
            Some(FeatureVariations::parse(&mut cursor)?)
        } else {
            None
        };
        Ok(Self {
            segment,
            feature_variation_list,
            script_list,
            feature_list,
            lookup_list,
            loaded_subsitutions: HashMap::new(),
        })
    }
    pub fn parse_script(tag: &[u8; 4]) -> Substitution {
        Substitution::ChainedContextual(ChainedContextualSubstitution {})
    }
}
