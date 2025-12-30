// Generated macro for compute_abi_info (function)
macro_rules! Depcrate_callconv_loongarchcompute_abi_info {
() => {
// Module: crate::callconv::loongarch
// Provides: {"compute_abi_info"}
// Dependencies: {}
pub (crate) fn compute_abi_info < 'a , Ty , C > (cx : & C , fn_abi : & mut FnAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout + HasTargetSpec , { let xlen = cx . data_layout () . pointer_size () . bits () ; let flen = match & cx . target_spec () . llvm_abiname [..] { "ilp32f" | "lp64f" => 32 , "ilp32d" | "lp64d" => 64 , _ => 0 , } ; let mut avail_gprs = 8 ; let mut avail_fprs = 8 ; if ! fn_abi . ret . is_ignore () && classify_ret (cx , & mut fn_abi . ret , xlen , flen) { avail_gprs -= 1 ; } for (i , arg) in fn_abi . args . iter_mut () . enumerate () { if arg . is_ignore () { continue ; } classify_arg (cx , arg , xlen , flen , i >= fn_abi . fixed_count as usize , & mut avail_gprs , & mut avail_fprs ,) ; } }
};
}
