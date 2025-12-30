// Generated macro for compute_abi_info (function)
macro_rules! Depcrate_callconv_powerpccompute_abi_info {
() => {
// Module: crate::callconv::powerpc
// Provides: {"compute_abi_info"}
// Dependencies: {}
pub (crate) fn compute_abi_info < Ty > (cx : & impl HasTargetSpec , fn_abi : & mut FnAbi < '_ , Ty >) { if ! fn_abi . ret . is_ignore () { classify_ret (& mut fn_abi . ret) ; } for arg in fn_abi . args . iter_mut () { classify_arg (cx , arg) ; } }
};
}
