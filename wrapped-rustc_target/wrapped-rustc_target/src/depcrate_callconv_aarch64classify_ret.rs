// Generated macro for classify_ret (function)
macro_rules! Depcrate_callconv_aarch64classify_ret {
() => {
// Module: crate::callconv::aarch64
// Provides: {"classify_ret"}
// Dependencies: {}
fn classify_ret < 'a , Ty , C > (cx : & C , ret : & mut ArgAbi < 'a , Ty > , kind : AbiKind) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout + HasTargetSpec , { if ! ret . layout . is_sized () { return ; } if ! ret . layout . is_aggregate () { if kind == AbiKind :: DarwinPCS { ret . extend_integer_width_to (32) } softfloat_float_abi (cx . target_spec () , ret) ; return ; } if let Some (uniform) = is_homogeneous_aggregate (cx , ret) { ret . cast_to (uniform) ; return ; } let size = ret . layout . size ; let bits = size . bits () ; if bits <= 128 { ret . cast_to (Uniform :: new (Reg :: i64 () , size)) ; return ; } ret . make_indirect () ; }
};
}
