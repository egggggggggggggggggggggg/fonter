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
    tags::Tag,
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
#[derive(Hash)]
pub struct FeatureSetKey {}

pub struct Gsub<'a> {
    segment: &'a [u8],
    pub lookup_list: LookupList,
    pub script_list: ScriptList,
    pub feature_list: FeatureList,
    pub feature_variation_list: Option<FeatureVariations>,
    pub loaded_subsitutions: HashMap<&'static [u8; 4], Substitution>,
}
///Lots of dumb workaround stuff, to flesh out the general idea, will fix later.
impl<'a> Gsub<'a> {
    pub fn parse(data: &'a [u8], tables: &HashMap<[u8; 4], TableRecord>) -> Result<Self, Error> {
        let rec = tables.get(b"GSUB").ok_or(Error::MissingTable("GSUB"))?;
        let mut cursor = Cursor::set(data, rec.table_offset);
        let base = rec.table_offset as usize;
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
        cursor.seek(base + script_list_offset as usize)?;
        let script_list = ScriptList::parse(&mut cursor)?;
        cursor.seek(base + feature_list_offset as usize)?;
        let feature_list = FeatureList::parse(&mut cursor)?;
        cursor.seek(base + lookup_list_offset as usize)?;
        let lookup_list = LookupList::parse(&mut cursor)?;
        let feature_variation_list = if let Some(offset) = feature_variation_offset {
            cursor.seek(offset as usize)?;
            Some(FeatureVariations::parse(&mut cursor)?)
        } else {
            None
        };
        Ok(Self {
            segment: data,
            feature_variation_list,
            script_list,
            feature_list,
            lookup_list,
            loaded_subsitutions: HashMap::new(),
        })
    }
    pub fn get_substitution(
        &mut self,
        script_tag: Tag,
        lang_tag: Option<Tag>,
        feature: Tag,
    ) -> Result<(), Error> {
        let mut cursor = Cursor::set(self.segment, 0);
        let script = self.script_list.get_or_parse(&mut cursor, &script_tag)?;
        let lang_sys = match lang_tag {
            Some(tag) => {
                if let Some(lang) = script.lang_sys.get(&tag) {
                    lang
                } else {
                    return Err(Error::InvalidTag(tag));
                }
            }
            None => {
                if let Some(dflt) = &script.default_lang_sys {
                    dflt
                } else {
                    return Err(Error::InvalidFormat("This failed"));
                }
            }
        };
        println!("lang_sys: {:?}", lang_sys);
        Ok(())
    }
}
