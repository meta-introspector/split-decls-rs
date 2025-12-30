// Generated macro for macro_195 (macro)
macro_rules! Depcrate_datamacro_195 {
() => {
// Module: crate::data
// Provides: {"macro_195"}
// Dependencies: {}
ast_struct ! { # [doc = " Named fields of a struct or struct variant such as `Point { x: f64,"] # [doc = " y: f64 }`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct FieldsNamed { pub brace_token : token :: Brace , pub named : Punctuated < Field , Token ! [,] >, } }
};
}
