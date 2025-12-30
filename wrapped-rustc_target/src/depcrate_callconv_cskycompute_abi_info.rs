// Generated macro for compute_abi_info (function)
macro_rules! Depcrate_callconv_cskycompute_abi_info {
() => {
// Module: crate::callconv::csky
// Provides: {"compute_abi_info"}
// Dependencies: {}
pub (crate) fn compute_abi_info < Ty > (fn_abi : & mut FnAbi < '_ , Ty >) { if ! fn_abi . ret . is_ignore () { classify_ret (& mut fn_abi . ret) ; } for arg in fn_abi . args . iter_mut () { if arg . is_ignore () { continue ; } classify_arg (arg) ; } }
};
}
