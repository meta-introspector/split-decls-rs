// Generated macro for impl_666 (impl)
macro_rules! Depcrate_macimpl_666 {
() => {
// Module: crate::mac
// Provides: {"impl_666"}
// Dependencies: {}
impl MacroDelimiter { pub fn span (& self) -> & DelimSpan { match self { MacroDelimiter :: Paren (token) => & token . span , MacroDelimiter :: Brace (token) => & token . span , MacroDelimiter :: Bracket (token) => & token . span , } } # [cfg (all (feature = "full" , any (feature = "parsing" , feature = "printing")))] pub (crate) fn is_brace (& self) -> bool { match self { MacroDelimiter :: Brace (_) => true , MacroDelimiter :: Paren (_) | MacroDelimiter :: Bracket (_) => false , } } }
};
}
