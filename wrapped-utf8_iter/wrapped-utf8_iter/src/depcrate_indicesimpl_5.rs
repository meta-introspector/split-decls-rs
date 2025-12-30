// Generated macro for impl_5 (impl)
macro_rules! Depcrate_indicesimpl_5 {
() => {
// Module: crate::indices
// Provides: {"impl_5"}
// Dependencies: {}
impl < 'a > Iterator for Utf8CharIndices < 'a > { type Item = (usize , char) ; # [inline] fn next (& mut self) -> Option < (usize , char) > { let pre_len = self . as_slice () . len () ; match self . iter . next () { None => None , Some (ch) => { let index = self . front_offset ; let len = self . as_slice () . len () ; self . front_offset += pre_len - len ; Some ((index , ch)) } } } # [inline] fn count (self) -> usize { self . iter . count () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } # [inline] fn last (mut self) -> Option < (usize , char) > { self . next_back () } }
};
}
