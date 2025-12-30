// Generated macro for impl_224 (impl)
macro_rules! Depcrate_describeimpl_224 {
() => {
// Module: crate::describe
// Provides: {"impl_224"}
// Dependencies: {}
impl < T : WasmDescribe , E : Into < JsValue > > WasmDescribe for Result < T , E > { # [cfg_attr (wasm_bindgen_unstable_test_coverage , coverage (off))] fn describe () { inform (RESULT) ; T :: describe () ; } }
};
}
