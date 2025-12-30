// Generated macro for impl_150 (impl)
macro_rules! Depcrate_convert_slicesimpl_150 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_150"}
// Dependencies: {}
impl IntoWasmAbi for String { type Abi = < Vec < u8 > as IntoWasmAbi > :: Abi ; # [inline] fn into_abi (self) -> Self :: Abi { unsafe_get_cached_str (& self) . unwrap_or_else (| | self . into_bytes () . into_abi ()) } }
};
}
