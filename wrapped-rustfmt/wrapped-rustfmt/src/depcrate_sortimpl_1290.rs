// Generated macro for impl_1290 (impl)
macro_rules! Depcrate_sortimpl_1290 {
() => {
// Module: crate::sort
// Provides: {"impl_1290"}
// Dependencies: {}
impl < 'a > Iterator for VersionChunkIter < 'a > { type Item = VersionChunk < 'a > ; fn next (& mut self) -> Option < Self :: Item > { let mut chars = self . ident [self . start ..] . char_indices () ; let (_ , next) = chars . next () ? ; if next == '_' { self . start = self . start + next . len_utf8 () ; return Some (VersionChunk :: Underscore) ; } if next . is_ascii_digit () { return self . parse_numeric_chunk (chars) ; } self . parse_str_chunk (chars) } }
};
}
