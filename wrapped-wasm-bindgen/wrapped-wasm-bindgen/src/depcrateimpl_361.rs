// Generated macro for impl_361 (impl)
macro_rules! Depcrateimpl_361 {
() => {
// Module: crate
// Provides: {"impl_361"}
// Dependencies: {}
impl < T : VectorIntoWasmAbi > From < Clamped < Box < [T] > > > for JsValue { fn from (vector : Clamped < Box < [T] > >) -> Self { wbg_cast (vector) } }
};
}
