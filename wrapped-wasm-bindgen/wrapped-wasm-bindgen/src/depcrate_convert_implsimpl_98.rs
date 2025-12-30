// Generated macro for impl_98 (impl)
macro_rules! Depcrate_convert_implsimpl_98 {
() => {
// Module: crate::convert::impls
// Provides: {"impl_98"}
// Dependencies: {}
impl < T > FromWasmAbi for NonNull < T > { type Abi = u32 ; # [inline] unsafe fn from_abi (js : Self :: Abi) -> Self { NonNull :: new_unchecked (js as * mut T) } }
};
}
