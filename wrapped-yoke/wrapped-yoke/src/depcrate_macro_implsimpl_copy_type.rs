// Generated macro for impl_copy_type (macro)
macro_rules! Depcrate_macro_implsimpl_copy_type {
() => {
// Module: crate::macro_impls
// Provides: {"impl_copy_type"}
// Dependencies: {}
macro_rules ! impl_copy_type { ($ ty : ty) => { unsafe impl <'a > Yokeable <'a > for $ ty { type Output = Self ; copy_yoke_impl ! () ; } } ; }
};
}
