// Generated macro for macro_220 (macro)
macro_rules! Depcrate_tymacro_220 {
() => {
// Module: crate::ty
// Provides: {"macro_220"}
// Dependencies: {}
ast_struct ! { # [doc = " A \"Path\" is essentially Rust's notion of a name."] # [doc = ""] # [doc = " It's represented as a sequence of identifiers,"] # [doc = " along with a bunch of supporting information."] # [doc = ""] # [doc = " E.g. `std::cmp::PartialEq`"] pub struct Path { # [doc = " A `::foo` path, is relative to the crate root rather than current"] # [doc = " module (like paths in an import)."] pub leading_colon : Option < tokens :: Colon2 >, # [doc = " The segments in the path: the things separated by `::`."] pub segments : Delimited < PathSegment , tokens :: Colon2 >, } }
};
}
