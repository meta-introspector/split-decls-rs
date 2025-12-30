// Generated macro for impl_94 (impl)
macro_rules! Depcrate_convert_implsimpl_94 {
() => {
// Module: crate::convert::impls
// Provides: {"impl_94"}
// Dependencies: {}
impl < T > IntoWasmAbi for Option < * mut T > { type Abi = f64 ; # [inline] fn into_abi (self) -> f64 { self . map (| ptr | ptr as u32 as f64) . unwrap_or (F64_ABI_OPTION_SENTINEL) } }
};
}
