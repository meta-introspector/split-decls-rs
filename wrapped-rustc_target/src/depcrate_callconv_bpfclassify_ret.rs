// Generated macro for classify_ret (function)
macro_rules! Depcrate_callconv_bpfclassify_ret {
() => {
// Module: crate::callconv::bpf
// Provides: {"classify_ret"}
// Dependencies: {}
fn classify_ret < Ty > (ret : & mut ArgAbi < '_ , Ty >) { if ret . layout . is_aggregate () || ret . layout . size . bits () > 64 { ret . make_indirect () ; } else { ret . extend_integer_width_to (32) ; } }
};
}
