// Generated macro for impl_sub_assign (macro)
macro_rules! Depcrate_internal_macrosimpl_sub_assign {
() => {
// Module: crate::internal_macros
// Provides: {"impl_sub_assign"}
// Dependencies: {}
# [doc = " Implement `SubAssign` for the provided types."] macro_rules ! impl_sub_assign { ($ target : ty : $ ($ (# [$ attr : meta]) * $ t : ty) ,+ $ (,) ?) => { $ crate :: internal_macros :: __impl_assign ! (- SubAssign sub_assign $ target : $ ($ (# [$ attr]) * $ t) ,+) ; } ; }
};
}
