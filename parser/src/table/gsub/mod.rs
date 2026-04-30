use crate::{
    TableRecord,
    common::{
        features::FeatureList, lookup::LookupList, script::ScriptList, variation::FeatureVariations,
    },
    cursor::Cursor,
    error::Error,
    gsub::sub::Substitution,
    tags::Tag,
};
use std::collections::HashMap;
pub mod sub;
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
        feature_tag: Tag,
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
        let mut requested_feature = None;
        for index in &lang_sys.feature_indices {
            let feature = &self.feature_list.features[*index as usize];
            if feature.tag == feature_tag {
                requested_feature = Some(feature);
                break;
            }
        }
        if let Some(found_feature) = requested_feature {
            for index in &found_feature.lookup_list_indices {
                let lookup = &self.lookup_list.lookups[*index as usize];
                for subtable_index in &lookup.sub_table_offsets {
                    let new_index = *subtable_index as usize + lookup.offset;
                    println!(
                        "Attempting to seek here: {} with this type: {}",
                        new_index, lookup.lookup_type
                    );
                    cursor.seek(*subtable_index as usize + lookup.offset)?;
                    let sub = Substitution::parse(&mut cursor, lookup.lookup_type)?;
                    println!("Substitution: {:?}", sub);
                }
            }
        } else {
            return Err(Error::AnyMessage("Failed to get the requested feature."));
        }

        println!("lang_sys: {:?}", lang_sys);
        Ok(())
    }
}
