// Generated macro for impl_239 (impl)
macro_rules! Depcrate_parser_astimpl_239 {
() => {
// Module: crate::parser::ast
// Provides: {"impl_239"}
// Dependencies: {}
impl Expr { # [doc = " Create a new basic Expr"] pub fn new (val : ExprVal) -> Expr { Expr { val , negated : false , filters : vec ! [] } } # [doc = " Create a new negated Expr"] pub fn new_negated (val : ExprVal) -> Expr { Expr { val , negated : true , filters : vec ! [] } } # [doc = " Create a new basic Expr with some filters"] pub fn with_filters (val : ExprVal , filters : Vec < FunctionCall >) -> Expr { Expr { val , filters , negated : false } } # [doc = " Check if the expr has a default filter as first filter"] pub fn has_default_filter (& self) -> bool { if self . filters . is_empty () { return false ; } self . filters [0] . name == "default" } # [doc = " Check if the last filter is `safe`"] pub fn is_marked_safe (& self) -> bool { if self . filters . is_empty () { return false ; } self . filters [self . filters . len () - 1] . name == "safe" } }
};
}
