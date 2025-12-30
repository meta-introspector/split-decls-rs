// Generated macro for impl_values (macro)
macro_rules! Depcrate_fieldimpl_values {
() => {
// Module: crate::field
// Provides: {"impl_values"}
// Dependencies: {}
macro_rules ! impl_values { ($ ($ record : ident ($ ($ whatever : tt) +)) ,+) => { $ (impl_value ! { $ record ($ ($ whatever) +) }) + } }
};
}
