// Generated macro for compute_abi_info (function)
macro_rules! Depcrate_callconv_x86_64compute_abi_info {
() => {
// Module: crate::callconv::x86_64
// Provides: {"compute_abi_info"}
// Dependencies: {}
pub (crate) fn compute_abi_info < 'a , Ty , C > (cx : & C , fn_abi : & mut FnAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout + HasTargetSpec , { let mut int_regs = MAX_INT_REGS ; let mut sse_regs = MAX_SSE_REGS ; let mut x86_64_arg_or_ret = | arg : & mut ArgAbi < 'a , Ty > , is_arg : bool | { if ! arg . layout . is_sized () { return ; } let mut cls_or_mem = classify_arg (cx , arg) ; if is_arg { if let Ok (cls) = cls_or_mem { let mut needed_int = 0 ; let mut needed_sse = 0 ; for c in cls { match c { Some (Class :: Int) => needed_int += 1 , Some (Class :: Sse) => needed_sse += 1 , _ => { } } } match (int_regs . checked_sub (needed_int) , sse_regs . checked_sub (needed_sse)) { (Some (left_int) , Some (left_sse)) => { int_regs = left_int ; sse_regs = left_sse ; } _ => { if arg . layout . is_aggregate () { cls_or_mem = Err (Memory) ; } } } } } match cls_or_mem { Err (Memory) => { if is_arg { arg . pass_by_stack_offset (None) ; } else { arg . make_indirect () ; assert_eq ! (int_regs , MAX_INT_REGS) ; int_regs -= 1 ; } } Ok (ref cls) => { if arg . layout . is_aggregate () { let size = arg . layout . size ; arg . cast_to (cast_target (cls , size)) ; } else if is_arg || cx . target_spec () . is_like_darwin { arg . extend_integer_width_to (32) ; } } } } ; if ! fn_abi . ret . is_ignore () { x86_64_arg_or_ret (& mut fn_abi . ret , false) ; } for arg in fn_abi . args . iter_mut () { if arg . is_ignore () { continue ; } x86_64_arg_or_ret (arg , true) ; } }
};
}
