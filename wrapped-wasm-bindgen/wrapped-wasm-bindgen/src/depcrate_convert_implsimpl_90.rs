// Generated macro for impl_90 (impl)
macro_rules! Depcrate_convert_implsimpl_90 {
() => {
// Module: crate::convert::impls
// Provides: {"impl_90"}
// Dependencies: {}
impl < T > IntoWasmAbi for Option < * const T > { type Abi = f64 ; # [inline] fn into_abi (self) -> f64 { self . map (| ptr | ptr as u32 as f64) . unwrap_or (F64_ABI_OPTION_SENTINEL) } }
};
}
