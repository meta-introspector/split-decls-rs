// Generated macro for check_parser (function)
macro_rules! Depcrate_fuzzcheck_parser {
() => {
// Module: crate::fuzz
// Provides: {"check_parser"}
// Dependencies: {}
pub fn check_parser (text : & str) { let file = SourceFile :: parse (text , Edition :: CURRENT) ; check_file_invariants (& file . tree ()) ; }
};
}
