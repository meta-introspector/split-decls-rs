// Generated macro for impl_114 (impl)
macro_rules! Depcrate_convert_implsimpl_114 {
() => {
// Module: crate::convert::impls
// Provides: {"impl_114"}
// Dependencies: {}
impl IntoWasmAbi for JsError { type Abi = < JsValue as IntoWasmAbi > :: Abi ; fn into_abi (self) -> Self :: Abi { self . value . into_abi () } }
};
}
