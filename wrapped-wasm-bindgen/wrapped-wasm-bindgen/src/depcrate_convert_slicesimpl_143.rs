// Generated macro for impl_143 (impl)
macro_rules! Depcrate_convert_slicesimpl_143 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_143"}
// Dependencies: {}
impl VectorIntoWasmAbi for String { type Abi = < Box < [JsValue] > as IntoWasmAbi > :: Abi ; fn vector_into_abi (vector : Box < [Self] >) -> Self :: Abi { js_value_vector_into_abi (vector) } }
};
}
