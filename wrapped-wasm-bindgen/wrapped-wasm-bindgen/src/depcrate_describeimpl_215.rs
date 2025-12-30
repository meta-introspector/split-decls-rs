// Generated macro for impl_215 (impl)
macro_rules! Depcrate_describeimpl_215 {
() => {
// Module: crate::describe
// Provides: {"impl_215"}
// Dependencies: {}
impl < T : WasmDescribe > WasmDescribe for [T] { # [cfg_attr (wasm_bindgen_unstable_test_coverage , coverage (off))] fn describe () { inform (SLICE) ; T :: describe () ; } }
};
}
