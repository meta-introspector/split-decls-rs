// Generated macro for impl_264 (impl)
macro_rules! Depcrate_symbolimpl_264 {
() => {
// Module: crate::symbol
// Provides: {"impl_264"}
// Dependencies: {}
impl IdentPrinter { # [doc = " The most general `IdentPrinter` constructor. Do not use this."] pub fn new (symbol : Symbol , mode : IdentPrintMode , convert_dollar_crate : Option < Span > ,) -> IdentPrinter { IdentPrinter { symbol , mode , convert_dollar_crate } } # [doc = " This implementation is supposed to be used when printing identifiers"] # [doc = " as a part of pretty-printing for larger AST pieces."] # [doc = " Do not use this either."] pub fn for_ast_ident (ident : Ident , mode : IdentPrintMode) -> IdentPrinter { IdentPrinter :: new (ident . name , mode , Some (ident . span)) } }
};
}
