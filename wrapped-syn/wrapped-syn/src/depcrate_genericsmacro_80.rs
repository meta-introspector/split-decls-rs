// Generated macro for macro_80 (macro)
macro_rules! Depcrate_genericsmacro_80 {
() => {
// Module: crate::generics
// Provides: {"macro_80"}
// Dependencies: {}
ast_enum ! { # [doc = " The AST represents all type param bounds as types."] # [doc = " `typeck::collect::compute_bounds` matches these against"] # [doc = " the \"special\" built-in traits (see `middle::lang_items`) and"] # [doc = " detects Copy, Send and Sync."] pub enum TyParamBound { Trait (PolyTraitRef , TraitBoundModifier) , Region (Lifetime) , } }
};
}
