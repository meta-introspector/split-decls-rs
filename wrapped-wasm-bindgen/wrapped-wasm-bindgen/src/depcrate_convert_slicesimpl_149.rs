// Generated macro for impl_149 (impl)
macro_rules! Depcrate_convert_slicesimpl_149 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_149"}
// Dependencies: {}
impl < T > OptionFromWasmAbi for Vec < T > where Box < [T] > : FromWasmAbi < Abi = WasmSlice > , { # [inline] fn is_none (abi : & WasmSlice) -> bool { abi . ptr == 0 } }
};
}
