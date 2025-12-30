// Generated macro for Term (enum)
macro_rules! Depcrate_astTerm {
() => {
// Module: crate::ast
// Provides: {"Term"}
// Dependencies: {}
# [doc = " Either a \"leaf\" [Pipeline], or a [binary expression](BinExpr) of [Term]s."] # [derive (Debug , From , PartialEq)] pub (crate) enum Term { List (List) , Pipeline (Pipeline) , }
};
}
