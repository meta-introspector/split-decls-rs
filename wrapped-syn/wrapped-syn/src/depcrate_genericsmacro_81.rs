// Generated macro for macro_81 (macro)
macro_rules! Depcrate_genericsmacro_81 {
() => {
// Module: crate::generics
// Provides: {"macro_81"}
// Dependencies: {}
ast_enum ! { # [doc = " A modifier on a bound, currently this is only used for `?Sized`, where the"] # [doc = " modifier is `Maybe`. Negative bounds should also be handled here."] # [cfg_attr (feature = "clone-impls" , derive (Copy))] pub enum TraitBoundModifier { None , Maybe (tokens :: Question) , } }
};
}
