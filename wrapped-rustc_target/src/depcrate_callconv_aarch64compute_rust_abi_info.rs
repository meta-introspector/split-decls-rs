// Generated macro for compute_rust_abi_info (function)
macro_rules! Depcrate_callconv_aarch64compute_rust_abi_info {
() => {
// Module: crate::callconv::aarch64
// Provides: {"compute_rust_abi_info"}
// Dependencies: {}
pub (crate) fn compute_rust_abi_info < 'a , Ty , C > (cx : & C , fn_abi : & mut FnAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout + HasTargetSpec , { for arg in fn_abi . args . iter_mut () . chain (iter :: once (& mut fn_abi . ret)) { softfloat_float_abi (cx . target_spec () , arg) ; } }
};
}
