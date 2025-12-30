// Generated macro for classify_ret (function)
macro_rules! Depcrate_callconv_s390xclassify_ret {
() => {
// Module: crate::callconv::s390x
// Provides: {"classify_ret"}
// Dependencies: {}
fn classify_ret < Ty > (ret : & mut ArgAbi < '_ , Ty >) { let size = ret . layout . size ; if size . bits () <= 128 && matches ! (ret . layout . backend_repr , BackendRepr :: SimdVector { .. }) { return ; } if ! ret . layout . is_aggregate () && size . bits () <= 64 { ret . extend_integer_width_to (64) ; } else { ret . make_indirect () ; } }
};
}
