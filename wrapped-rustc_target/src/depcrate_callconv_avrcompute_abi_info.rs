// Generated macro for compute_abi_info (function)
macro_rules! Depcrate_callconv_avrcompute_abi_info {
() => {
// Module: crate::callconv::avr
// Provides: {"compute_abi_info"}
// Dependencies: {}
pub (crate) fn compute_abi_info < Ty > (fty : & mut FnAbi < '_ , Ty >) { if ! fty . ret . is_ignore () { classify_ret_ty (& mut fty . ret) ; } for arg in fty . args . iter_mut () { if arg . is_ignore () { continue ; } classify_arg_ty (arg) ; } }
};
}
