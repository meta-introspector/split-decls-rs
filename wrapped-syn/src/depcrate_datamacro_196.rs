// Generated macro for macro_196 (macro)
macro_rules! Depcrate_datamacro_196 {
() => {
// Module: crate::data
// Provides: {"macro_196"}
// Dependencies: {}
ast_struct ! { # [doc = " Named fields of a struct or struct variant such as `Point { x: f64,"] # [doc = " y: f64 }`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct FieldsNamed { pub brace_token : token :: Brace , pub named : Punctuated < Field , Token ! [,] >, } }
};
}
