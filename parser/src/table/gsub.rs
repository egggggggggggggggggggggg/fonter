use crate::{TableRecord, cursor::Cursor, error::Error};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Substitution {
    Single(SingleSubstitution),
    Multiple(MultipleSubstitution),
    Alternate(AlternateSubstitution),
    Ligature(LigatureSubstitution),
    Contextual(ContextualSubstitution),
    ChainedContextual(ChainedContextualSubstitution),
    Extension(ExtensionSubstitution),
    ReverseChainingContextSingle(ReverseChainingContextSingleSubstitution),
}
#[derive(Debug, Clone)]
pub enum SingleSubstitution {
    Format1 {
        coverage: Coverage,
        delta_glyph_id: i16,
    },
    Format2 {
        coverage: Coverage,
        substitute_glyph_ids: Vec<u16>,
    },
}
#[derive(Debug, Clone)]
pub struct MultipleSubstitution {
    pub coverage: Coverage,
    pub sequences: Vec<Vec<u16>>, // each input glyph → sequence of glyphs
}
#[derive(Debug, Clone)]
pub struct AlternateSubstitution {
    pub coverage: Coverage,
    pub alternate_sets: Vec<Vec<u16>>, // each glyph → list of alternates
}
#[derive(Debug, Clone)]
pub struct LigatureSubstitution {
    pub coverage: Coverage,
    pub ligature_sets: Vec<Vec<Ligature>>,
}

#[derive(Debug, Clone)]
pub struct Ligature {
    pub ligature_glyph: u16,
    pub component_glyphs: Vec<u16>,
}
#[derive(Debug, Clone)]
pub enum ContextualSubstitution {
    Format1 {
        coverage: Coverage,
        rule_sets: Vec<Vec<SequenceRule>>,
    },
    Format2 {
        coverage: Coverage,
        class_def: ClassDef,
        class_sets: Vec<Vec<ClassSequenceRule>>,
    },
    Format3 {
        coverages: Vec<Coverage>,
        lookup_records: Vec<LookupRecord>,
    },
}
#[derive(Debug, Clone)]
pub enum ChainedContextualSubstitution {
    Format1 {
        coverage: Coverage,
        rule_sets: Vec<Vec<ChainedSequenceRule>>,
    },
    Format2 {
        coverage: Coverage,
        backtrack_class_def: ClassDef,
        input_class_def: ClassDef,
        lookahead_class_def: ClassDef,
        class_sets: Vec<Vec<ChainedClassSequenceRule>>,
    },
    Format3 {
        backtrack_coverages: Vec<Coverage>,
        input_coverages: Vec<Coverage>,
        lookahead_coverages: Vec<Coverage>,
        lookup_records: Vec<LookupRecord>,
    },
}

#[derive(Debug, Clone)]
pub struct ExtensionSubstitution {
    pub extension_lookup_type: u16,
    pub extension: Box<Substitution>,
}

#[derive(Debug, Clone)]
pub struct ReverseChainingContextSingleSubstitution {
    pub coverage: Coverage,
    pub backtrack_coverages: Vec<Coverage>,
    pub lookahead_coverages: Vec<Coverage>,
    pub substitutes: Vec<u16>,
}
#[derive(Debug, Clone)]
pub struct Coverage {
    pub glyphs: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct ClassDef {
    pub classes: Vec<(u16, u16)>, // (glyph, class)
}
#[derive(Debug, Clone)]
pub struct SequenceRule {
    pub input_sequence: Vec<u16>,
    pub lookup_records: Vec<LookupRecord>,
}

#[derive(Debug, Clone)]
pub struct ClassSequenceRule {
    pub input_classes: Vec<u16>,
    pub lookup_records: Vec<LookupRecord>,
}

#[derive(Debug, Clone)]
pub struct ChainedSequenceRule {
    pub backtrack: Vec<u16>,
    pub input: Vec<u16>,
    pub lookahead: Vec<u16>,
    pub lookup_records: Vec<LookupRecord>,
}

#[derive(Debug, Clone)]
pub struct ChainedClassSequenceRule {
    pub backtrack: Vec<u16>,
    pub input: Vec<u16>,
    pub lookahead: Vec<u16>,
    pub lookup_records: Vec<LookupRecord>,
}
#[derive(Debug, Clone)]
pub struct LookupRecord {
    pub sequence_index: u16,
    pub lookup_list_index: u16,
}
pub struct Gsub {}
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

        Ok(Self {})
    }
}
