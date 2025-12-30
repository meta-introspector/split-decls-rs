// Generated macro for classify_arg (function)
macro_rules! Depcrate_callconv_cskyclassify_arg {
() => {
// Module: crate::callconv::csky
// Provides: {"classify_arg"}
// Dependencies: {}
fn classify_arg < Ty > (arg : & mut ArgAbi < '_ , Ty >) { if ! arg . layout . is_sized () { return ; } if arg . layout . is_aggregate () { let total = arg . layout . size ; if total . bits () > 32 { arg . cast_to (Uniform :: new (Reg :: i32 () , total)) ; } else { arg . cast_to (Reg :: i32 ()) ; } } else { arg . extend_integer_width_to (32) ; } }
};
}
