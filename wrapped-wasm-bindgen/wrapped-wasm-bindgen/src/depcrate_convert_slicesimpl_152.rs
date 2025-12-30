// Generated macro for impl_152 (impl)
macro_rules! Depcrate_convert_slicesimpl_152 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_152"}
// Dependencies: {}
impl FromWasmAbi for String { type Abi = < Vec < u8 > as FromWasmAbi > :: Abi ; # [inline] unsafe fn from_abi (js : Self :: Abi) -> Self { String :: from_utf8_unchecked (< Vec < u8 > > :: from_abi (js)) } }
};
}
