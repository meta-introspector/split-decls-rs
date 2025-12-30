// Generated macro for classify_ret (function)
macro_rules! Depcrate_callconv_cskyclassify_ret {
() => {
// Module: crate::callconv::csky
// Provides: {"classify_ret"}
// Dependencies: {}
fn classify_ret < Ty > (arg : & mut ArgAbi < '_ , Ty >) { if ! arg . layout . is_sized () { return ; } if arg . layout . is_aggregate () { let total = arg . layout . size ; if total . bits () > 64 { arg . make_indirect () ; } else if total . bits () > 32 { arg . cast_to (Uniform :: new (Reg :: i32 () , total)) ; } else { arg . cast_to (Reg :: i32 ()) ; } } else { arg . extend_integer_width_to (32) ; } }
};
}
