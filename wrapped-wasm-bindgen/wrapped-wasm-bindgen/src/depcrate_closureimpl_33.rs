// Generated macro for impl_33 (impl)
macro_rules! Depcrate_closureimpl_33 {
() => {
// Module: crate::closure
// Provides: {"impl_33"}
// Dependencies: {}
impl < T > IntoWasmAbi for OwnedClosure < T > where T : WasmClosure + ? Sized , { type Abi = WasmSlice ; fn into_abi (self) -> WasmSlice { let (a , b) : (usize , usize) = unsafe { mem :: transmute_copy (& ManuallyDrop :: new (self)) } ; WasmSlice { ptr : a as u32 , len : b as u32 , } } }
};
}
