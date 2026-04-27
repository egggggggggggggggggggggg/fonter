use crate::{TableRecord, cursor::Cursor, error::Error};
use std::collections::HashMap;

#[derive(Debug)]
enum Kern {
    Apple {
        version: u32,
        subtables: Vec<AppleSubtable>,
    },
    Microsoft {
        version: u16,
        subtables: Vec<MsSubtable>,
    },
}

#[derive(Debug)]
pub struct KernPair {
    pub left: u16,
    pub right: u16,
    pub value: i16,
}

#[derive(Debug)]
struct MsSubtable {
    pub coverage: u16,
    pub pairs: Vec<KernPair>,
}

#[derive(Debug)]
struct AppleSubtable {
    pub coverage: u16,
}

impl Kern {
    pub fn parse(data: &[u8], tables: &HashMap<[u8; 4], TableRecord>) -> Result<Self, Error> {
        let rec = tables.get(b"kern").ok_or(Error::MissingTable("kern"))?;
        let mut cursor = Cursor::set(data, rec.table_offset);
        let start = cursor.position();
        let version32 = cursor.read_u32()?;
        cursor.seek(start)?;
        if version32 == 0x00010000 {
            Self::parse_apple(cursor)
        } else {
            Self::parse_ms(cursor)
        }
    }
    fn search() {}
    fn parse_ms(mut cursor: Cursor) -> Result<Self, Error> {
        let version = cursor.read_u16()?; // usually 0
        let n_tables = cursor.read_u16()?;
        let mut subtables = Vec::with_capacity(n_tables as usize);
        for _ in 0..n_tables {
            let _sub_version = cursor.read_u16()?;
            let length = cursor.read_u16()?;
            let coverage = cursor.read_u16()?;
            let format = coverage & 0x00FF;
            let subtable_start = cursor.position();
            match format {
                0 => {
                    let n_pairs = cursor.read_u16()?;
                    let _search_range = cursor.read_u16()?;
                    let _entry_selector = cursor.read_u16()?;
                    let _range_shift = cursor.read_u16()?;
                    let mut pairs = Vec::with_capacity(n_pairs as usize);
                    for _ in 0..n_pairs {
                        pairs.push(KernPair {
                            left: cursor.read_u16()?,
                            right: cursor.read_u16()?,
                            value: cursor.read_i16()?,
                        });
                    }
                    subtables.push(MsSubtable { coverage, pairs });
                }
                2 => {}
                _ => {}
            }
            cursor.seek(subtable_start + (length as usize - 6))?;
        }
        Ok(Kern::Microsoft { version, subtables })
    }
    fn parse_apple(mut cursor: Cursor) -> Result<Self, Error> {
        let version = cursor.read_u32()?; // 0x00010000
        let n_tables = cursor.read_u32()?;
        let mut subtables = Vec::with_capacity(n_tables as usize);
        for _ in 0..n_tables {
            let length = cursor.read_u32()? as usize;
            let coverage = cursor.read_u16()?;
            let _tuple_index = cursor.read_u16()?;
            let subtable_start = cursor.position();
            subtables.push(AppleSubtable { coverage });
            cursor.seek(subtable_start + (length - 8))?;
        }
        Ok(Kern::Apple { version, subtables })
    }
}
