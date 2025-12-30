// Generated macro for impl_add_assign (macro)
macro_rules! Depcrate_internal_macrosimpl_add_assign {
() => {
// Module: crate::internal_macros
// Provides: {"impl_add_assign"}
// Dependencies: {}
# [doc = " Implement `AddAssign` for the provided types."] macro_rules ! impl_add_assign { ($ target : ty : $ ($ (# [$ attr : meta]) * $ t : ty) ,+ $ (,) ?) => { $ crate :: internal_macros :: __impl_assign ! (+ AddAssign add_assign $ target : $ ($ (# [$ attr]) * $ t) ,+) ; } ; }
};
}
