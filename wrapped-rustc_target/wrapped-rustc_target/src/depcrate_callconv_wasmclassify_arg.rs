// Generated macro for classify_arg (function)
macro_rules! Depcrate_callconv_wasmclassify_arg {
() => {
// Module: crate::callconv::wasm
// Provides: {"classify_arg"}
// Dependencies: {}
fn classify_arg < 'a , Ty , C > (cx : & C , arg : & mut ArgAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout , { if ! arg . layout . is_sized () { return ; } arg . extend_integer_width_to (32) ; if arg . layout . is_aggregate () && ! unwrap_trivial_aggregate (cx , arg) { arg . make_indirect () ; } }
};
}
