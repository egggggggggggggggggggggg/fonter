mod cursor;
pub mod error;
mod table;
mod tags;
pub mod ttf_parse;
pub use ttf_parse::*;
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
