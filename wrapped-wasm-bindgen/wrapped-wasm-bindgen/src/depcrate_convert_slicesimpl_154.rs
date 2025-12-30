// Generated macro for impl_154 (impl)
macro_rules! Depcrate_convert_slicesimpl_154 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_154"}
// Dependencies: {}
impl < 'a > IntoWasmAbi for & 'a str { type Abi = < & 'a [u8] as IntoWasmAbi > :: Abi ; # [inline] fn into_abi (self) -> Self :: Abi { unsafe_get_cached_str (self) . unwrap_or_else (| | self . as_bytes () . into_abi ()) } }
};
}
