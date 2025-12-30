// Generated macro for impl_148 (impl)
macro_rules! Depcrate_convert_slicesimpl_148 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_148"}
// Dependencies: {}
impl < T > FromWasmAbi for Vec < T > where Box < [T] > : FromWasmAbi < Abi = WasmSlice > , { type Abi = < Box < [T] > as FromWasmAbi > :: Abi ; # [inline] unsafe fn from_abi (js : Self :: Abi) -> Self { < Box < [T] > > :: from_abi (js) . into () } }
};
}
