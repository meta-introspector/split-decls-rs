// Generated macro for impl_162 (impl)
macro_rules! Depcrate_convert_slicesimpl_162 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_162"}
// Dependencies: {}
impl < T : JsCast + WasmDescribe > VectorFromWasmAbi for T { type Abi = WasmSlice ; # [inline] unsafe fn vector_from_abi (js : WasmSlice) -> Box < [Self] > { let ptr = < * mut T > :: from_abi (js . ptr) ; let len = js . len as usize ; Vec :: from_raw_parts (ptr , len , len) . into_boxed_slice () } }
};
}
