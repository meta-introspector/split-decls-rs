// Generated macro for impl_91 (impl)
macro_rules! Depcrate_convert_implsimpl_91 {
() => {
// Module: crate::convert::impls
// Provides: {"impl_91"}
// Dependencies: {}
impl < T > FromWasmAbi for Option < * const T > { type Abi = f64 ; # [inline] unsafe fn from_abi (js : f64) -> Option < * const T > { if js == F64_ABI_OPTION_SENTINEL { None } else { Some (js as u32 as * const T) } } }
};
}
