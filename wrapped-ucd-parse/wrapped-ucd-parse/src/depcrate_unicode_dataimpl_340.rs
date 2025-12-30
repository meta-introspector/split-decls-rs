// Generated macro for impl_340 (impl)
macro_rules! Depcrate_unicode_dataimpl_340 {
() => {
// Module: crate::unicode_data
// Provides: {"impl_340"}
// Dependencies: {}
impl < I : Iterator < Item = UnicodeData > > Iterator for UnicodeDataExpander < I > { type Item = UnicodeData ; fn next (& mut self) -> Option < UnicodeData > { if let Some (udata) = self . range . next () { return Some (udata) ; } let row1 = match self . it . next () { None => return None , Some (row1) => row1 , } ; if ! row1 . is_range_start () || ! self . it . peek () . map_or (false , | row2 | row2 . is_range_end ()) { return Some (row1) ; } let row2 = self . it . next () . unwrap () ; self . range = CodepointRange { range : row1 . codepoint . value () .. (row2 . codepoint . value () + 1) , start_record : row1 , } ; self . next () } }
};
}
