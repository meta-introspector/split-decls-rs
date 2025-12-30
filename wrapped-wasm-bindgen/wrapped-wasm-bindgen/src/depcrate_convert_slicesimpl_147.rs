// Generated macro for impl_147 (impl)
macro_rules! Depcrate_convert_slicesimpl_147 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_147"}
// Dependencies: {}
impl < T > OptionIntoWasmAbi for Vec < T > where Box < [T] > : IntoWasmAbi < Abi = WasmSlice > , { # [inline] fn none () -> WasmSlice { null_slice () } }
};
}
