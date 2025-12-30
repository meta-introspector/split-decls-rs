// Generated macro for impl_216 (impl)
macro_rules! Depcrate_describeimpl_216 {
() => {
// Module: crate::describe
// Provides: {"impl_216"}
// Dependencies: {}
impl < T : WasmDescribe + ? Sized > WasmDescribe for & T { # [cfg_attr (wasm_bindgen_unstable_test_coverage , coverage (off))] fn describe () { inform (REF) ; T :: describe () ; } }
};
}
