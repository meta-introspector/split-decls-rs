// Generated macro for classify_arg (function)
macro_rules! Depcrate_callconv_m68kclassify_arg {
() => {
// Module: crate::callconv::m68k
// Provides: {"classify_arg"}
// Dependencies: {}
fn classify_arg < Ty > (arg : & mut ArgAbi < '_ , Ty >) { if ! arg . layout . is_sized () { return ; } if arg . layout . is_aggregate () { arg . pass_by_stack_offset (None) ; } else { arg . extend_integer_width_to (32) ; } }
};
}
