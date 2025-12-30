// Generated macro for impl_partial_eq (macro)
macro_rules! Depcrate_stable_vecimpl_partial_eq {
() => {
// Module: crate::stable_vec
// Provides: {"impl_partial_eq"}
// Dependencies: {}
macro_rules ! impl_partial_eq { ([$ ($ vars : tt) *] $ lhs : ty , $ rhs : ty) => { impl < T , U , $ ($ vars) *> PartialEq <$ rhs > for $ lhs where T : PartialEq < U >, { # [inline] fn eq (& self , other : &$ rhs) -> bool { self [..] == other [..] } } } }
};
}
