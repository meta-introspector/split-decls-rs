// Generated macro for classify_arg (function)
macro_rules! Depcrate_callconv_powerpcclassify_arg {
() => {
// Module: crate::callconv::powerpc
// Provides: {"classify_arg"}
// Dependencies: {}
fn classify_arg < Ty > (cx : & impl HasTargetSpec , arg : & mut ArgAbi < '_ , Ty >) { if arg . is_ignore () { if cx . target_spec () . os == "linux" && matches ! (&* cx . target_spec () . env , "gnu" | "musl" | "uclibc") && arg . layout . is_zst () { arg . make_indirect_from_ignore () ; } return ; } if arg . layout . is_aggregate () { arg . make_indirect () ; } else { arg . extend_integer_width_to (32) ; } }
};
}
