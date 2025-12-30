// Generated macro for impl_144 (impl)
macro_rules! Depcrate_convert_slicesimpl_144 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_144"}
// Dependencies: {}
impl VectorFromWasmAbi for String { type Abi = < Box < [JsValue] > as FromWasmAbi > :: Abi ; unsafe fn vector_from_abi (js : Self :: Abi) -> Box < [Self] > { js_value_vector_from_abi (js) } }
};
}
