// Generated macro for impl_339 (impl)
macro_rules! Depcrate_binderimpl_339 {
() => {
// Module: crate::binder
// Provides: {"impl_339"}
// Dependencies: {}
impl < I : Interner > ValidateBoundVars < I > { pub fn new (bound_vars : I :: BoundVarKinds) -> Self { ValidateBoundVars { bound_vars , binder_index : ty :: INNERMOST , visited : SsoHashSet :: default () , } } }
};
}
