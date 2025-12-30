// Generated macro for macro_339 (macro)
macro_rules! Depcrate_exprmacro_339 {
() => {
// Module: crate::expr
// Provides: {"macro_339"}
// Dependencies: {}
ast_enum ! { # [doc = " A struct or tuple struct field accessed in a struct literal or field"] # [doc = " expression."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub enum Member { # [doc = " A named field like `self.x`."] Named (Ident) , # [doc = " An unnamed field like `self.0`."] Unnamed (Index) , } }
};
}
