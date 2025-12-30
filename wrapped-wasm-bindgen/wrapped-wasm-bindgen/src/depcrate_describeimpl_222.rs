// Generated macro for impl_222 (impl)
macro_rules! Depcrate_describeimpl_222 {
() => {
// Module: crate::describe
// Provides: {"impl_222"}
// Dependencies: {}
impl < T : WasmDescribe > WasmDescribe for Option < T > { # [cfg_attr (wasm_bindgen_unstable_test_coverage , coverage (off))] fn describe () { inform (OPTIONAL) ; T :: describe () ; } }
};
}
