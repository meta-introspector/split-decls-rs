// Generated macro for classify_arg (function)
macro_rules! Depcrate_callconv_nvptx64classify_arg {
() => {
// Module: crate::callconv::nvptx64
// Provides: {"classify_arg"}
// Dependencies: {}
fn classify_arg < Ty > (arg : & mut ArgAbi < '_ , Ty >) { if arg . layout . is_aggregate () && arg . layout . is_sized () { classify_aggregate (arg) } else if arg . layout . size . bits () < 32 && arg . layout . is_sized () { arg . extend_integer_width_to (32) ; } }
};
}
