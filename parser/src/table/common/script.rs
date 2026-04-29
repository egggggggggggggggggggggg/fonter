use std::collections::{HashMap, hash_map::Entry};

use crate::{
    common::lang_sys::{LangSys, LangSysRecord},
    cursor::Cursor,
    error::Error,
    tags::{self, Tag},
};

pub struct ScriptList {
    script_records: HashMap<Tag, ScriptRecords>,
    parsed_scripts: HashMap<Tag, Script>,
    base: usize,
}

impl ScriptList {
    /// Prior to calling this, cursor must be aligned to the offset specified for the table.
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let base = cursor.position();
        println!("Script List base was : {}", base);
        let script_count = cursor.read_u16()?;
        let mut script_records = HashMap::with_capacity(script_count as usize);
        for _ in 0..script_count {
            let record = ScriptRecords::parse(cursor)?;
            let tag = tags::Tag::from(record.script_tag);
            println!("Parsed tags: {}", tag);
            script_records.insert(tag, record);
        }
        Ok(Self {
            script_records,
            parsed_scripts: HashMap::new(),
            base,
        })
    }

    /// Parse every script immediately.
    pub fn parse_all(&mut self, cursor: &mut Cursor) -> Result<(), Error> {
        let kv: Vec<(tags::Tag, ScriptRecords)> = self.script_records.drain().collect();
        for (tag, record) in kv {
            cursor.seek(self.base + record.script_offset as usize)?;
            let script = Script::parse(cursor)?;
            self.parsed_scripts.insert(tag, script);
        }
        Ok(())
    }

    /// Ensure the script for `tag` is parsed and return a reference to it.
    /// This parses the script lazily if necessary, so callers don't need to call a separate parse method.

    pub fn get_or_parse(&mut self, cursor: &mut Cursor, tag: &Tag) -> Result<&Script, Error> {
        match self.parsed_scripts.entry(*tag) {
            Entry::Occupied(o) => return Ok(o.into_mut()),
            Entry::Vacant(v) => {
                // Need to remove the ScriptRecords from script_records first.
                let record = self.script_records.remove(tag).ok_or(Error::AnyMessage(
                    "Record does not exist in the script_records",
                ))?;
                println!(
                    "attempting to seek to here: {} when data was only: {} ",
                    self.base + record.script_offset as usize,
                    cursor.size()
                );
                cursor.seek(self.base + record.script_offset as usize)?;
                println!("Attempting to parse a script");
                let script = Script::parse(cursor)?;
                // Insert and return a reference to the inserted value.
                return Ok(v.insert(script));
            }
        }
    }
    pub fn get_parsed(&self, tag: &Tag) -> Option<&Script> {
        self.parsed_scripts.get(tag)
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
    pub lang_sys: HashMap<Tag, LangSys>,
}

impl Script {
    pub fn parse(cursor: &mut Cursor) -> Result<Self, Error> {
        let base = cursor.position();
        let default_lang_sys_offset = cursor.read_u16()?;
        let lang_sys_count = cursor.read_u16()?;
        let mut lang_sys = HashMap::with_capacity(lang_sys_count as usize);
        for _ in 0..lang_sys_count {
            let lang_sys_record = LangSysRecord::parse(cursor, base)?;
            lang_sys.insert(
                Tag::from(lang_sys_record.lang_sys_tag),
                lang_sys_record.lang_sys,
            );
        }
        let default_lang_sys = if default_lang_sys_offset == 0 {
            None
        } else {
            cursor.seek(base + default_lang_sys_offset as usize)?;
            let result = Some(LangSys::parse(cursor)?);
            result
        };
        Ok(Self {
            default_lang_sys,
            lang_sys,
        })
    }
}
