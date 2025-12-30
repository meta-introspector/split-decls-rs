// Generated macro for surround_macro_delimiter (function)
macro_rules! Depcrate_helperssurround_macro_delimiter {
() => {
// Module: crate::helpers
// Provides: {"surround_macro_delimiter"}
// Dependencies: {}
pub fn surround_macro_delimiter < F > (this : & MacroDelimiter , tokens : & mut TokenStream , f : F) where F : FnOnce (& mut TokenStream) , { match this { MacroDelimiter :: Paren (p) => p . surround (tokens , f) , MacroDelimiter :: Bracket (b) => b . surround (tokens , f) , MacroDelimiter :: Brace (b) => b . surround (tokens , f) , } }
};
}
