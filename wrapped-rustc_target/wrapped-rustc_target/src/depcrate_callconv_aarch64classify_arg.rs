// Generated macro for classify_arg (function)
macro_rules! Depcrate_callconv_aarch64classify_arg {
() => {
// Module: crate::callconv::aarch64
// Provides: {"classify_arg"}
// Dependencies: {}
fn classify_arg < 'a , Ty , C > (cx : & C , arg : & mut ArgAbi < 'a , Ty > , kind : AbiKind) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout + HasTargetSpec , { if ! arg . layout . is_sized () { return ; } if ! arg . layout . is_aggregate () { if kind == AbiKind :: DarwinPCS { arg . extend_integer_width_to (32) ; } softfloat_float_abi (cx . target_spec () , arg) ; return ; } if let Some (uniform) = is_homogeneous_aggregate (cx , arg) { arg . cast_to (uniform) ; return ; } let size = arg . layout . size ; let align = if kind == AbiKind :: AAPCS { arg . layout . unadjusted_abi_align } else { arg . layout . align . abi } ; if size . bits () <= 128 { if align . bits () == 128 { arg . cast_to (Uniform :: new (Reg :: i128 () , size)) ; } else { arg . cast_to (Uniform :: new (Reg :: i64 () , size)) ; } return ; } arg . make_indirect () ; }
};
}
