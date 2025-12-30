// Generated macro for impl_160 (impl)
macro_rules! Depcrate_convert_slicesimpl_160 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_160"}
// Dependencies: {}
impl < T : VectorFromWasmAbi > FromWasmAbi for Box < [T] > { type Abi = < T as VectorFromWasmAbi > :: Abi ; unsafe fn from_abi (js : Self :: Abi) -> Self { T :: vector_from_abi (js) } }
};
}
