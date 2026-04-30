use crate::{
    common::{class_def::ClassDef, coverage::Coverage},
    cursor::Cursor,
    error::Error,
};

#[derive(Debug, Clone)]
pub enum Substitution {
    Single(SingleSubstitution),
    Multiple(MultipleSubstitution),
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
            1 => Self::Single(SingleSubstitution::parse(cursor, base)?),
            2 => Self::Multiple(MultipleSubstitution::parse(cursor, base)?),
            3 => Self::Alternate(AlternateSubstitution::parse(cursor, base)?),
            4 => Self::Ligature(LigatureSubstitution::parse(cursor)?),
            5 => Self::Contextual(ContextualSubstitution::parse(cursor, base)?),
            6 => Self::ChainedContextual(ChainedContextualSubstitution::parse(cursor, base)?),
            7 => Self::Extension(ExtensionSubstitution::parse(cursor)?),
            8 => Self::Reverse(ReverseSubstitution::parse(cursor, base)?),
            _ => {
                return Err(Error::AnyMessage("Unrecognized lookup  type"));
            }
        })
    }
}

#[derive(Debug, Clone)]
pub enum SingleSubstitution {
    Format1 {
        coverage: Coverage,
        delta_glyph_id: u16,
    },
    Format2 {
        coverage: Coverage,
        substitute_glyph_ids: Vec<u16>,
    },
}

impl SingleSubstitution {
    pub fn parse(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let format = cursor.read_u16()?;
        let coverage_offset = cursor.read_u16()?;
        let saved_pos = cursor.position();
        cursor.seek(coverage_offset as usize + base)?;
        let coverage = Coverage::parse(cursor)?;
        cursor.seek(saved_pos)?;
        Ok(match format {
            1 => {
                let delta_glyph_id = cursor.read_u16()?;
                Self::Format1 {
                    coverage,
                    delta_glyph_id,
                }
            }
            2 => {
                let glyph_count = cursor.read_u16()?;
                let mut substitute_glyph_ids = Vec::with_capacity(glyph_count as usize);
                for _ in 0..glyph_count {
                    substitute_glyph_ids.push(cursor.read_u16()?);
                }
                Self::Format2 {
                    coverage,
                    substitute_glyph_ids,
                }
            }
            _ => {
                return Err(Error::AnyMessage("Unrecognized single substitution"));
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct MultipleSubstitution {
    pub format: u16,
    pub coverage: Coverage,
    pub sequences: Vec<Sequence>,
}

impl MultipleSubstitution {
    pub fn parse(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let format = cursor.read_u16()?;
        let coverage_offset = cursor.read_u16()?;
        let saved = cursor.position();
        cursor.seek(coverage_offset as usize + base)?;
        let coverage = Coverage::parse(cursor)?;
        cursor.seek(saved)?;
        let sequence_count = cursor.read_u16()?;
        let mut sequence_offsets = Vec::with_capacity(sequence_count as usize);
        for _ in 0..sequence_count {
            sequence_offsets.push(cursor.read_u16()?);
        }
        let mut sequences = Vec::with_capacity(sequence_count as usize);
        for offset in sequence_offsets {
            cursor.seek(offset as usize + base)?;
            sequences.push(Sequence::parse(cursor)?);
        }
        Ok(Self {
            format,
            coverage,
            sequences,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Sequence {
    pub substitute_glyph_ids: Vec<u16>,
}

impl Sequence {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let glyph_count = cursor.read_u16()?;
        let mut substitute_glyph_ids = Vec::with_capacity(glyph_count as usize);
        for _ in 0..glyph_count {
            substitute_glyph_ids.push(cursor.read_u16()?);
        }
        Ok(Self {
            substitute_glyph_ids,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AlternateSubstitution {
    pub format: u16,
    pub coverage: Coverage,
    pub alternate_sets: Vec<AlternateSet>, // each glyph → list of alternates
}

impl AlternateSubstitution {
    /// Takes a Cursor set to the current subtable offset + the start of the substitution table offset
    pub fn parse(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let format = cursor.read_u16()?;
        let coverage_offset = cursor.read_u16()?;
        let saved_pos = cursor.position();
        cursor.seek(coverage_offset as usize + base)?;
        let coverage = Coverage::parse(cursor)?;
        cursor.seek(saved_pos)?;
        let alternate_set_count = cursor.read_u16()?;
        let mut alternate_sets = Vec::with_capacity(alternate_set_count as usize);
        for _ in 0..alternate_set_count {
            let offset = cursor.read_u16()?;
            let saved = cursor.position();
            cursor.seek(offset as usize + base)?;
            alternate_sets.push(AlternateSet::parse(cursor)?);
            cursor.seek(saved)?;
        }
        Ok(Self {
            format,
            coverage,
            alternate_sets,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AlternateSet {
    pub alternate_glyph_ids: Vec<u16>,
}

impl AlternateSet {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let glyph_count = cursor.read_u16()?;
        let mut alternate_glyph_ids = Vec::with_capacity(glyph_count as usize);
        for _ in 0..glyph_count {
            alternate_glyph_ids.push(cursor.read_u16()?);
        }
        Ok(Self {
            alternate_glyph_ids,
        })
    }
}

#[derive(Debug, Clone)]
pub struct LigatureSubstitution {
    pub format: u16,
    pub coverage: Coverage,
    pub ligature_sets: Vec<LigatureSet>,
}

impl LigatureSubstitution {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let base = cursor.position();
        let format = cursor.read_u16()?;
        let coverage_offset = cursor.read_u16()?;
        let saved = cursor.position();
        cursor.seek(coverage_offset as usize + base)?;
        let coverage = Coverage::parse(cursor)?;
        cursor.seek(saved)?;
        let ligature_set_count = cursor.read_u16()?;
        let mut ligature_set_offsets = Vec::with_capacity(ligature_set_count as usize);
        for _ in 0..ligature_set_count {
            ligature_set_offsets.push(cursor.read_u16()?);
        }
        let mut ligature_sets = Vec::with_capacity(ligature_set_count as usize);
        for offset in ligature_set_offsets {
            cursor.seek(offset as usize + base)?;
            ligature_sets.push(LigatureSet::parse(cursor, base)?);
        }
        Ok(Self {
            format,
            coverage,
            ligature_sets,
        })
    }
}

#[derive(Debug, Clone)]
pub struct LigatureSet {
    pub ligatures: Vec<Ligature>,
}

impl LigatureSet {
    pub fn parse(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let ligature_count = cursor.read_u16()?;
        let mut ligature_offsets = Vec::with_capacity(ligature_count as usize);
        for _ in 0..ligature_count {
            ligature_offsets.push(cursor.read_u16()?);
        }
        let mut ligatures = Vec::with_capacity(ligature_count as usize);
        for offset in ligature_offsets {
            cursor.seek(offset as usize + base)?;
            ligatures.push(Ligature::parse(cursor)?);
        }
        Ok(Self { ligatures })
    }
}

#[derive(Debug, Clone)]
pub struct Ligature {
    pub ligature_glyph: u16,
    pub component_glyph_ids: Vec<u16>,
}

impl Ligature {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let ligature_glyph = cursor.read_u16()?;
        let component_count = cursor.read_u16()?;
        if component_count == 0 {
            return Err(Error::AnyMessage("Unrecognized ligature"));
        }
        let mut component_glyph_ids = Vec::with_capacity((component_count - 1) as usize);
        for _ in 0..(component_count - 1) {
            component_glyph_ids.push(cursor.read_u16()?);
        }
        Ok(Self {
            ligature_glyph,
            component_glyph_ids,
        })
    }
}

/// Contextual Substitution - applies substitutions based on glyph context
#[derive(Debug, Clone)]
pub enum ContextualSubstitution {
    /// Format 1: Simple context glyph substitution
    Format1 {
        coverage: Coverage,
        subrule_sets: Vec<SubruleSet>,
    },
    /// Format 2: Class-based context glyph substitution
    Format2 {
        coverage: Coverage,
        class_def: Vec<u16>, // Simplified - typically would use a ClassDef structure
        subclass_sets: Vec<SubclassSet>,
    },
    /// Format 3: Coverage table-based context glyph substitution
    Format3 {
        glyph_count: u16,
        lookup_count: u16,
        coverages: Vec<Coverage>,
        lookup_records: Vec<LookupRecord>,
    },
}

impl ContextualSubstitution {
    pub fn parse(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let format = cursor.read_u16()?;
        match format {
            1 => Self::parse_format1(cursor, base),
            2 => Self::parse_format2(cursor, base),
            3 => Self::parse_format3(cursor, base),
            _ => Err(Error::AnyMessage("Unrecognized Contextual substitution")),
        }
    }

    fn parse_format1(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let coverage_offset = cursor.read_u16()?;
        let subrule_set_count = cursor.read_u16()?;

        let saved_pos = cursor.position();
        cursor.seek(coverage_offset as usize + base)?;
        let coverage = Coverage::parse(cursor)?;
        cursor.seek(saved_pos)?;

        let mut subrule_set_offsets = Vec::with_capacity(subrule_set_count as usize);
        for _ in 0..subrule_set_count {
            subrule_set_offsets.push(cursor.read_u16()?);
        }

        let mut subrule_sets = Vec::with_capacity(subrule_set_count as usize);
        for offset in subrule_set_offsets {
            let saved = cursor.position();
            cursor.seek(offset as usize + base)?;
            subrule_sets.push(SubruleSet::parse(cursor, base)?);
            cursor.seek(saved)?;
        }

        Ok(Self::Format1 {
            coverage,
            subrule_sets,
        })
    }

    fn parse_format2(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let coverage_offset = cursor.read_u16()?;
        let class_def_offset = cursor.read_u16()?;
        let subclass_set_count = cursor.read_u16()?;

        let saved_pos = cursor.position();
        cursor.seek(coverage_offset as usize + base)?;
        let coverage = Coverage::parse(cursor)?;
        cursor.seek(saved_pos)?;

        let saved_pos = cursor.position();
        cursor.seek(class_def_offset as usize + base)?;
        // Simplified class_def parsing - in real implementation, would parse ClassDef structure
        let mut class_def = Vec::new();
        // TODO: Parse full ClassDef structure
        cursor.seek(saved_pos)?;

        let mut subclass_set_offsets = Vec::with_capacity(subclass_set_count as usize);
        for _ in 0..subclass_set_count {
            subclass_set_offsets.push(cursor.read_u16()?);
        }

        let mut subclass_sets = Vec::with_capacity(subclass_set_count as usize);
        for offset in subclass_set_offsets {
            let saved = cursor.position();
            cursor.seek(offset as usize + base)?;
            subclass_sets.push(SubclassSet::parse(cursor, base)?);
            cursor.seek(saved)?;
        }

        Ok(Self::Format2 {
            coverage,
            class_def,
            subclass_sets,
        })
    }

    fn parse_format3(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let glyph_count = cursor.read_u16()?;
        let lookup_count = cursor.read_u16()?;

        let mut coverage_offsets = Vec::with_capacity(glyph_count as usize);
        for _ in 0..glyph_count {
            coverage_offsets.push(cursor.read_u16()?);
        }

        let mut lookup_records = Vec::with_capacity(lookup_count as usize);
        for _ in 0..lookup_count {
            lookup_records.push(LookupRecord::parse(cursor)?);
        }

        let mut coverages = Vec::with_capacity(glyph_count as usize);
        for offset in coverage_offsets {
            let saved = cursor.position();
            cursor.seek(offset as usize + base)?;
            coverages.push(Coverage::parse(cursor)?);
            cursor.seek(saved)?;
        }

        Ok(Self::Format3 {
            glyph_count,
            lookup_count,
            coverages,
            lookup_records,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SubruleSet {
    pub subrules: Vec<Subrule>,
}

impl SubruleSet {
    pub fn parse(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let subrule_count = cursor.read_u16()?;
        let mut subrule_offsets = Vec::with_capacity(subrule_count as usize);
        for _ in 0..subrule_count {
            subrule_offsets.push(cursor.read_u16()?);
        }

        let mut subrules = Vec::with_capacity(subrule_count as usize);
        for offset in subrule_offsets {
            let saved = cursor.position();
            cursor.seek(offset as usize + base)?;
            subrules.push(Subrule::parse(cursor)?);
            cursor.seek(saved)?;
        }

        Ok(Self { subrules })
    }
}

#[derive(Debug, Clone)]
pub struct Subrule {
    pub input_glyph_count: u16,
    pub input_glyphs: Vec<u16>,
    pub lookup_records: Vec<LookupRecord>,
}

impl Subrule {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let input_glyph_count = cursor.read_u16()?;
        let lookup_record_count = cursor.read_u16()?;

        let mut input_glyphs = Vec::with_capacity((input_glyph_count - 1) as usize);
        for _ in 1..input_glyph_count {
            input_glyphs.push(cursor.read_u16()?);
        }

        let mut lookup_records = Vec::with_capacity(lookup_record_count as usize);
        for _ in 0..lookup_record_count {
            lookup_records.push(LookupRecord::parse(cursor)?);
        }

        Ok(Self {
            input_glyph_count,
            input_glyphs,
            lookup_records,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SubclassSet {
    pub subclass_rules: Vec<SubclassRule>,
}

impl SubclassSet {
    pub fn parse(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let subclass_rule_count = cursor.read_u16()?;
        let mut subclass_rule_offsets = Vec::with_capacity(subclass_rule_count as usize);
        for _ in 0..subclass_rule_count {
            subclass_rule_offsets.push(cursor.read_u16()?);
        }

        let mut subclass_rules = Vec::with_capacity(subclass_rule_count as usize);
        for offset in subclass_rule_offsets {
            let saved = cursor.position();
            cursor.seek(offset as usize + base)?;
            subclass_rules.push(SubclassRule::parse(cursor)?);
            cursor.seek(saved)?;
        }

        Ok(Self { subclass_rules })
    }
}

#[derive(Debug, Clone)]
pub struct SubclassRule {
    pub class_count: u16,
    pub classes: Vec<u16>,
    pub lookup_records: Vec<LookupRecord>,
}

impl SubclassRule {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let class_count = cursor.read_u16()?;
        let lookup_record_count = cursor.read_u16()?;

        let mut classes = Vec::with_capacity((class_count - 1) as usize);
        for _ in 1..class_count {
            classes.push(cursor.read_u16()?);
        }

        let mut lookup_records = Vec::with_capacity(lookup_record_count as usize);
        for _ in 0..lookup_record_count {
            lookup_records.push(LookupRecord::parse(cursor)?);
        }

        Ok(Self {
            class_count,
            classes,
            lookup_records,
        })
    }
}

/// Chained Contextual Substitution - applies substitutions based on both backtrack and lookahead context
#[derive(Debug, Clone)]
pub enum ChainedContextualSubstitution {
    /// Format 1: Simple chaining context substitution
    Format1 {
        coverage: Coverage,
        chain_subrule_sets: Vec<ChainSubruleSet>,
    },
    /// Format 2: Class-based chaining context substitution
    Format2 {
        coverage: Coverage,
        backtrack_class: ClassDef,
        input_class: ClassDef,
        lookahead_class: ClassDef,
        chain_subclass_sets: Vec<ChainSubclassSet>,
    },
    /// Format 3: Coverage table-based chaining context substitution
    Format3 {
        backtrack_glyph_count: u16,
        input_glyph_count: u16,
        lookahead_glyph_count: u16,
        backtrack_coverages: Vec<Coverage>,
        input_coverages: Vec<Coverage>,
        lookahead_coverages: Vec<Coverage>,
        seq_lookup_records: Vec<LookupRecord>,
    },
}

impl ChainedContextualSubstitution {
    pub fn parse(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let format = cursor.read_u16()?;
        println!("Format was : {}", format);
        match format {
            1 => Self::parse_format1(cursor, base),
            2 => Self::parse_format2(cursor, base),
            3 => Self::parse_format3(cursor, base),
            _ => {
                println!("Format that was given: {}", format);
                return Err(Error::AnyMessage(
                    "Unrecognized chained contextual substitution",
                ));
            }
        }
    }

    fn parse_format1(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let coverage_offset = cursor.read_u16()?;
        let chain_subrule_set_count = cursor.read_u16()?;

        let saved_pos = cursor.position();
        cursor.seek(coverage_offset as usize + base)?;
        let coverage = Coverage::parse(cursor)?;
        cursor.seek(saved_pos)?;

        let mut chain_subrule_set_offsets = Vec::with_capacity(chain_subrule_set_count as usize);
        for _ in 0..chain_subrule_set_count {
            chain_subrule_set_offsets.push(cursor.read_u16()?);
        }

        let mut chain_subrule_sets = Vec::with_capacity(chain_subrule_set_count as usize);
        for offset in chain_subrule_set_offsets {
            let saved = cursor.position();
            cursor.seek(offset as usize + base)?;
            chain_subrule_sets.push(ChainSubruleSet::parse(cursor, base)?);
            cursor.seek(saved)?;
        }

        Ok(Self::Format1 {
            coverage,
            chain_subrule_sets,
        })
    }

    fn parse_format2(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let coverage_offset = cursor.read_u16()?;
        let backtrack_offset = cursor.read_u16()?;
        let input_offset = cursor.read_u16()?;
        let lookahead_offset = cursor.read_u16()?;
        let ruleset_offset_count = cursor.read_u16()?;
        let mut ruleset_offsets = Vec::with_capacity(ruleset_offset_count as usize);
        for _ in 0..ruleset_offset_count {
            ruleset_offsets.push(cursor.read_u16()?);
        }
        cursor.seek(coverage_offset as usize + base)?;
        let coverage = Coverage::parse(cursor)?;
        cursor.seek(backtrack_offset as usize + base)?;
        let backtrack = ClassDef::parse(cursor)?;
        cursor.seek(input_offset as usize + base)?;
        let input = ClassDef::parse(cursor)?;
        cursor.seek(lookahead_offset as usize + base)?;
        let lookahead = ClassDef::parse(cursor)?;
        let mut rulesets = Vec::with_capacity(ruleset_offset_count as usize);
        for offset in ruleset_offsets {
            cursor.seek(base + offset as usize)?;
            let chained_class_rule_offset_count = cursor.read_u16()?;
            let mut offsets = Vec::with_capacity(chained_class_rule_offset_count as usize);
            let saved = cursor.position();
            for _ in 0..chained_class_rule_offset_count {
                offsets.push(cursor.read_u16()?);
            }
            for offset in offsets {
                cursor.seek(saved + offset as usize)?;
                ChainedClassSeqRule::p
            }
        }
        Ok(Self::Format2 {
            coverage,
            class_def,
            chain_subclass_sets,
        })
    }

    fn parse_format3(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let backtrack_glyph_count = cursor.read_u16()?;

        let mut backtrack_coverage_offsets = Vec::with_capacity(backtrack_glyph_count as usize);
        for _ in 0..backtrack_glyph_count {
            backtrack_coverage_offsets.push(cursor.read_u16()?);
        }

        let input_glyph_count = cursor.read_u16()?;
        let mut input_coverage_offsets = Vec::with_capacity(input_glyph_count as usize);
        for _ in 0..input_glyph_count {
            input_coverage_offsets.push(cursor.read_u16()?);
        }

        let lookahead_glyph_count = cursor.read_u16()?;
        let mut lookahead_coverage_offsets = Vec::with_capacity(lookahead_glyph_count as usize);
        for _ in 0..lookahead_glyph_count {
            lookahead_coverage_offsets.push(cursor.read_u16()?);
        }

        let lookup_record_count = cursor.read_u16()?;
        let mut seq_lookup_records = Vec::with_capacity(lookup_record_count as usize);
        for _ in 0..lookup_record_count {
            seq_lookup_records.push(LookupRecord::parse(cursor)?);
        }

        // Parse coverage tables
        let mut backtrack_coverages = Vec::with_capacity(backtrack_glyph_count as usize);
        for offset in backtrack_coverage_offsets {
            let saved = cursor.position();
            cursor.seek(offset as usize + base)?;
            backtrack_coverages.push(Coverage::parse(cursor)?);
            cursor.seek(saved)?;
        }

        let mut input_coverages = Vec::with_capacity(input_glyph_count as usize);
        for offset in input_coverage_offsets {
            let saved = cursor.position();
            cursor.seek(offset as usize + base)?;
            input_coverages.push(Coverage::parse(cursor)?);
            cursor.seek(saved)?;
        }

        let mut lookahead_coverages = Vec::with_capacity(lookahead_glyph_count as usize);
        for offset in lookahead_coverage_offsets {
            let saved = cursor.position();
            cursor.seek(offset as usize + base)?;
            lookahead_coverages.push(Coverage::parse(cursor)?);
            cursor.seek(saved)?;
        }

        Ok(Self::Format3 {
            backtrack_glyph_count,
            input_glyph_count,
            lookahead_glyph_count,
            backtrack_coverages,
            input_coverages,
            lookahead_coverages,
            seq_lookup_records,
        })
    }
}
#[derive(Debug, Clone)]
pub struct ChainedClassSeqRule {
    backtrack_sequence: Vec<u16>,
    input_glyph: Vec<u16>,
    lookahead_sequence: Vec<u16>,
    seq_lookup_records: Vec<SequenceLookup>,
}
impl ChainedClassSeqRule {
    fn parse(cursor: &mut Cursor) -> Result<Self, Error> {}
}

#[derive(Debug, Clone)]
pub struct SequenceLookup {
    sequence_index: u16,
    lookup_list_index: u16,
}

#[derive(Debug, Clone)]
pub struct ChainSubruleSet {
    pub chained_cl: Vec<ChainSubrule>,
}

impl ChainSubruleSet {
    pub fn parse(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let chain_subrule_count = cursor.read_u16()?;
        let mut chain_subrule_offsets = Vec::with_capacity(chain_subrule_count as usize);
        for _ in 0..chain_subrule_count {
            chain_subrule_offsets.push(cursor.read_u16()?);
        }

        let mut chain_subrules = Vec::with_capacity(chain_subrule_count as usize);
        for offset in chain_subrule_offsets {
            let saved = cursor.position();
            cursor.seek(offset as usize + base)?;
            chain_subrules.push(ChainSubrule::parse(cursor)?);
            cursor.seek(saved)?;
        }

        Ok(Self { chain_subrules })
    }
}

#[derive(Debug, Clone)]
pub struct ChainSubrule {
    pub backtrack_glyph_count: u16,
    pub backtrack_glyphs: Vec<u16>,
    pub input_glyph_count: u16,
    pub input_glyphs: Vec<u16>,
    pub lookahead_glyph_count: u16,
    pub lookahead_glyphs: Vec<u16>,
    pub lookup_records: Vec<LookupRecord>,
}

impl ChainSubrule {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let backtrack_glyph_count = cursor.read_u16()?;
        let mut backtrack_glyphs = Vec::with_capacity(backtrack_glyph_count as usize);
        for _ in 0..backtrack_glyph_count {
            backtrack_glyphs.push(cursor.read_u16()?);
        }

        let input_glyph_count = cursor.read_u16()?;
        let mut input_glyphs = Vec::with_capacity((input_glyph_count - 1) as usize);
        for _ in 1..input_glyph_count {
            input_glyphs.push(cursor.read_u16()?);
        }

        let lookahead_glyph_count = cursor.read_u16()?;
        let mut lookahead_glyphs = Vec::with_capacity(lookahead_glyph_count as usize);
        for _ in 0..lookahead_glyph_count {
            lookahead_glyphs.push(cursor.read_u16()?);
        }

        let lookup_record_count = cursor.read_u16()?;
        let mut lookup_records = Vec::with_capacity(lookup_record_count as usize);
        for _ in 0..lookup_record_count {
            lookup_records.push(LookupRecord::parse(cursor)?);
        }

        Ok(Self {
            backtrack_glyph_count,
            backtrack_glyphs,
            input_glyph_count,
            input_glyphs,
            lookahead_glyph_count,
            lookahead_glyphs,
            lookup_records,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ChainSubclassSet {
    pub chain_subclass_rules: Vec<ChainSubclassRule>,
}

impl ChainSubclassSet {
    pub fn parse(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let chain_subclass_rule_count = cursor.read_u16()?;
        let mut chain_subclass_rule_offsets =
            Vec::with_capacity(chain_subclass_rule_count as usize);
        for _ in 0..chain_subclass_rule_count {
            chain_subclass_rule_offsets.push(cursor.read_u16()?);
        }

        let mut chain_subclass_rules = Vec::with_capacity(chain_subclass_rule_count as usize);
        for offset in chain_subclass_rule_offsets {
            let saved = cursor.position();
            cursor.seek(offset as usize + base)?;
            chain_subclass_rules.push(ChainSubclassRule::parse(cursor)?);
            cursor.seek(saved)?;
        }

        Ok(Self {
            chain_subclass_rules,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ChainSubclassRule {
    pub backtrack_class_count: u16,
    pub backtrack_classes: Vec<u16>,
    pub input_class_count: u16,
    pub input_classes: Vec<u16>,
    pub lookahead_class_count: u16,
    pub lookahead_classes: Vec<u16>,
    pub lookup_records: Vec<LookupRecord>,
}

impl ChainSubclassRule {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let backtrack_class_count = cursor.read_u16()?;
        let mut backtrack_classes = Vec::with_capacity(backtrack_class_count as usize);
        for _ in 0..backtrack_class_count {
            backtrack_classes.push(cursor.read_u16()?);
        }

        let input_class_count = cursor.read_u16()?;
        let mut input_classes = Vec::with_capacity((input_class_count - 1) as usize);
        for _ in 1..input_class_count {
            input_classes.push(cursor.read_u16()?);
        }

        let lookahead_class_count = cursor.read_u16()?;
        let mut lookahead_classes = Vec::with_capacity(lookahead_class_count as usize);
        for _ in 0..lookahead_class_count {
            lookahead_classes.push(cursor.read_u16()?);
        }

        let lookup_record_count = cursor.read_u16()?;
        let mut lookup_records = Vec::with_capacity(lookup_record_count as usize);
        for _ in 0..lookup_record_count {
            lookup_records.push(LookupRecord::parse(cursor)?);
        }

        Ok(Self {
            backtrack_class_count,
            backtrack_classes,
            input_class_count,
            input_classes,
            lookahead_class_count,
            lookahead_classes,
            lookup_records,
        })
    }
}

/// Extension Substitution - allows other substitution lookups to be used in extension format
#[derive(Debug, Clone)]
pub struct ExtensionSubstitution {
    pub extension_lookup_type: u16,
    pub extension_substitution: Box<Substitution>,
}

impl ExtensionSubstitution {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let base = cursor.position();
        let extension_lookup_type = cursor.read_u16()?;
        let extension_offset = cursor.read_u32()?;

        let saved_pos = cursor.position();
        cursor.seek(extension_offset as usize + base)?;
        let extension_substitution =
            Box::new(Substitution::parse(cursor, cursor.position() as u16)?);
        cursor.seek(saved_pos)?;

        Ok(Self {
            extension_lookup_type,
            extension_substitution,
        })
    }
}

/// Reverse Substitution - applies reverse chaining contextual substitution
#[derive(Debug, Clone)]
pub struct ReverseSubstitution {
    pub format: u16,
    pub coverage: Coverage,
    pub backtrack_glyph_count: u16,
    pub backtrack_coverages: Vec<Coverage>,
    pub lookahead_glyph_count: u16,
    pub lookahead_coverages: Vec<Coverage>,
    pub substitute_count: u16,
    pub substitute_glyph_ids: Vec<u16>,
}

impl ReverseSubstitution {
    pub fn parse(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let format = cursor.read_u16()?;
        if format != 1 {
            return Err(Error::AnyMessage("Unrecognized reverse substitution"));
        }

        let coverage_offset = cursor.read_u16()?;
        let saved_pos = cursor.position();
        cursor.seek(coverage_offset as usize + base)?;
        let coverage = Coverage::parse(cursor)?;
        cursor.seek(saved_pos)?;

        let backtrack_glyph_count = cursor.read_u16()?;
        let mut backtrack_coverage_offsets = Vec::with_capacity(backtrack_glyph_count as usize);
        for _ in 0..backtrack_glyph_count {
            backtrack_coverage_offsets.push(cursor.read_u16()?);
        }

        let lookahead_glyph_count = cursor.read_u16()?;
        let mut lookahead_coverage_offsets = Vec::with_capacity(lookahead_glyph_count as usize);
        for _ in 0..lookahead_glyph_count {
            lookahead_coverage_offsets.push(cursor.read_u16()?);
        }

        let substitute_count = cursor.read_u16()?;
        let mut substitute_glyph_ids = Vec::with_capacity(substitute_count as usize);
        for _ in 0..substitute_count {
            substitute_glyph_ids.push(cursor.read_u16()?);
        }

        let mut backtrack_coverages = Vec::with_capacity(backtrack_glyph_count as usize);
        for offset in backtrack_coverage_offsets {
            let saved = cursor.position();
            cursor.seek(offset as usize + base)?;
            backtrack_coverages.push(Coverage::parse(cursor)?);
            cursor.seek(saved)?;
        }

        let mut lookahead_coverages = Vec::with_capacity(lookahead_glyph_count as usize);
        for offset in lookahead_coverage_offsets {
            let saved = cursor.position();
            cursor.seek(offset as usize + base)?;
            lookahead_coverages.push(Coverage::parse(cursor)?);
            cursor.seek(saved)?;
        }

        Ok(Self {
            format,
            coverage,
            backtrack_glyph_count,
            backtrack_coverages,
            lookahead_glyph_count,
            lookahead_coverages,
            substitute_count,
            substitute_glyph_ids,
        })
    }
}

/// Shared structure for lookup records used in contextual rules
#[derive(Debug, Clone)]
pub struct LookupRecord {
    pub sequence_index: u16,
    pub lookup_list_index: u16,
}

impl LookupRecord {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let sequence_index = cursor.read_u16()?;
        let lookup_list_index = cursor.read_u16()?;
        Ok(Self {
            sequence_index,
            lookup_list_index,
        })
    }
}
