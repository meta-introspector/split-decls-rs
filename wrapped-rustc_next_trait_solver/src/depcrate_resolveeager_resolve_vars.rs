// Generated macro for eager_resolve_vars (function)
macro_rules! Depcrate_resolveeager_resolve_vars {
() => {
// Module: crate::resolve
// Provides: {"eager_resolve_vars"}
// Dependencies: {}
pub fn eager_resolve_vars < D : SolverDelegate , T : TypeFoldable < D :: Interner > > (delegate : & D , value : T ,) -> T { if value . has_infer () { let mut folder = EagerResolver :: new (delegate) ; value . fold_with (& mut folder) } else { value } }
};
}
