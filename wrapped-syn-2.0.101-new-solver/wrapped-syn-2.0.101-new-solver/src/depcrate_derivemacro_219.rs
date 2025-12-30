// Generated macro for macro_219 (macro)
macro_rules! Depcrate_derivemacro_219 {
() => {
// Module: crate::derive
// Provides: {"macro_219"}
// Dependencies: {}
ast_struct ! { # [doc = " An enum input to a `proc_macro_derive` macro."] # [cfg_attr (docsrs , doc (cfg (feature = "derive")))] pub struct DataEnum { pub enum_token : Token ! [enum] , pub brace_token : token :: Brace , pub variants : Punctuated < Variant , Token ! [,] >, } }
};
}
