// Generated macro for classify_arg_ty (function)
macro_rules! Depcrate_callconv_avrclassify_arg_ty {
() => {
// Module: crate::callconv::avr
// Provides: {"classify_arg_ty"}
// Dependencies: {}
fn classify_arg_ty < Ty > (arg : & mut ArgAbi < '_ , Ty >) { if arg . layout . is_aggregate () { arg . make_indirect () ; } }
};
}
