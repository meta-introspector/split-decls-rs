// Generated macro for impl_108 (impl)
macro_rules! Depcrate_convert_implsimpl_108 {
() => {
// Module: crate::convert::impls
// Provides: {"impl_108"}
// Dependencies: {}
impl < T : OptionFromWasmAbi > FromWasmAbi for Option < T > { type Abi = T :: Abi ; # [inline] unsafe fn from_abi (js : T :: Abi) -> Self { if T :: is_none (& js) { None } else { Some (T :: from_abi (js)) } } }
};
}
