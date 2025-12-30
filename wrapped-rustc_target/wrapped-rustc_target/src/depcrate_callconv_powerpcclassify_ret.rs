// Generated macro for classify_ret (function)
macro_rules! Depcrate_callconv_powerpcclassify_ret {
() => {
// Module: crate::callconv::powerpc
// Provides: {"classify_ret"}
// Dependencies: {}
fn classify_ret < Ty > (ret : & mut ArgAbi < '_ , Ty >) { if ret . layout . is_aggregate () { ret . make_indirect () ; } else { ret . extend_integer_width_to (32) ; } }
};
}
