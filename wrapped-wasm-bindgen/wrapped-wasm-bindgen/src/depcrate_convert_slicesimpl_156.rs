// Generated macro for impl_156 (impl)
macro_rules! Depcrate_convert_slicesimpl_156 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_156"}
// Dependencies: {}
impl RefFromWasmAbi for str { type Abi = < [u8] as RefFromWasmAbi > :: Abi ; type Anchor = Box < str > ; # [inline] unsafe fn ref_from_abi (js : Self :: Abi) -> Self :: Anchor { mem :: transmute :: < Box < [u8] > , Box < str > > (< Box < [u8] > > :: from_abi (js)) } }
};
}
