// Generated macro for State (struct)
macro_rules! Depcrate_solve_inspectState {
() => {
// Module: crate::solve::inspect
// Provides: {"State"}
// Dependencies: {}
# [doc = " Some `data` together with information about how they relate to the input"] # [doc = " of the canonical query."] # [doc = ""] # [doc = " This is only ever used as [CanonicalState]. Any type information in proof"] # [doc = " trees used mechanically has to be canonicalized as we otherwise leak"] # [doc = " inference variables from a nested `InferCtxt`."] # [derive_where (Clone , PartialEq , Hash , Debug ; I : Interner , T)] # [derive_where (Copy ; I : Interner , T : Copy)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic)] pub struct State < I : Interner , T > { pub var_values : CanonicalVarValues < I > , pub data : T , }
};
}
