// Generated macro for compute_abi_info (function)
macro_rules! Depcrate_callconv_armcompute_abi_info {
() => {
// Module: crate::callconv::arm
// Provides: {"compute_abi_info"}
// Dependencies: {}
pub (crate) fn compute_abi_info < 'a , Ty , C > (cx : & C , fn_abi : & mut FnAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout + HasTargetSpec , { let vfp = cx . target_spec () . llvm_target . ends_with ("hf") && fn_abi . conv != CanonAbi :: Arm (ArmCall :: Aapcs) && ! fn_abi . c_variadic ; if ! fn_abi . ret . is_ignore () { classify_ret (cx , & mut fn_abi . ret , vfp) ; } for arg in fn_abi . args . iter_mut () { if arg . is_ignore () { continue ; } classify_arg (cx , arg , vfp) ; } }
};
}
