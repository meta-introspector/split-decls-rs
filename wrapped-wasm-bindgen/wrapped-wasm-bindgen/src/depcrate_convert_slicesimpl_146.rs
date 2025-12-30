// Generated macro for impl_146 (impl)
macro_rules! Depcrate_convert_slicesimpl_146 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_146"}
// Dependencies: {}
impl < T > IntoWasmAbi for Vec < T > where Box < [T] > : IntoWasmAbi < Abi = WasmSlice > , { type Abi = < Box < [T] > as IntoWasmAbi > :: Abi ; # [inline] fn into_abi (self) -> Self :: Abi { self . into_boxed_slice () . into_abi () } }
};
}
