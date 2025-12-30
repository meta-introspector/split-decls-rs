// Generated macro for compute_abi_info (function)
macro_rules! Depcrate_callconv_mips64compute_abi_info {
() => {
// Module: crate::callconv::mips64
// Provides: {"compute_abi_info"}
// Dependencies: {}
pub (crate) fn compute_abi_info < 'a , Ty , C > (cx : & C , fn_abi : & mut FnAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout , { if ! fn_abi . ret . is_ignore () { classify_ret (cx , & mut fn_abi . ret) ; } for arg in fn_abi . args . iter_mut () { if arg . is_ignore () { continue ; } classify_arg (cx , arg) ; } }
};
}
