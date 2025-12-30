// Generated macro for impl_95 (impl)
macro_rules! Depcrate_convert_implsimpl_95 {
() => {
// Module: crate::convert::impls
// Provides: {"impl_95"}
// Dependencies: {}
impl < T > FromWasmAbi for Option < * mut T > { type Abi = f64 ; # [inline] unsafe fn from_abi (js : f64) -> Option < * mut T > { if js == F64_ABI_OPTION_SENTINEL { None } else { Some (js as u32 as * mut T) } } }
};
}
