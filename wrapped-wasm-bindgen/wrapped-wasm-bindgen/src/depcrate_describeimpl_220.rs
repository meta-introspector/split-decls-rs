// Generated macro for impl_220 (impl)
macro_rules! Depcrate_describeimpl_220 {
() => {
// Module: crate::describe
// Provides: {"impl_220"}
// Dependencies: {}
impl < T : WasmDescribeVector > WasmDescribe for Box < [T] > { # [cfg_attr (wasm_bindgen_unstable_test_coverage , coverage (off))] fn describe () { T :: describe_vector () ; } }
};
}
