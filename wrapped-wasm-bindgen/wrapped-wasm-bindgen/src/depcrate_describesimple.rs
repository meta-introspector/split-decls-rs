// Generated macro for simple (macro)
macro_rules! Depcrate_describesimple {
() => {
// Module: crate::describe
// Provides: {"simple"}
// Dependencies: {}
macro_rules ! simple { ($ ($ t : ident => $ d : ident) *) => ($ (impl WasmDescribe for $ t { # [cfg_attr (wasm_bindgen_unstable_test_coverage , coverage (off))] fn describe () { inform ($ d) } }) *) }
};
}
