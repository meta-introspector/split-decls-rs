// Generated macro for impl_17 (impl)
macro_rules! Depcrate_graphemeimpl_17 {
() => {
// Module: crate::grapheme
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'a > Iterator for Graphemes < 'a > { type Item = & 'a str ; # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let slen = self . cursor_back . cur_cursor () - self . cursor . cur_cursor () ; (cmp :: min (slen , 1) , Some (slen)) } # [inline] fn next (& mut self) -> Option < & 'a str > { let start = self . cursor . cur_cursor () ; if start == self . cursor_back . cur_cursor () { return None ; } let next = self . cursor . next_boundary (self . string , 0) . unwrap () . unwrap () ; Some (& self . string [start .. next]) } }
};
}
