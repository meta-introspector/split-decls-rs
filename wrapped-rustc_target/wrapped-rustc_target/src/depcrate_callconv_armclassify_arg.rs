// Generated macro for classify_arg (function)
macro_rules! Depcrate_callconv_armclassify_arg {
() => {
// Module: crate::callconv::arm
// Provides: {"classify_arg"}
// Dependencies: {}
fn classify_arg < 'a , Ty , C > (cx : & C , arg : & mut ArgAbi < 'a , Ty > , vfp : bool) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout , { if ! arg . layout . is_sized () { return ; } if ! arg . layout . is_aggregate () { arg . extend_integer_width_to (32) ; return ; } if vfp { if let Some (uniform) = is_homogeneous_aggregate (cx , arg) { arg . cast_to (uniform) ; return ; } } let align = arg . layout . align . abi . bytes () ; let total = arg . layout . size ; arg . cast_to (Uniform :: consecutive (if align <= 4 { Reg :: i32 () } else { Reg :: i64 () } , total)) ; }
};
}
