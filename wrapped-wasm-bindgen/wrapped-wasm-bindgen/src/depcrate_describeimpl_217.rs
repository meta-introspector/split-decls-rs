// Generated macro for impl_217 (impl)
macro_rules! Depcrate_describeimpl_217 {
() => {
// Module: crate::describe
// Provides: {"impl_217"}
// Dependencies: {}
impl < T : WasmDescribe + ? Sized > WasmDescribe for & mut T { # [cfg_attr (wasm_bindgen_unstable_test_coverage , coverage (off))] fn describe () { inform (REFMUT) ; T :: describe () ; } }
};
}
