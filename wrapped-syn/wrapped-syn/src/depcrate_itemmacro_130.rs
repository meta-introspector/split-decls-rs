// Generated macro for macro_130 (macro)
macro_rules! Depcrate_itemmacro_130 {
() => {
// Module: crate::item
// Provides: {"macro_130"}
// Dependencies: {}
ast_enum ! { # [cfg_attr (feature = "clone-impls" , derive (Copy))] pub enum ImplPolarity { # [doc = " `impl Trait for Type`"] Positive , # [doc = " `impl !Trait for Type`"] Negative (tokens :: Bang) , } }
};
}
