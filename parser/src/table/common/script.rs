use std::collections::HashMap;

use crate::{
    common::lang_sys::{LangSys, LangSysRecord},
    cursor::Cursor,
    error::Error,
};
///SriptList parses on demand.
pub struct ScriptList {
    script_records: HashMap<[u8; 4], ScriptRecords>,
    ///When the Script is parsed, its cached into a hashMap to avoid reparsing.
    parsed_scripts: HashMap<[u8; 4], Script>,
    ///Where the ScriptList starts. This allows us to lazily parse by allowing us to resume our
    ///previous position when needed.  
    base: usize,
}
impl ScriptList {
    ///Prior to calling this, cursor must be alligned to the offset specified for the table.
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let base = cursor.position();
        let script_count = cursor.read_u16()?;
        let mut script_records = HashMap::with_capacity(script_count as usize);
        for _ in 0..script_count {
            let record = ScriptRecords::parse(cursor)?;
            script_records.insert(record.script_tag, record);
        }
        Ok(Self {
            parsed_scripts: HashMap::new(),
            script_records,
            base,
        })
    }
    pub fn parse_all(&mut self, cursor: &mut Cursor) -> Result<(), Error> {
        let kvpair: Vec<([u8; 4], ScriptRecords)> = self.script_records.drain().collect();
        for (tag, record) in kvpair {
            cursor.seek(self.base + record.script_offset as usize)?;
            let res = Script::parse(cursor)?;
            self.parsed_scripts.insert(tag, res);
        }
        Ok(())
    }
    ///Parses the specified tag.
    pub fn parse_script(&mut self, cursor: &mut Cursor, tag: &[u8; 4]) -> Result<(), Error> {
        let record = self.script_records.remove(tag).ok_or(Error::Unknown)?;
        cursor.seek(self.base + record.script_offset as usize)?;
        let res = Script::parse(cursor)?;
        self.parsed_scripts.insert(*tag, res.clone());
        Ok(())
    }
    ///Returns a script if it has been parsed. If not call parse_script and then get_script.
    pub fn get_script(&mut self, tag: &[u8; 4]) -> Result<&Script, Error> {
        self.parsed_scripts.get(tag).ok_or(Error::Unknown)
    }
}

pub struct ScriptRecords {
    script_tag: [u8; 4],
    script_offset: u16,
}
impl ScriptRecords {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let script_tag = cursor.read_u32()?.to_be_bytes();
        let script_offset = cursor.read_u16()?;
        Ok(Self {
            script_tag,
            script_offset,
        })
    }
}
#[derive(Clone, Debug)]
pub struct Script {
    pub default_lang_sys: Option<LangSys>,
    pub lang_sys: HashMap<[u8; 4], LangSys>,
}

impl Script {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let base = cursor.position();
        let default_lang_sys_offset = cursor.read_u16()?;
        let lang_sys_count = cursor.read_u16()?;
        let mut lang_sys = HashMap::with_capacity(lang_sys_count as usize);
        for _ in 0..lang_sys_count {
            let lang_sys_record = LangSysRecord::parse(cursor, base)?;
            lang_sys.insert(lang_sys_record.lang_sys_tag, lang_sys_record.lang_sys);
        }
        let default_lang_sys = if default_lang_sys_offset == 0 {
            None
        } else {
            cursor.seek(default_lang_sys_offset as usize)?;
            Some(LangSys::parse(cursor)?)
        };
        Ok(Self {
            default_lang_sys,
            lang_sys,
        })
    }
}
