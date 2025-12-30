// Generated macro for impl_330 (impl)
macro_rules! Depcrate_binderimpl_330 {
() => {
// Module: crate::binder
// Provides: {"impl_330"}
// Dependencies: {}
impl < I : Interner , T > Binder < I , T > where T : TypeVisitable < I > , { # [doc = " Wraps `value` in a binder, asserting that `value` does not"] # [doc = " contain any bound vars that would be bound by the"] # [doc = " binder. This is commonly used to 'inject' a value T into a"] # [doc = " different binding level."] # [track_caller] pub fn dummy (value : T) -> Binder < I , T > { assert ! (! value . has_escaping_bound_vars () , "`{value:?}` has escaping bound vars, so it cannot be wrapped in a dummy binder.") ; Binder { value , bound_vars : Default :: default () } } pub fn bind_with_vars (value : T , bound_vars : I :: BoundVarKinds) -> Binder < I , T > { if cfg ! (debug_assertions) { let mut validator = ValidateBoundVars :: new (bound_vars) ; let _ = value . visit_with (& mut validator) ; } Binder { value , bound_vars } } }
};
}
