// Generated macro for In (struct)
macro_rules! Depcrate_parser_astIn {
() => {
// Module: crate::parser::ast
// Provides: {"In"}
// Dependencies: {}
# [doc = " Something that checks whether the left side is contained in the right side"] # [derive (Clone , Debug , PartialEq)] pub struct In { # [doc = " The needle, a string or a basic expression/literal"] pub lhs : Box < Expr > , # [doc = " The haystack, can be a string, an array or an ident only currently"] pub rhs : Box < Expr > , # [doc = " Is it using `not` as in `b` not in `...`?"] pub negated : bool , }
};
}
