// Generated macro for impl_411 (impl)
macro_rules! Depcrate_utils_linesimpl_411 {
() => {
// Module: crate::utils::lines
// Provides: {"impl_411"}
// Dependencies: {}
impl < 'a > Iterator for LinesWithTerminator < 'a > { type Item = & 'a str ; # [inline] fn next (& mut self) -> Option < & 'a str > { match self . data . find ('\n') { None if self . data . is_empty () => None , None => { let line = self . data ; self . data = "" ; Some (line) } Some (end) => { let line = & self . data [.. end + 1] ; self . data = & self . data [end + 1 ..] ; Some (line) } } } }
};
}
