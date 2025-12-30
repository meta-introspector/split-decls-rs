// Generated macro for classify_ret (function)
macro_rules! Depcrate_callconv_armclassify_ret {
() => {
// Module: crate::callconv::arm
// Provides: {"classify_ret"}
// Dependencies: {}
fn classify_ret < 'a , Ty , C > (cx : & C , ret : & mut ArgAbi < 'a , Ty > , vfp : bool) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout , { if ! ret . layout . is_sized () { return ; } if ! ret . layout . is_aggregate () { ret . extend_integer_width_to (32) ; return ; } if vfp { if let Some (uniform) = is_homogeneous_aggregate (cx , ret) { ret . cast_to (uniform) ; return ; } } let size = ret . layout . size ; let bits = size . bits () ; if bits <= 32 { ret . cast_to (Uniform :: new (Reg :: i32 () , size)) ; return ; } ret . make_indirect () ; }
};
}
