use std::time::Instant;

use atlas_gen::entry;
///NOTES: Flamegraph seems really inconsistent with the areas it points out. Perf might be better
///suited.
fn main() {
    entry();
}
