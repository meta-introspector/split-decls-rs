// Generated macro for impl_362 (impl)
macro_rules! Depcrateimpl_362 {
() => {
// Module: crate
// Provides: {"impl_362"}
// Dependencies: {}
impl < T : VectorIntoWasmAbi > From < Vec < T > > for JsValue { fn from (vector : Vec < T >) -> Self { JsValue :: from (vector . into_boxed_slice ()) } }
};
}
