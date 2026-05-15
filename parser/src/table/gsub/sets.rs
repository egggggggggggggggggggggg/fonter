use crate::{common::sequence::SequenceLookup, cursor::Cursor, error::Error};

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
#[derive(Debug, Clone)]
pub struct ChainedClassSeqRule {
    pub backtrack_sequences: Vec<u16>,
    pub input_sequences: Vec<u16>,
    pub lookahead_sequences: Vec<u16>,
    pub seq_lookup_records: Vec<SequenceLookup>,
}
impl ChainedClassSeqRule {
    pub fn parse(cursor: &mut Cursor, _base: usize) -> Result<Self, Error> {
        let backtrack_glyph_count = cursor.read_u16()?;
        let mut backtrack_sequences = Vec::with_capacity(backtrack_glyph_count as usize);
        for _ in 0..backtrack_glyph_count {
            backtrack_sequences.push(cursor.read_u16()?);
        }
        let input_glyph_count = cursor.read_u16()?;
        let mut input_sequences = Vec::with_capacity(input_glyph_count as usize - 1);
        for _ in 0..input_glyph_count - 1 {
            input_sequences.push(cursor.read_u16()?);
        }
        let lookahead_glyph_count = cursor.read_u16()?;
        let mut lookahead_sequences = Vec::with_capacity(lookahead_glyph_count as usize);
        for _ in 0..lookahead_glyph_count {
            lookahead_sequences.push(cursor.read_u16()?);
        }
        let seq_lookup_count = cursor.read_u16()?;
        let mut seq_lookup_records = Vec::with_capacity(seq_lookup_count as usize);
        for _ in 0..seq_lookup_count {
            seq_lookup_records.push(SequenceLookup::parse(cursor)?);
        }
        Ok(Self {
            backtrack_sequences,
            input_sequences,
            lookahead_sequences,
            seq_lookup_records,
        })
    }
}
#[derive(Debug, Clone)]
pub struct ChainedClassSeqRuleSet {
    pub chained_class_seq_rules: Vec<ChainedClassSeqRule>,
}
impl ChainedClassSeqRuleSet {
    pub fn parse(cursor: &mut Cursor, base: usize) -> Result<Self, Error> {
        let start = cursor.position();
        let chained_class_seq_rule_count = cursor.read_u16()?;
        let mut chained_class_seq_rule_offsets =
            Vec::with_capacity(chained_class_seq_rule_count as usize);
        for _ in 0..chained_class_seq_rule_count {
            chained_class_seq_rule_offsets.push(cursor.read_u16()?);
        }
        let mut chained_class_seq_rules = Vec::with_capacity(chained_class_seq_rule_count as usize);
        for offset in chained_class_seq_rule_offsets {
            cursor.seek(start + offset as usize)?;
            chained_class_seq_rules.push(ChainedClassSeqRule::parse(cursor, base)?);
        }
        Ok(Self {
            chained_class_seq_rules,
        })
    }
}

pub struct ClassSequenceRuleSet {
    pub class_sequence_rules: Vec<ClassSequenceRule>,
}

pub struct ClassSequenceRule {
    pub glyph_count: u16,
    pub seq_lookup_count: u16,
    pub input_sequence: Vec<u16>,
    pub seq_lookup_records: Vec<SequenceLookup>,
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
pub struct ChainSubruleSet {
    pub chain_subrules: Vec<ChainSubrule>,
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
