// Generated macro for impl_109 (impl)
macro_rules! Depcrate_convert_implsimpl_109 {
() => {
// Module: crate::convert::impls
// Provides: {"impl_109"}
// Dependencies: {}
impl < T : IntoWasmAbi > IntoWasmAbi for Clamped < T > { type Abi = T :: Abi ; # [inline] fn into_abi (self) -> Self :: Abi { self . 0 . into_abi () } }
};
}
