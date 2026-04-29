pub mod aat;
mod cursor;
pub mod error;
mod table;
pub mod tags;
pub use aat::*;
pub mod ttf_parse;
use math::shape::Shape;
pub use ttf_parse::*;

use crate::{error::Error, table::gsub::Gsub};
/*
* TO DO:
- Implement lazy parsing
- Heapless parsing?
- Minimize allocations when possible
- Abstract away the tedious stuff. (Don't do offsets though for lazy parsing).
- Bench against ttf_parser(mimic the same benches that ttf_parser has).
- Optimize based off benches
- Work on Text-Shaping engine. Read up on how to consume characters to be replaced with new ones.
- Avoid passing around Cursor, but instead give slices to the struct being parsed. Avoids data being cloned + makes it easier to reason about.
- Outputting pos + etc,
- Rest of the tables like GPOS, ...,
*/

pub trait Font: Sized {
    fn glyph_index(&self, codepoint: u32) -> u16;
    fn gsub(&self) -> Option<Gsub<'static>>;
    fn gpos(&self) -> Option<()>;
    fn metrics(&self, glyph_id: u16) -> f32;
}
///Static dispatching fonts or smth, idk.
pub enum FontTypes {
    Ttf(TtfFont),
    Otf,
    Woff,
}
impl Font for FontTypes {
    fn glyph_index(&self, codepoint: u32) -> u16 {
        todo!()
    }
    fn gsub(&self) -> Option<Gsub<'static>> {
        todo!()
    }
    fn gpos(&self) -> Option<()> {
        todo!()
    }
    fn metrics(&self, glyph_id: u16) -> f32 {
        todo!()
    }
}
///The data lasts as long as the file is alive.
pub trait Table<'a>: Sized {
    fn parse(data: &'a [u8]) -> Result<Self, Error>;
}
