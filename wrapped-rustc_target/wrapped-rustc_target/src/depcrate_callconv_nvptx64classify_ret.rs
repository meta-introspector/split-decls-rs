// Generated macro for classify_ret (function)
macro_rules! Depcrate_callconv_nvptx64classify_ret {
() => {
// Module: crate::callconv::nvptx64
// Provides: {"classify_ret"}
// Dependencies: {}
fn classify_ret < Ty > (ret : & mut ArgAbi < '_ , Ty >) { if ret . layout . is_aggregate () && ret . layout . is_sized () { classify_aggregate (ret) } else if ret . layout . size . bits () < 32 && ret . layout . is_sized () { ret . extend_integer_width_to (32) ; } }
};
}
