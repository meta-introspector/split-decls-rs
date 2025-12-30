// Generated macro for impl_32 (impl)
macro_rules! Depcrate_closureimpl_32 {
() => {
// Module: crate::closure
// Provides: {"impl_32"}
// Dependencies: {}
impl < T > WasmDescribe for OwnedClosure < T > where T : WasmClosure + ? Sized , { # [cfg_attr (wasm_bindgen_unstable_test_coverage , coverage (off))] fn describe () { inform (CLOSURE) ; inform (destroy :: < T > as usize as u32) ; inform (T :: IS_MUT as u32) ; T :: describe () ; } }
};
}
