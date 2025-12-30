// Generated macro for macro_665 (macro)
macro_rules! Depcrate_macmacro_665 {
() => {
// Module: crate::mac
// Provides: {"macro_665"}
// Dependencies: {}
ast_enum ! { # [doc = " A grouping token that surrounds a macro body: `m!(...)` or `m!{...}` or `m![...]`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub enum MacroDelimiter { Paren (Paren) , Brace (Brace) , Bracket (Bracket) , } }
};
}
