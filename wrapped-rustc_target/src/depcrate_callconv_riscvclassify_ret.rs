// Generated macro for classify_ret (function)
macro_rules! Depcrate_callconv_riscvclassify_ret {
() => {
// Module: crate::callconv::riscv
// Provides: {"classify_ret"}
// Dependencies: {}
fn classify_ret < 'a , Ty , C > (cx : & C , arg : & mut ArgAbi < 'a , Ty > , xlen : u64 , flen : u64) -> bool where Ty : TyAbiInterface < 'a , C > + Copy , { if ! arg . layout . is_sized () { return false ; } if let Some (conv) = should_use_fp_conv (cx , & arg . layout , xlen , flen) { match conv { FloatConv :: Float (f) => { arg . cast_to (f) ; } FloatConv :: FloatPair { first_ty , second_ty_offset_from_start , second_ty } => { arg . cast_to (CastTarget :: offset_pair (first_ty , second_ty_offset_from_start , second_ty ,)) ; } FloatConv :: MixedPair { first_ty , second_ty_offset_from_start , second_ty } => { arg . cast_to (CastTarget :: offset_pair (first_ty , second_ty_offset_from_start , second_ty ,)) ; } } return false ; } let total = arg . layout . size ; if total . bits () > 2 * xlen { if is_riscv_aggregate (arg) { arg . make_indirect () ; } return true ; } let xlen_reg = match xlen { 32 => Reg :: i32 () , 64 => Reg :: i64 () , _ => unreachable ! ("Unsupported XLEN: {}" , xlen) , } ; if is_riscv_aggregate (arg) { if total . bits () <= xlen { arg . cast_to (xlen_reg) ; } else { arg . cast_to (Uniform :: new (xlen_reg , Size :: from_bits (xlen * 2))) ; } return false ; } extend_integer_width (arg , xlen) ; false }
};
}
