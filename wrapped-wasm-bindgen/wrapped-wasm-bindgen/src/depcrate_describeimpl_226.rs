// Generated macro for impl_226 (impl)
macro_rules! Depcrate_describeimpl_226 {
() => {
// Module: crate::describe
// Provides: {"impl_226"}
// Dependencies: {}
impl < T : WasmDescribe > WasmDescribe for Clamped < T > { # [cfg_attr (wasm_bindgen_unstable_test_coverage , coverage (off))] fn describe () { inform (CLAMPED) ; T :: describe () ; } }
};
}
