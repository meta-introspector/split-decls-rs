// Generated macro for impl_958 (impl)
macro_rules! Depcrate_modules_visitorimpl_958 {
() => {
// Module: crate::modules::visitor
// Provides: {"impl_958"}
// Dependencies: {}
impl SnippetProvider { pub (crate) fn span_to_snippet (& self , span : Span) -> Option < & str > { let start_index = span . lo () . to_usize () . checked_sub (self . start_pos) ? ; let end_index = span . hi () . to_usize () . checked_sub (self . start_pos) ? ; Some (& self . big_snippet [start_index .. end_index]) } pub (crate) fn new (start_pos : BytePos , end_pos : BytePos , big_snippet : Arc < String >) -> Self { let start_pos = start_pos . to_usize () ; let end_pos = end_pos . to_usize () ; SnippetProvider { big_snippet , start_pos , end_pos , } } pub (crate) fn entire_snippet (& self) -> & str { self . big_snippet . as_str () } pub (crate) fn start_pos (& self) -> BytePos { BytePos :: from_usize (self . start_pos) } pub (crate) fn end_pos (& self) -> BytePos { BytePos :: from_usize (self . end_pos) } }
};
}
