// Generated macro for compute_abi_info (function)
macro_rules! Depcrate_callconv_s390xcompute_abi_info {
() => {
// Module: crate::callconv::s390x
// Provides: {"compute_abi_info"}
// Dependencies: {}
pub (crate) fn compute_abi_info < 'a , Ty , C > (cx : & C , fn_abi : & mut FnAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout + HasTargetSpec , { if ! fn_abi . ret . is_ignore () { classify_ret (& mut fn_abi . ret) ; } for arg in fn_abi . args . iter_mut () { classify_arg (cx , arg) ; } }
};
}
