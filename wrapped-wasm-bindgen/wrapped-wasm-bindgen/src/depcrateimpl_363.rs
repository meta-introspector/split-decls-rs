// Generated macro for impl_363 (impl)
macro_rules! Depcrateimpl_363 {
() => {
// Module: crate
// Provides: {"impl_363"}
// Dependencies: {}
impl < T : VectorIntoWasmAbi > From < Clamped < Vec < T > > > for JsValue { fn from (vector : Clamped < Vec < T > >) -> Self { JsValue :: from (Clamped (vector . 0 . into_boxed_slice ())) } }
};
}
