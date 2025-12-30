// Generated macro for macro_51 (macro)
macro_rules! Depcrate_exprmacro_51 {
() => {
// Module: crate::expr
// Provides: {"macro_51"}
// Dependencies: {}
# [cfg (feature = "full")] ast_enum ! { # [doc = " A statement, usually ending in a semicolon."] pub enum Stmt { # [doc = " A local (let) binding."] Local (Box < Local >) , # [doc = " An item definition."] Item (Box < Item >) , # [doc = " Expr without trailing semicolon."] Expr (Box < Expr >) , # [doc = " Expression with trailing semicolon;"] Semi (Box < Expr >, tokens :: Semi) , # [doc = " Macro invocation."] Mac (Box < (Mac , MacStmtStyle , Vec < Attribute >) >) , } }
};
}
