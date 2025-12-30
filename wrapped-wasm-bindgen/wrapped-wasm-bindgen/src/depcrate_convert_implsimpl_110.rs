// Generated macro for impl_110 (impl)
macro_rules! Depcrate_convert_implsimpl_110 {
() => {
// Module: crate::convert::impls
// Provides: {"impl_110"}
// Dependencies: {}
impl < T : FromWasmAbi > FromWasmAbi for Clamped < T > { type Abi = T :: Abi ; # [inline] unsafe fn from_abi (js : T :: Abi) -> Self { Clamped (T :: from_abi (js)) } }
};
}
