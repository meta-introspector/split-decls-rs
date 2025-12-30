// Generated macro for impl_143 (impl)
macro_rules! Depcrate_tidyimpl_143 {
() => {
// Module: crate::tidy
// Provides: {"impl_143"}
// Dependencies: {}
impl TidyMarks { fn visit (& mut self , _path : & Path , text : & str) { find_marks (& mut self . hits , text , "hit") ; find_marks (& mut self . checks , text , "check") ; find_marks (& mut self . checks , text , "check_count") ; } fn finish (self) { assert ! (! self . hits . is_empty ()) ; let diff : Vec < _ > = self . hits . symmetric_difference (& self . checks) . map (| it | it . as_str ()) . collect () ; if ! diff . is_empty () { panic ! ("unpaired marks: {diff:?}") } } }
};
}
