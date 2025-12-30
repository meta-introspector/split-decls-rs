// Generated macro for impl_161 (impl)
macro_rules! Depcrate_convert_slicesimpl_161 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_161"}
// Dependencies: {}
impl < T > OptionFromWasmAbi for Box < [T] > where Self : FromWasmAbi < Abi = WasmSlice > , { fn is_none (slice : & WasmSlice) -> bool { slice . ptr == 0 } }
};
}
