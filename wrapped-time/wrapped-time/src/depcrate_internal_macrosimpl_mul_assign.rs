// Generated macro for impl_mul_assign (macro)
macro_rules! Depcrate_internal_macrosimpl_mul_assign {
() => {
// Module: crate::internal_macros
// Provides: {"impl_mul_assign"}
// Dependencies: {}
# [doc = " Implement `MulAssign` for the provided types."] macro_rules ! impl_mul_assign { ($ target : ty : $ ($ (# [$ attr : meta]) * $ t : ty) ,+ $ (,) ?) => { $ crate :: internal_macros :: __impl_assign ! (* MulAssign mul_assign $ target : $ ($ (# [$ attr]) * $ t) ,+) ; } ; }
};
}
