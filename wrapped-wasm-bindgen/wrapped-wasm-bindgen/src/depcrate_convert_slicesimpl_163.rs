// Generated macro for impl_163 (impl)
macro_rules! Depcrate_convert_slicesimpl_163 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_163"}
// Dependencies: {}
impl < T : JsCast + WasmDescribe > VectorIntoWasmAbi for T { type Abi = WasmSlice ; # [inline] fn vector_into_abi (vector : Box < [T] >) -> WasmSlice { let ptr = vector . as_ptr () ; let len = vector . len () ; mem :: forget (vector) ; WasmSlice { ptr : ptr . into_abi () , len : len as u32 , } } }
};
}
