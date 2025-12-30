// Generated macro for classify_arg (function)
macro_rules! Depcrate_callconv_bpfclassify_arg {
() => {
// Module: crate::callconv::bpf
// Provides: {"classify_arg"}
// Dependencies: {}
fn classify_arg < Ty > (arg : & mut ArgAbi < '_ , Ty >) { if arg . layout . is_aggregate () || arg . layout . size . bits () > 64 { arg . make_indirect () ; } else { arg . extend_integer_width_to (32) ; } }
};
}
