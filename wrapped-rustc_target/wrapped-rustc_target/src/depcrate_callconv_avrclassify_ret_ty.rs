// Generated macro for classify_ret_ty (function)
macro_rules! Depcrate_callconv_avrclassify_ret_ty {
() => {
// Module: crate::callconv::avr
// Provides: {"classify_ret_ty"}
// Dependencies: {}
fn classify_ret_ty < Ty > (ret : & mut ArgAbi < '_ , Ty >) { if ret . layout . is_aggregate () { ret . make_indirect () ; } }
};
}
