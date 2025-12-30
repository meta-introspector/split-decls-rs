// Generated macro for impl_157 (impl)
macro_rules! Depcrate_convert_slicesimpl_157 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_157"}
// Dependencies: {}
impl LongRefFromWasmAbi for str { type Abi = < [u8] as RefFromWasmAbi > :: Abi ; type Anchor = Box < str > ; # [inline] unsafe fn long_ref_from_abi (js : Self :: Abi) -> Self :: Anchor { Self :: ref_from_abi (js) } }
};
}
