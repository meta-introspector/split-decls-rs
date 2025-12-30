// Generated macro for impl_159 (impl)
macro_rules! Depcrate_convert_slicesimpl_159 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_159"}
// Dependencies: {}
impl < T > OptionIntoWasmAbi for Box < [T] > where Self : IntoWasmAbi < Abi = WasmSlice > , { fn none () -> WasmSlice { null_slice () } }
};
}
