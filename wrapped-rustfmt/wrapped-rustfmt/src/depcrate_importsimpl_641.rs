// Generated macro for impl_641 (impl)
macro_rules! Depcrate_importsimpl_641 {
() => {
// Module: crate::imports
// Provides: {"impl_641"}
// Dependencies: {}
impl < 'a > FmtVisitor < 'a > { pub (crate) fn format_import (& mut self , item : & ast :: Item , tree : & ast :: UseTree) { let span = item . span () ; let shape = self . shape () ; let rw = UseTree :: from_ast (& self . get_context () , tree , None , Some (item . vis . clone ()) , Some (item . span . lo ()) , Some (item . attrs . clone ()) ,) . rewrite_top_level (& self . get_context () , shape) . ok () ; match rw { Some (ref s) if s . is_empty () => { let prev_span = mk_sp (self . last_pos , source ! (self , span) . lo ()) ; let trimmed_snippet = self . snippet (prev_span) . trim_end () ; let span_end = self . last_pos + BytePos (trimmed_snippet . len () as u32) ; self . format_missing (span_end) ; if self . buffer . ends_with ('\n') { self . buffer . pop () ; self . line_number -= 1 ; } self . last_pos = source ! (self , span) . hi () ; } Some (ref s) => { self . format_missing_with_indent (source ! (self , span) . lo ()) ; self . push_str (s) ; self . last_pos = source ! (self , span) . hi () ; } None => { self . format_missing_with_indent (source ! (self , span) . lo ()) ; self . format_missing (source ! (self , span) . hi ()) ; } } } }
};
}
