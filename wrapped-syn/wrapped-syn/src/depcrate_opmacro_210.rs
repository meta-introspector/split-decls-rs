// Generated macro for macro_210 (macro)
macro_rules! Depcrate_opmacro_210 {
() => {
// Module: crate::op
// Provides: {"macro_210"}
// Dependencies: {}
ast_enum ! { # [cfg_attr (feature = "clone-impls" , derive (Copy))] pub enum UnOp { # [doc = " The `*` operator for dereferencing"] Deref (tokens :: Star) , # [doc = " The `!` operator for logical inversion"] Not (tokens :: Bang) , # [doc = " The `-` operator for negation"] Neg (tokens :: Sub) , } }
};
}
