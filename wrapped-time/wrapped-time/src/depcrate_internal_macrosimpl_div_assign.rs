// Generated macro for impl_div_assign (macro)
macro_rules! Depcrate_internal_macrosimpl_div_assign {
() => {
// Module: crate::internal_macros
// Provides: {"impl_div_assign"}
// Dependencies: {}
# [doc = " Implement `DivAssign` for the provided types."] macro_rules ! impl_div_assign { ($ target : ty : $ ($ (# [$ attr : meta]) * $ t : ty) ,+ $ (,) ?) => { $ crate :: internal_macros :: __impl_assign ! (/ DivAssign div_assign $ target : $ ($ (# [$ attr]) * $ t) ,+) ; } ; }
};
}
