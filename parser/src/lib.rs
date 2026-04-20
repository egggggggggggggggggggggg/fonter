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
- Better cursor?
- Abstract away the tedious stuff. (Don't do offsets though for lazy parsing).
*/
