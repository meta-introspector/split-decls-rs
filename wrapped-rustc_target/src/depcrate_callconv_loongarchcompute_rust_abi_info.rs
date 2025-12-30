// Generated macro for compute_rust_abi_info (function)
macro_rules! Depcrate_callconv_loongarchcompute_rust_abi_info {
() => {
// Module: crate::callconv::loongarch
// Provides: {"compute_rust_abi_info"}
// Dependencies: {}
pub (crate) fn compute_rust_abi_info < 'a , Ty , C > (cx : & C , fn_abi : & mut FnAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout + HasTargetSpec , { let grlen = cx . data_layout () . pointer_size () . bits () ; for arg in fn_abi . args . iter_mut () { if arg . is_ignore () { continue ; } extend_integer_width (arg , grlen) ; } }
};
}
