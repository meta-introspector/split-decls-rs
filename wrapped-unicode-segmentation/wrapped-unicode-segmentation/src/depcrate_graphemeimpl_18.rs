// Generated macro for impl_18 (impl)
macro_rules! Depcrate_graphemeimpl_18 {
() => {
// Module: crate::grapheme
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for Graphemes < 'a > { # [inline] fn next_back (& mut self) -> Option < & 'a str > { let end = self . cursor_back . cur_cursor () ; if end == self . cursor . cur_cursor () { return None ; } let prev = self . cursor_back . prev_boundary (self . string , 0) . unwrap () . unwrap () ; Some (& self . string [prev .. end]) } }
};
}
