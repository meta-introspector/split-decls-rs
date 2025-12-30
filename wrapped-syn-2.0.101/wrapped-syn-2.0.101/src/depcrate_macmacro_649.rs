// Generated macro for macro_649 (macro)
macro_rules! Depcrate_macmacro_649 {
() => {
// Module: crate::mac
// Provides: {"macro_649"}
// Dependencies: {}
ast_enum ! { # [doc = " A grouping token that surrounds a macro body: `m!(...)` or `m!{...}` or `m![...]`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub enum MacroDelimiter { Paren (Paren) , Brace (Brace) , Bracket (Bracket) , } }
};
}
