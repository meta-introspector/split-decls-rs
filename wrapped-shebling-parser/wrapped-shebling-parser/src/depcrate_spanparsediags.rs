// Generated macro for ParseDiags (struct)
macro_rules! Depcrate_spanParseDiags {
() => {
// Module: crate::span
// Provides: {"ParseDiags"}
// Dependencies: {}
# [doc = " Diagnostics reported during parsing. These should serve as possible hints"] # [doc = " of what went wrong in case of failure."] # [doc = " It might also contain minor lints that refer to code not included in the AST (e.g."] # [doc = " trivia)."] # [derive (Debug)] pub (crate) struct ParseDiags (RefCell < Vec < Diagnostic > >) ;
};
}
