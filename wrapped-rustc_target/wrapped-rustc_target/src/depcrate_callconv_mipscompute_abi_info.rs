// Generated macro for compute_abi_info (function)
macro_rules! Depcrate_callconv_mipscompute_abi_info {
() => {
// Module: crate::callconv::mips
// Provides: {"compute_abi_info"}
// Dependencies: {}
pub (crate) fn compute_abi_info < Ty , C > (cx : & C , fn_abi : & mut FnAbi < '_ , Ty >) where C : HasDataLayout , { let mut offset = Size :: ZERO ; if ! fn_abi . ret . is_ignore () { classify_ret (cx , & mut fn_abi . ret , & mut offset) ; } for arg in fn_abi . args . iter_mut () { if arg . is_ignore () { continue ; } classify_arg (cx , arg , & mut offset) ; } }
};
}
