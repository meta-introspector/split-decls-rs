// Generated macro for impl_158 (impl)
macro_rules! Depcrate_convert_slicesimpl_158 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_158"}
// Dependencies: {}
impl < T : VectorIntoWasmAbi > IntoWasmAbi for Box < [T] > { type Abi = < T as VectorIntoWasmAbi > :: Abi ; fn into_abi (self) -> Self :: Abi { T :: vector_into_abi (self) } }
};
}
