// Generated macro for has_only_region_constraints (function)
macro_rules! Depcrate_solvehas_only_region_constraints {
() => {
// Module: crate::solve
// Provides: {"has_only_region_constraints"}
// Dependencies: {}
fn has_only_region_constraints < I : Interner > (response : ty :: Canonical < I , Response < I > >) -> bool { let ExternalConstraintsData { region_constraints : _ , ref opaque_types , ref normalization_nested_goals , } = * response . value . external_constraints ; response . value . var_values . is_identity_modulo_regions () && opaque_types . is_empty () && normalization_nested_goals . is_empty () }
};
}
