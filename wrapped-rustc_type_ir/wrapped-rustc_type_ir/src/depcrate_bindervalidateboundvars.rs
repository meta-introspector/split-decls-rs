// Generated macro for ValidateBoundVars (struct)
macro_rules! Depcrate_binderValidateBoundVars {
() => {
// Module: crate::binder
// Provides: {"ValidateBoundVars"}
// Dependencies: {}
pub struct ValidateBoundVars < I : Interner > { bound_vars : I :: BoundVarKinds , binder_index : ty :: DebruijnIndex , visited : SsoHashSet < (ty :: DebruijnIndex , I :: Ty) > , }
};
}
