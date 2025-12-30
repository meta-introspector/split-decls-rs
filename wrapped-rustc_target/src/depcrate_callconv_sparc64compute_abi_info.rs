// Generated macro for compute_abi_info (function)
macro_rules! Depcrate_callconv_sparc64compute_abi_info {
() => {
// Module: crate::callconv::sparc64
// Provides: {"compute_abi_info"}
// Dependencies: {}
pub (crate) fn compute_abi_info < 'a , Ty , C > (cx : & C , fn_abi : & mut FnAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout + HasTargetSpec , { if ! fn_abi . ret . is_ignore () { classify_arg (cx , & mut fn_abi . ret , Size :: from_bytes (32)) ; } for arg in fn_abi . args . iter_mut () { if arg . is_ignore () { if cx . target_spec () . os == "linux" && matches ! (&* cx . target_spec () . env , "gnu" | "musl" | "uclibc") && arg . layout . is_zst () { arg . make_indirect_from_ignore () ; } return ; } classify_arg (cx , arg , Size :: from_bytes (16)) ; } }
};
}
