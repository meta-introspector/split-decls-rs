// Generated macro for macro_239 (macro)
macro_rules! Depcrate_tymacro_239 {
() => {
// Module: crate::ty
// Provides: {"macro_239"}
// Dependencies: {}
ast_enum ! { # [doc = " Names of arguments in the `BareFnArg` structure"] pub enum BareFnArgName { # [doc = " Argument with the provided name"] Named (Ident) , # [doc = " Argument matched with `_`"] Wild (tokens :: Underscore) , } }
};
}
