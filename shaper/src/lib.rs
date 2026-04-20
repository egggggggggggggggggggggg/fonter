pub struct SequenceLookup {
    sequence_index: u16,
    lookup_list_index: u16,
}
pub enum SequenceContext {
    Format1 {},
    Format2 {},
    Format3 {
        glyph_count: u16,
        seq_lookup_count: u16,
        coverage_offsets: Vec<u16>,
        seq_lookup_records: Vec<SequenceLookup>,
    },
}
